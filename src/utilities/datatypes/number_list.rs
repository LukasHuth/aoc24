use core::num;
use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::utilities::{CollectIntoResult as _, MapParse};

pub struct NumberList<T>
where
    T: num::ZeroablePrimitive,
{
    list: Vec<T>,
}
impl<T> NumberList<T>
where
    T: num::ZeroablePrimitive,
{
    pub fn new(list: Vec<T>) -> Self {
        Self { list }
    }
}
impl<T> Deref for NumberList<T>
where
    T: num::ZeroablePrimitive,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}
impl<T> DerefMut for NumberList<T>
where
    T: num::ZeroablePrimitive,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}
impl<T> FromStr for NumberList<T>
where
    T: num::ZeroablePrimitive + FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace().parse().collect_result().map(Self::new)
    }
}

impl<T> FromIterator<T> for NumberList<T>
where
    T: num::ZeroablePrimitive,
{
    fn from_iter<T0: IntoIterator<Item = T>>(iter: T0) -> Self {
        Self {
            list: iter.into_iter().collect(),
        }
    }
}
#[test]
fn test_number_list() {
    let test = NumberList::new(vec![1, 2, 3, 4]);
    assert_eq!(test[0], 1);
    let mut test = test;
    test[1] = 5;
    assert_eq!(test[1], 5);
}
