use std::{
    fmt::{Debug, Display},
    ops::{Add, Sub},
};

#[derive(Clone, Copy)]
pub(super) struct Antenna {
    x: isize,
    y: isize,
}
impl Sub for &Antenna {
    type Output = (isize, isize);

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y)
    }
}
impl Add for &Antenna {
    type Output = (isize, isize);

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<(isize, isize)> for &Antenna {
    type Output = Antenna;

    #[inline]
    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Antenna::new(self.x + rhs.0, self.y + rhs.1)
    }
}
impl Sub<(isize, isize)> for &Antenna {
    type Output = Antenna;

    #[inline]
    fn sub(self, rhs: (isize, isize)) -> Self::Output {
        Antenna::new(self.x - rhs.0, self.y - rhs.1)
    }
}
impl Add<(isize, isize)> for Antenna {
    type Output = Antenna;

    #[inline]
    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Antenna::new(self.x + rhs.0, self.y + rhs.1)
    }
}
impl Sub<(isize, isize)> for Antenna {
    type Output = Antenna;

    #[inline]
    fn sub(self, rhs: (isize, isize)) -> Self::Output {
        Antenna::new(self.x - rhs.0, self.y - rhs.1)
    }
}
impl Debug for Antenna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}
impl Display for Antenna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("( {} {} )", self.x, self.y))
    }
}
impl PartialEq for Antenna {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Antenna {
    pub(super) fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub(super) fn coords(&self) -> (isize, isize) {
        (self.x, self.y)
    }
    pub(super) fn is_possible(&self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < super::STR_LINE_LEN && self.y < super::STR_LINE_LEN
    }
}
