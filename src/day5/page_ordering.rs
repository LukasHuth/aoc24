use std::{collections::HashSet, str::FromStr};

use crate::utilities::{CollectIntoResult as _, MapParse as _};

pub struct PageOrdering {
    data: HashSet<(u32, u32)>,
}
impl PageOrdering {
    pub fn new(data: Vec<PageOrderingElement>) -> Self {
        Self { data: data.into_iter().map(PageOrderingElement::into).collect() }
    }
    pub fn should_be_before(&self, a: u32, b: u32) -> bool {
        !self.data.contains(&(b, a))
    }
}
pub struct PageOrderingElement {
    a: u32,
    b: u32,
}
impl FromStr for PageOrderingElement {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u32> = s.trim().splitn(2, '|').parse().collect_result()?;
        Ok(Self {
            a: numbers[0],
            b: numbers[1],
        })
    }
}
impl Into<(u32, u32)> for PageOrderingElement {
    #[inline]
    fn into(self) -> (u32, u32) {
        (self.a, self.b)
    }
}
impl PartialEq for PageOrderingElement {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

