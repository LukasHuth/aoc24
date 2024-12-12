use std::collections::HashSet;

use crate::{build_run, build_test, utilities::IteratorTrimmedLines};

fn part1() -> usize {
    let data = load_data();
    let mut set = HashSet::new();
    for antenna_collection in data {
        for (a1, a2) in antenna_collection
            .iter()
            .flat_map(|a1| antenna_collection.iter().map(move |a2| (a1, a2)))
            .filter(|(a, b)| a != b)
        {
            let diff = a1 - a2;
            let p1 = a2 - diff;
            if p1.is_possible() {}
            let p2 = a1 + diff;
            if p2.is_possible() {
                set.insert(p2.coords());
            }
        }
    }
    set.len()
}

fn part2() -> usize {
    let data = load_data();
    let mut set = HashSet::new();
    for antenna_collection in data {
        for (a1, a2) in antenna_collection
            .iter()
            .flat_map(|a1| antenna_collection.iter().map(move |a2| (a1, a2)))
            .filter(|(a, b)| a != b)
        {
            if a1 == a2 {
                continue;
            }
            set.insert(a1.coords());
            set.insert(a2.coords());
            let diff = a1 - a2;
            let mut p1 = a2 - diff;
            while p1.is_possible() {
                set.insert(p1.coords());
                p1 = p1 - diff;
            }
            let mut p2 = a1 + diff;
            while p2.is_possible() {
                set.insert(p2.coords());
                p2 = p2 + diff;
            }
        }
    }
    set.len()
}

mod antenna;
use antenna::Antenna;

const STR: &str = include_str!("input.txt");
const STR_LINE_LEN: isize = STR.len().isqrt() as isize;

fn load_data() -> Vec<Vec<Antenna>> {
    let mut antennas: Vec<Vec<Antenna>> = Vec::new();
    let mut chars = [None; 127];
    let mut next = 0;
    for (i, line) in STR.trimmed_lines().enumerate() {
        for (j, char) in line.trim().chars().enumerate() {
            if char == '.' {
                continue;
            }
            let index = if let Some(index) = chars[char as usize] {
                index
            } else {
                chars[char as usize] = Some(next);
                next += 1;
                next - 1
            };
            let antenna = Antenna::new(i as isize, j as isize);
            if let Some(list) = antennas.get_mut(index) {
                list.push(antenna);
            } else {
                antennas.push(Vec::new());
                antennas.last_mut().unwrap().push(antenna);
            }
        }
    }
    antennas
}

build_run!(8, part1, part2);
build_test!(part1: 303, part2: 1045);
