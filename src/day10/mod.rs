use std::{
    collections::{HashSet, VecDeque},
    num::ParseIntError,
    ops::Index,
    str::FromStr,
};

use crate::{
    build_run, build_test,
    utilities::{IteratorToVec, ParseTrimmedLines, StringCharVec},
};

fn part1() -> usize {
    let map = load_map();
    map.get_trailheads()
        .into_iter()
        .map(|point| map.get_score(point))
        .sum()
}

fn part2() -> usize {
    let map = load_map();
    map.get_trailheads()
        .into_iter()
        .map(|point| map.get_rating(point))
        .sum()
}

struct Map<const SIZE: usize> {
    data: [[u8; SIZE]; SIZE],
}
impl<const SIZE: usize> Map<SIZE> {
    fn get_trailheads(&self) -> Vec<(usize, usize)> {
        self.data
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .filter(|&(_, value)| value == 0)
                    .map(move |(j, _)| (i, j))
            })
            .to_vec()
    }
    fn get_score(&self, coord: (usize, usize)) -> usize {
        let mut to_search = VecDeque::new();
        to_search.push_back(coord);
        let mut found = HashSet::new();
        while let Some(point) = to_search.pop_front() {
            if self[point] == 9 {
                found.insert(point);
                continue;
            }
            for point in self.sourrounding_higher(point) {
                if let Some(point) = point {
                    to_search.push_back(point);
                }
            }
        }
        found.len()
    }
    fn get_rating(&self, coord: (usize, usize)) -> usize {
        let mut to_search = VecDeque::new();
        to_search.push_back(coord);
        let mut found = 0;
        while let Some(point) = to_search.pop_front() {
            if self[point] == 9 {
                found += 1;
                continue;
            }
            for point in self.sourrounding_higher(point) {
                if let Some(point) = point {
                    to_search.push_back(point);
                }
            }
        }
        found
    }
    fn sourrounding_higher(&self, coord: (usize, usize)) -> [Option<(usize, usize)>; 4] {
        let mut result = [None; 4];
        let value = self[coord];
        if coord.0 > 0 && self.data[coord.0 - 1][coord.1] == value + 1 {
            result[0] = Some((coord.0 - 1, coord.1));
        }
        if coord.1 > 0 && self.data[coord.0][coord.1 - 1] == value + 1 {
            result[1] = Some((coord.0, coord.1 - 1));
        }
        if coord.0 < SIZE - 1 && self.data[coord.0 + 1][coord.1] == value + 1 {
            result[2] = Some((coord.0 + 1, coord.1));
        }
        if coord.1 < SIZE - 1 && self.data[coord.0][coord.1 + 1] == value + 1 {
            result[3] = Some((coord.0, coord.1 + 1));
        }
        result
    }
}

impl<const SIZE: usize> Index<(usize, usize)> for Map<SIZE> {
    type Output = u8;

    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

const STR: &str = include_str!("input.txt");
const STR_LINES: usize = STR.len().isqrt();
struct Line<const SIZE: usize> {
    data: [u8; SIZE],
}
#[derive(Debug)]
#[allow(dead_code)]
enum LineError {
    ParseIntError(ParseIntError),
    Other(&'static str),
}
impl From<ParseIntError> for LineError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
impl<const SIZE: usize> FromStr for Line<SIZE> {
    type Err = LineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: [char; SIZE] = s
            .char_vec()
            .try_into()
            .map_err(|_| LineError::Other("Could not convert to the correct sized char array"))?;
        let data: [u8; SIZE] = chars
            .map(|c| c.to_digit(10))
            .transpose()
            .ok_or(LineError::Other("Could not Parse an Char"))?
            .map(|v| v as u8);
        Ok(Self { data })
    }
}
fn load_map() -> Map<STR_LINES> {
    let test: Vec<Line<STR_LINES>> = STR.parse_trimmed_lines().unwrap();
    let data: [[u8; STR_LINES]; STR_LINES] = test
        .into_iter()
        .map(|line| line.data)
        .to_vec()
        .try_into()
        .unwrap();
    Map { data }
}

build_run!(10, part1, part2);
build_test!(part1: 778, part2: 1925);
