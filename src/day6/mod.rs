use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use crate::{build_run, build_test};

const SIZE: usize = 130;

fn part1() -> usize {
    let mut simulation = load_data();
    simulation.run();
    simulation.visited_area.len()
}

fn part2() -> usize {
    let simulation = load_data();
    let start_guard = simulation.guard;
    let mut first_simulation = simulation.clone();
    first_simulation.run();
    let mut simulation = simulation.into_lightweight();
    first_simulation
        .visited_area
        .into_iter()
        .filter(|&point| point != start_guard.pos)
        .filter_map(|point| {
            simulation.reset(start_guard);
            simulation.obstacles.insert(point);
            let result = if simulation.run() { Some(()) } else { None };
            simulation.obstacles.remove(&point);
            result
        })
        .count()
}
fn load_data() -> GuardSimulation {
    let str = include_str!("input.txt");
    let data = str
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.trim().chars().enumerate().map(move |(j, c)| (i, j, c)))
        .filter(|&(_, _, c)| c != '.')
        .collect::<Vec<_>>();
    let obstacles = data
        .iter()
        .filter(|&(_, _, c)| *c == '#')
        .map(|&(a, b, _)| (a, b))
        .collect();
    let guard_position = data
        .into_iter()
        .filter(|&(_, _, c)| c == '^')
        .map(|(a, b, _)| (a, b))
        .next()
        .unwrap();
    GuardSimulation {
        guard: Guard {
            pos: guard_position,
            direction: Direction::Up,
        },
        obstacles,
        out_of_area: false,
        visited_area: HashSet::new(),
        in_loop: false,
    }
}

build_run!(6, part1, part2);
build_test!(part1: 5269, part2: 1957);
type Position = (usize, usize);
#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn rotate90(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Guard {
    pos: Position,
    direction: Direction,
}
#[derive(Debug, Clone)]
struct GuardSimulation {
    guard: Guard,
    obstacles: HashSet<Position>,
    out_of_area: bool,
    visited_area: HashSet<Position>,
    in_loop: bool,
}
struct LightWeightGuardSimulation {
    guard: Guard,
    obstacles: HashSet<Position>,
    out_of_area: bool,
    in_loop: bool,
}
impl Display for GuardSimulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..SIZE {
            for y in 0..SIZE {
                let pos = (x, y);
                if self.guard.pos == pos && !self.out_of_area {
                    f.write_char(match self.guard.direction {
                        Direction::Up => '^',
                        Direction::Left => '<',
                        Direction::Right => '>',
                        Direction::Down => 'v',
                    })?;
                } else if self.obstacles.contains(&pos) {
                    f.write_char('#')?;
                } else if self.visited_area.contains(&pos) {
                    f.write_char('X')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
#[inline]
fn get_next_obstacle(guard: &Guard, obstacles: &HashSet<Position>) -> Option<Position> {
    match guard.direction {
        Direction::Left => obstacles
            .iter()
            .filter(|&(x, _)| *x == guard.pos.0)
            .filter(|&(_, y)| *y < guard.pos.1)
            .max_by_key(|&(_, y)| y)
            .map(|v| *v),
        Direction::Right => obstacles
            .iter()
            .filter(|&(x, _)| *x == guard.pos.0)
            .filter(|&(_, y)| *y > guard.pos.1)
            .min_by_key(|&(_, y)| y)
            .map(|v| *v),
        Direction::Up => obstacles
            .iter()
            .filter(|&(_, y)| *y == guard.pos.1)
            .filter(|&(x, _)| *x < guard.pos.0)
            .max_by_key(|&(x, _)| x)
            .map(|v| *v),
        Direction::Down => obstacles
            .iter()
            .filter(|&(_, y)| *y == guard.pos.1)
            .filter(|&(x, _)| *x > guard.pos.0)
            .min_by_key(|&(x, _)| x)
            .map(|v| *v),
    }
}
impl GuardSimulation {
    #[inline]
    fn run(&mut self) -> bool {
        let mut moves = HashSet::new();
        while !(self.out_of_area || self.in_loop) {
            self.step(&mut moves);
        }
        self.in_loop
    }
    #[inline]
    fn into_lightweight(&self) -> LightWeightGuardSimulation {
        LightWeightGuardSimulation {
            guard: self.guard,
            obstacles: self.obstacles.clone(),
            in_loop: false,
            out_of_area: false,
        }
    }
    #[inline]
    fn get_next_obstacle(&self) -> Option<Position> {
        get_next_obstacle(&self.guard, &self.obstacles)
    }
    #[inline]
    fn step(&mut self, moves: &mut HashSet<Guard>) {
        if moves.contains(&self.guard) {
            self.in_loop = true;
            return;
        }
        moves.insert(self.guard);
        if let Some(next_obstacle) = self.get_next_obstacle() {
            self.do_move(next_obstacle);
            self.guard.direction.rotate90();
        } else {
            self.finish_move();
            self.out_of_area = true;
        }
    }
    #[inline]
    fn finish_move(&mut self) {
        match self.guard.direction {
            Direction::Left => {
                let x = self.guard.pos.0;
                for y in 0..=self.guard.pos.1 {
                    self.visited_area.insert((x, y));
                }
            }
            Direction::Right => {
                let x = self.guard.pos.0;
                for y in self.guard.pos.1..SIZE {
                    self.visited_area.insert((x, y));
                }
            }
            Direction::Up => {
                let y = self.guard.pos.1;
                for x in 0..=self.guard.pos.0 {
                    self.visited_area.insert((x, y));
                }
            }
            Direction::Down => {
                let y = self.guard.pos.1;
                for x in self.guard.pos.0..SIZE {
                    self.visited_area.insert((x, y));
                }
            }
        }
    }
    #[inline]
    fn do_move(&mut self, next_obstacle: Position) {
        match self.guard.direction {
            Direction::Left => {
                let x = next_obstacle.0;
                for y in next_obstacle.1 + 1..=self.guard.pos.1 {
                    self.visited_area.insert((x, y));
                }
                self.guard.pos = (x, next_obstacle.1 + 1);
            }
            Direction::Right => {
                let x = next_obstacle.0;
                for y in self.guard.pos.1..next_obstacle.1 - 1 {
                    self.visited_area.insert((x, y));
                }
                self.guard.pos = (x, next_obstacle.1 - 1);
            }
            Direction::Up => {
                let y = next_obstacle.1;
                for x in next_obstacle.0 + 1..=self.guard.pos.0 {
                    self.visited_area.insert((x, y));
                }
                self.guard.pos = (next_obstacle.0 + 1, y);
            }
            Direction::Down => {
                let y = next_obstacle.1;
                for x in self.guard.pos.0..=next_obstacle.0 - 1 {
                    self.visited_area.insert((x, y));
                }
                self.guard.pos = (next_obstacle.0 - 1, y);
            }
        }
    }
}
impl LightWeightGuardSimulation {
    #[inline]
    fn run(&mut self) -> bool {
        let mut moves = HashSet::new();
        while !(self.out_of_area || self.in_loop) {
            self.step(&mut moves);
        }
        self.in_loop
    }
    #[inline]
    fn get_next_obstacle(&self) -> Option<Position> {
        get_next_obstacle(&self.guard, &self.obstacles)
    }
    #[inline]
    fn step(&mut self, moves: &mut HashSet<Guard>) {
        if moves.contains(&self.guard) {
            self.in_loop = true;
            return;
        }
        moves.insert(self.guard);
        if let Some(next_obstacle) = self.get_next_obstacle() {
            self.do_move(next_obstacle);
            self.guard.direction.rotate90();
        } else {
            self.out_of_area = true;
        }
    }
    #[inline]
    fn do_move(&mut self, next_obstacle: Position) {
        match self.guard.direction {
            Direction::Left => {
                let x = next_obstacle.0;
                self.guard.pos = (x, next_obstacle.1 + 1);
            }
            Direction::Right => {
                let x = next_obstacle.0;
                self.guard.pos = (x, next_obstacle.1 - 1);
            }
            Direction::Up => {
                let y = next_obstacle.1;
                self.guard.pos = (next_obstacle.0 + 1, y);
            }
            Direction::Down => {
                let y = next_obstacle.1;
                self.guard.pos = (next_obstacle.0 - 1, y);
            }
        }
    }

    fn reset(&mut self, start_guard: Guard) {
        self.guard = start_guard;
        self.in_loop = false;
        self.out_of_area = false;
    }
}
