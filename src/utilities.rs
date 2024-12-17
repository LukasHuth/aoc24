use std::{
    num::ZeroablePrimitive,
    ops::{Add, AddAssign, BitAnd, Deref, DivAssign, Index},
    str::FromStr,
};

pub mod datatypes;

pub trait CollectIntoResult<T, E, W>: Iterator
where
    W: FromIterator<T>,
{
    /// Collects an iterator of `Result` items into a single `Result` containing the collection.
    ///
    /// If all items are `Ok`, a `Result` containing the collection of unwrapped values is returned.
    /// If any item is `Err`, the first `Err` is returned.
    ///
    /// # Returns
    /// - `Ok(W<T>)`: If all items are `Ok(T)`, returns a collection of `T`.
    /// - `Err(E)`: If any item is `Err(E)`, returns the first encountered `Err`.
    fn collect_result(self) -> Result<W, E>;
}

impl<I, W, T, E> CollectIntoResult<T, E, W> for I
where
    I: Iterator<Item = Result<T, E>>,
    W: FromIterator<T>,
{
    #[inline]
    fn collect_result(self) -> Result<W, E> {
        self.collect::<Result<W, E>>()
    }
}

type MapParseIter<'a, T, T0> = std::iter::Map<T0, fn(&'a str) -> Result<T, <T as FromStr>::Err>>;

pub trait MapParse<'a, T>: Iterator<Item = &'a str>
where
    T: FromStr,
    Self: Iterator<Item = &'a str> + Sized,
{
    /// Parses an iterator of strings into the requested datatype.
    ///
    /// This method maps each string to a `Result<T, <T as FromStr>::Err>` using the `FromStr`
    /// trait.
    ///
    /// # Returns
    /// An iterator of `Resullt<T, <T as FromStr>::Err>`
    fn parse(self) -> MapParseIter<'a, T, Self>;
}

impl<'a, T, I> MapParse<'a, T> for I
where
    I: Iterator<Item = &'a str>,
    T: FromStr,
    Self: Iterator<Item = &'a str> + Sized,
{
    #[inline]
    fn parse(self) -> MapParseIter<'a, T, Self> {
        self.map(str::parse::<T>)
    }
}

pub trait IteratorWithout: Iterator
where
    Self: Sized + Clone,
{
    #[inline]
    fn without(self, i: usize) -> std::iter::Chain<std::iter::Take<Self>, std::iter::Skip<Self>> {
        self.clone().take(i).chain(self.skip(i + 1))
    }
}

impl<I> IteratorWithout for I where I: Iterator + Clone {}

pub trait IteratorTrimmedLines {
    fn trimmed_lines(&self) -> std::str::Lines;
}
impl IteratorTrimmedLines for &str {
    fn trimmed_lines(&self) -> std::str::Lines {
        self.trim().lines().into_iter()
    }
}
impl IteratorTrimmedLines for String {
    #[inline]
    fn trimmed_lines(&self) -> std::str::Lines {
        self.trim().lines().into_iter()
    }
}
pub trait StringCharVec {
    fn char_vec(&self) -> Vec<char>;
}
impl StringCharVec for &str {
    #[inline]
    fn char_vec(&self) -> Vec<char> {
        self.chars().collect::<Vec<_>>()
    }
}
impl StringCharVec for String {
    #[inline]
    fn char_vec(&self) -> Vec<char> {
        self.chars().collect::<Vec<_>>()
    }
}
pub trait IteratorToVec<T> {
    fn to_vec(self) -> Vec<T>;
}
impl<T, I> IteratorToVec<T> for I
where
    I: Iterator<Item = T>,
{
    #[inline]
    fn to_vec(self) -> Vec<T> {
        self.collect()
    }
}

pub trait IteratorCount<T>
where
    Self: Iterator<Item = T>,
{
    fn count_element(self, value: T) -> usize;
}

impl<T, I> IteratorCount<T> for I
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    #[inline]
    fn count_element(self, value: T) -> usize {
        Iterator::count(self.filter(|v| v == &value))
    }
}

pub trait IteratorHasNElementsOf<T>
where
    Self: Iterator<Item = T> + IteratorCount<T>,
{
    fn has_n_elements_of(self, n: usize, element: T) -> bool;
}

impl<T, I> IteratorHasNElementsOf<T> for I
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    #[inline]
    fn has_n_elements_of(self, n: usize, element: T) -> bool {
        self.count_element(element) == n
    }
}

pub trait ParseTrimmedLines<T>
where
    Self: IteratorTrimmedLines,
    T: FromStr,
{
    fn parse_trimmed_lines(self) -> Result<Vec<T>, <T as FromStr>::Err>;
}

impl<T, T0> ParseTrimmedLines<T> for T0
where
    T0: IteratorTrimmedLines,
    T: FromStr,
{
    #[inline]
    fn parse_trimmed_lines(self) -> Result<Vec<T>, <T as FromStr>::Err> {
        self.trimmed_lines().parse().collect_result()
    }
}

pub trait MoveElement {
    fn move_element(&mut self, from: usize, to: usize);
}

impl<T> MoveElement for Vec<T> {
    fn move_element(&mut self, from: usize, to: usize) {
        if from < self.len() && to < self.len() && from != to {
            if from < to {
                for i in from..to {
                    self.swap(i, i + 1);
                }
            } else {
                for i in (to + 1..from + 1).rev() {
                    self.swap(i, i - 1);
                }
            }
        }
    }
}

pub trait IsEven {
    fn is_even(&self) -> bool;
}

impl<T> IsEven for T
where
    T: std::num::ZeroablePrimitive + BitAnd<Output = T> + From<u8> + Copy + PartialEq,
{
    #[inline]
    fn is_even(&self) -> bool {
        *self & 1u8.into() == 0u8.into()
    }
}

pub trait IncrementAfter
where
    Self: Add + Sized,
{
    fn increment_after(&mut self) -> Self;
}

impl<T> IncrementAfter for T
where
    T: Sized + Add<Output = T> + From<u8> + Copy,
{
    fn increment_after(&mut self) -> Self {
        let temp = *self;
        *self = *self + 1u8.into();
        temp
    }
}

pub trait DigitCount {
    fn digit_count(&self) -> usize;
}

impl<T> DigitCount for T
where
    T: ZeroablePrimitive + Copy + From<u8> + PartialEq + DivAssign,
{
    fn digit_count(&self) -> usize {
        let mut count = 0;
        let mut data = *self;
        while data != 0u8.into() {
            count += 1;
            data /= 10u8.into();
        }
        count
    }
}

#[test]
fn test_collect_result() {
    let test_data = vec!["1", "2", "3"];
    let test_data: Vec<u32> = test_data
        .into_iter()
        .map(str::parse::<u32>)
        .collect_result()
        .unwrap();
    assert_eq!(test_data, vec![1, 2, 3]);
}
#[test]
fn test_map_parse() {
    let test_data = vec!["1", "2", "3"];
    let test_data: Vec<u32> = test_data
        .into_iter()
        .parse()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(test_data, vec![1, 2, 3]);
}
#[test]
fn test_map_parse_collect_result() {
    let test_data = vec!["1", "2", "3"];
    let test_data: Vec<u32> = test_data.into_iter().parse().collect_result().unwrap();
    assert_eq!(test_data, vec![1, 2, 3]);
}
