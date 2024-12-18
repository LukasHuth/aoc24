use std::{
    array::TryFromSliceError,
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    num::ParseIntError,
    ops::Index,
    str::FromStr,
};

use crate::{
    build_run, build_test,
    utilities::{CollectIntoResult, IteratorToVec, IteratorTrimmedLines, StringCharVec},
};

fn part1() -> u32 {
    let regions = load_regions();
    regions.iter().map(Region::get_value).sum()
}

fn part2() -> u32 {
    let _regions = load_regions();
    _regions.iter().map(Region::get_discount_value).sum()
}

struct Region<const SIZE: usize> {
    area: u32,
    points: HashMap<Coord, [bool; 4]>,
}

type Coord = (isize, isize);
impl<const SIZE: usize> Region<SIZE> {
    fn load_region(coord: Coord, data: &Data<SIZE>, not_visited: &mut HashSet<Coord>) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(coord);
        let mut found = HashMap::new();
        while let Some(coord) = queue.pop_front() {
            if found.contains_key(&coord) {
                continue;
            }
            let neighbors = [
                Self::neighbor_up(coord, data),
                Self::neighbor_down(coord, data),
                Self::neighbor_left(coord, data),
                Self::neighbor_right(coord, data),
            ];
            found.insert(coord, neighbors);
            not_visited.remove(&coord);
            for point in data.get_sourounding(coord) {
                queue.push_back(point);
            }
        }
        let area = found.iter().count() as u32;
        Self {
            points: found,
            area,
        }
    }
    fn get_value(&self) -> u32 {
        self.area * grahams_scan::<SIZE>(&self.points)
    }
    #[allow(unused)]
    fn get_discount_value(&self) -> u32 {
        self.get_sides() * self.area as u32
    }
    fn get_sides(&self) -> u32 {
        let top_open = self
            .points
            .iter()
            .filter(|(_, [top, ..])| !*top)
            .map(|(&(x, y), _)| (y, ListElement { start: x, end: x }))
            .merge_ranges();
        let bottom_open = self
            .points
            .iter()
            .filter(|(_, [_, bottom, ..])| !*bottom)
            .map(|(&(x, y), _)| (y, ListElement { start: x, end: x }))
            .merge_ranges();
        let left_open = self
            .points
            .iter()
            .filter(|(_, [_, _, left, _])| !*left)
            .map(|(&(x, y), _)| (x, ListElement { start: y, end: y }))
            .merge_ranges();
        let right_open = self
            .points
            .iter()
            .filter(|(_, [_, _, _, right])| !*right)
            .map(|(&(x, y), _)| (x, ListElement { start: y, end: y }))
            .merge_ranges();
        (top_open.len() + bottom_open.len() + left_open.len() + right_open.len()) as u32
    }
    fn neighbor_left(coord: Coord, data: &Data<SIZE>) -> bool {
        data[coord] == data[(coord.0 - 1, coord.1)]
    }
    fn neighbor_right(coord: Coord, data: &Data<SIZE>) -> bool {
        data[coord] == data[(coord.0 + 1, coord.1)]
    }
    fn neighbor_up(coord: Coord, data: &Data<SIZE>) -> bool {
        data[coord] == data[(coord.0, coord.1 - 1)]
    }
    fn neighbor_down(coord: Coord, data: &Data<SIZE>) -> bool {
        data[coord] == data[(coord.0, coord.1 + 1)]
    }
}
trait MergeRanges<T> {
    fn merge_ranges(self) -> Vec<T>;
}
impl<T> MergeRanges<(isize, ListElement)> for T
where
    T: Iterator<Item = (isize, ListElement)>,
{
    fn merge_ranges(self) -> Vec<(isize, ListElement)> {
        let mut list = self.to_vec();
        list.sort_by(|a, b| match a.0.cmp(&b.0) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => a.1.start.cmp(&b.1.start),
        });
        let mut merged_ranges: Vec<(isize, ListElement)> = Vec::new();
        for (same, range) in list {
            if let Some((same_last, range_last)) = merged_ranges.last_mut() {
                if *same_last == same && range_last.end + 1 == range.start {
                    range_last.end = range.end;
                    continue;
                }
            }
            merged_ranges.push((same, range));
        }
        merged_ranges
    }
}
struct ListElement {
    start: isize,
    end: isize,
}
impl Display for ListElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}..{}", self.start, self.end))
    }
}
impl Debug for ListElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

fn grahams_scan<const SIZE: usize>(points: &HashMap<Coord, [bool; 4]>) -> u32 {
    points
        .values()
        .map(|&neighbor| neighbor.into_iter().filter(|&v| !v).count())
        .sum::<usize>() as u32
}

struct Data<const SIZE: usize> {
    data: [[char; SIZE]; SIZE],
}

impl<const SIZE: usize> Index<Coord> for Data<SIZE> {
    type Output = char;

    #[inline(always)]
    fn index(&self, index: Coord) -> &Self::Output {
        if index.0 < 0 || index.1 < 0 || index.0 >= SIZE as isize || index.1 >= SIZE as isize {
            return &'\0';
        }
        &self.data[index.1 as usize][index.0 as usize]
    }
}

impl<const SIZE: usize> Data<SIZE> {
    fn get_sourounding(&self, coord: Coord) -> Vec<Coord> {
        let mut result = Vec::with_capacity(4);
        let value = self[coord];
        if coord.0 > 0 && self[(coord.0 - 1, coord.1)] == value {
            result.push((coord.0 - 1, coord.1));
        }
        if coord.1 > 0 && self[(coord.0, coord.1 - 1)] == value {
            result.push((coord.0, coord.1 - 1));
        }
        if coord.0 < SIZE as isize - 1 && self[(coord.0 + 1, coord.1)] == value {
            result.push((coord.0 + 1, coord.1));
        }
        if coord.1 < SIZE as isize - 1 && self[(coord.0, coord.1 + 1)] == value {
            result.push((coord.0, coord.1 + 1));
        }
        result
    }
}

#[derive(Debug)]
#[allow(unused)]
enum DataError {
    ParseIntError(ParseIntError),
    TryFromSliceError(TryFromSliceError),
    Other(&'static str),
}
impl From<ParseIntError> for DataError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
impl From<Vec<char>> for DataError {
    fn from(_: Vec<char>) -> Self {
        Self::Other("Could not convert Vector<char> to the array slice")
    }
}
impl<const SIZE: usize> From<Vec<[char; SIZE]>> for DataError {
    fn from(_: Vec<[char; SIZE]>) -> Self {
        Self::Other("Could not convert Vector to the array slice")
    }
}
impl<const SIZE: usize> FromStr for Data<SIZE> {
    type Err = DataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<[char; SIZE]> = s
            .trimmed_lines()
            .map(|line| line.char_vec().try_into().map_err(Self::Err::from))
            .collect_result()?;
        let data: [[char; SIZE]; SIZE] = lines.try_into().map_err(Self::Err::from)?;
        Ok(Self { data })
    }
}

const STR: &str = include_str!("input.txt");
const STR_LINES: usize = STR.len().isqrt();
fn load_data() -> Data<STR_LINES> {
    STR.trim().parse().unwrap()
}
fn load_regions() -> Vec<Region<STR_LINES>> {
    let data = load_data();
    let mut not_visited: HashSet<Coord> = (0..STR_LINES as isize)
        .flat_map(|i| (0..STR_LINES as isize).map(move |j| (i, j)))
        .collect();
    let mut regions = Vec::new();
    while let Some(not_visited_point) = not_visited.iter().next() {
        let region = Region::load_region(*not_visited_point, &data, &mut not_visited);
        regions.push(region);
    }
    regions
}

build_run!(12, part1, part2);
build_test!(part1: 1304764,part2: 811148);
