use std::{collections::HashMap, hash::Hash, str::FromStr};

use crate::{
    build_run, build_test,
    utilities::{CollectIntoResult, DigitCount, IsEven, MapParse},
};

fn part1() -> u128 {
    let mut stones = load_stones();
    stones.blinkn(25);
    stones.count_stones()
}

fn part2() -> u128 {
    let mut stones = load_stones();
    stones.blinkn(75);
    stones.count_stones()
}

#[derive(Clone, Copy)]
struct Stone {
    value: u64,
}
impl PartialEq for Stone {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for Stone {}
impl Hash for Stone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.value);
    }
}
impl Stone {
    fn blink(&mut self) -> Option<Stone> {
        if self.value == 0 {
            self.value = 1;
            return None;
        }
        let digit_count = self.value.digit_count() as u32;
        if digit_count.is_even() {
            let ten_pow = 10u64.pow(digit_count / 2);
            let new_value = self.value % ten_pow;
            self.value /= ten_pow;
            return Some(Self { value: new_value });
        }
        self.value *= 2024;
        None
    }
}
impl FromStr for Stone {
    type Err = <u64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s.trim().parse()?,
        })
    }
}

struct StoneCollection {
    stones: Vec<(Stone, u64)>,
}
impl StoneCollection {
    // this function ignores to contain the order because it is not needed to solve the problem
    fn blink(&mut self) {
        for i in 0..self.stones.len() {
            let (stone, amount) = &mut self.stones[i];
            let amount = *amount;
            if let Some(stone) = stone.blink() {
                self.stones.push((stone, amount));
                continue;
            }
        }
        self.merge();
    }
    fn blinkn(&mut self, n: usize) {
        for _ in 0..n {
            self.blink();
        }
    }
    fn merge(&mut self) {
        let mut data = HashMap::new();
        for (stone, amount) in &self.stones {
            *data.entry(stone).or_insert(0) += amount;
        }
        self.stones = data.into_iter().map(|(k, v)| (*k, v)).collect();
    }
    fn count_stones(&self) -> u128 {
        self.stones.iter().map(|&(_, amount)| amount as u128).sum()
    }
}

const STR: &str = include_str!("input.txt");
fn load_stones() -> StoneCollection {
    StoneCollection {
        stones: STR
            .trim()
            .split_whitespace()
            .parse()
            .map(|stone_res| stone_res.map(|stone| (stone, 1)))
            .collect_result()
            .unwrap(),
    }
}

build_run!(11, part1, part2);
build_test!(part1: 189547, part2: 224577979481346);
