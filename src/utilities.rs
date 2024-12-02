use std::str::FromStr;

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
    fn parse(self) -> MapParseIter<'a, T, Self> {
        self.map(str::parse::<T>)
    }
}

pub trait IteratorWithout: Iterator
where
    Self: Sized + Clone,
{
    fn without(self, i: usize) -> std::iter::Chain<std::iter::Take<Self>, std::iter::Skip<Self>> {
        self.clone().take(i).chain(self.skip(i + 1))
    }
}

impl<I> IteratorWithout for I where I: Iterator + Clone {}

pub trait NewResult<T, E>
where
    Self: Sized,
{
    /// Creates a new instance of the implementing type from a `Result`.
    ///
    /// If the input is `Ok(T)`, it returns `Ok(Self)` with a new instance of the type.
    /// If the input is `Err(E)`, it returns the `Err` as-is.
    ///
    /// # Parameters
    /// - `a`: A `Result<T, E>` value.
    ///
    /// # Returns
    /// - `Ok(Self)`: If the input is `Ok(T)`, creates and returns a new instance of the type.
    /// - `Err(E)`: If the input is `Err(E)`, returns the error.
    fn new_result(a: Result<T, E>) -> Result<Self, E>;
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
