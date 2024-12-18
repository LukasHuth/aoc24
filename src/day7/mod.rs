use std::str::FromStr;

use crate::{
    build_run, build_test,
    utilities::{CollectIntoResult, MapParse as _, ParseTrimmedLines as _},
};

fn part1() -> u128 {
    let data = load_equations();
    data.into_iter()
        .filter(Equation::is_possible_p1)
        .map(|e| e.result)
        .sum()
}

fn part2() -> u128 {
    let data = load_equations();
    data.into_iter()
        .filter(Equation::is_possible_p2)
        .map(|e| e.result)
        .sum()
}

struct Equation {
    pub result: u128,
    values: Vec<u128>,
}

impl Equation {
    #[inline]
    fn new(result: u128, values: Vec<u128>) -> Self {
        Self { result, values }
    }
    #[inline]
    fn is_possible_p1(&self) -> bool {
        self.test_is_possible_p1(self.values[0], 1)
    }
    #[inline]
    fn is_possible_p2(&self) -> bool {
        self.test_is_possible_p2(self.values[0], 1)
    }
    #[inline]
    fn test_is_possible_p1(&self, n: u128, i: usize) -> bool {
        if n > self.result {
            return false;
        }
        if i >= self.values.len() {
            return n == self.result;
        }
        self.test_is_possible_p1(n + self.values[i], i + 1)
            || self.test_is_possible_p1(n * self.values[i], i + 1)
    }
    #[inline]
    fn test_is_possible_p2(&self, n: u128, i: usize) -> bool {
        if n > self.result {
            return false;
        }
        if i >= self.values.len() {
            return n == self.result;
        }
        self.test_is_possible_p2(n + self.values[i], i + 1)
            || self.test_is_possible_p2(n * self.values[i], i + 1)
            || self.test_is_possible_p2(concat(n, self.values[i]), i + 1)
    }
}
#[inline]
fn concat(a: u128, b: u128) -> u128 {
    a * 10u128.pow(count_digits(b)) + b
}
#[inline]
fn count_digits(mut a: u128) -> u32 {
    let mut result = 0;
    while a != 0 {
        result += 1;
        a /= 10;
    }
    result
}
impl FromStr for Equation {
    type Err = <u128 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splited = s.trim().split(": ");
        let result: u128 = splited.next().unwrap().trim().parse()?;
        let values: Vec<u128> = splited
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .parse()
            .collect_result()?;
        Ok(Self::new(result, values))
    }
}

fn load_equations() -> Vec<Equation> {
    include_str!("input.txt").parse_trimmed_lines().unwrap()
}

build_run!(7, part1, part2);
build_test!(part1: 1430271835320, part2: 456565678667482);
#[test]
fn test() {
    let test_eq = Equation::new(21037, vec![9, 7, 18, 3]);
    assert!(!test_eq.is_possible_p1());
}
#[test]
fn test_cancat() {
    assert_eq!(concat(10, 1), 101)
}
