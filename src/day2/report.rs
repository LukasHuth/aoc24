use std::{cmp::Ordering, str::FromStr};

use crate::utilities::{datatypes::NumberList, CollectIntoResult as _, MapParse as _, NewResult};

pub struct Report {
    numbers: NumberList<u32>,
}
fn calculate_diff_cmp(input: &[&u32; 2]) -> (u32, Ordering) {
    let &[&x, &y] = input;
    (x.abs_diff(y), x.cmp(&y))
}
use crate::utilities::IteratorWithout;
impl Report {
    fn new(numbers: NumberList<u32>) -> Self {
        Self { numbers }
    }
    pub fn is_safe_zero_tolerance(&self) -> bool {
        let mut windowed_numbers = self.numbers.iter().map_windows(calculate_diff_cmp);
        if let Some((first_abs_diff, first_ord)) = windowed_numbers.next() {
            if first_abs_diff > 3 || first_abs_diff == 0 {
                return false;
            }
            windowed_numbers.all(|(v, ord)| ord == first_ord && v <= 3 && v >= 1)
        } else {
            false
        }
    }
    pub fn is_safe_one_tolerance(&self) -> bool {
        if self.is_safe_zero_tolerance() {
            return true;
        }
        for i in (0..=self.numbers.len()).rev() {
            let mut numbers = self
                .numbers
                .iter()
                .without(i)
                .map_windows(calculate_diff_cmp);
            if let Some((first_diff, first_ord)) = numbers.next() {
                if first_diff > 3 || first_diff == 0 {
                    continue;
                }
                if numbers.all(|(diff, ord)| ord == first_ord && diff <= 3 && diff >= 1) {
                    return true;
                }
            }
        }
        false
    }
}
impl<E> NewResult<Vec<u32>, E> for Report {
    fn new_result(a: Result<Vec<u32>, E>) -> Result<Self, E> {
        match a {
            Ok(v) => Ok(Self::new(NumberList::new(v))),
            Err(e) => Err(e),
        }
    }
}
impl FromStr for Report {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: NumberList<u32> = s.split_whitespace().parse().collect_result()?;
        Ok(Self { numbers })
    }
}
