use std::ops::{Add, AddAssign, Deref, DerefMut};

#[derive(Clone, Copy)]
pub struct NumWrapper<T>(T)
where
    T: std::num::ZeroablePrimitive;

impl<T> NumWrapper<T>
where
    T: std::num::ZeroablePrimitive,
{
    #[inline]
    pub fn new(value: T) -> Self {
        Self(value)
    }
}
impl<T> Deref for NumWrapper<T>
where
    T: std::num::ZeroablePrimitive,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for NumWrapper<T>
where
    T: std::num::ZeroablePrimitive,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Add for NumWrapper<T>
where
    T: std::num::ZeroablePrimitive + Add<Output = T>,
{
    type Output = NumWrapper<T>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        NumWrapper(self.0.add(rhs.0))
    }
}
impl<T> Add<T> for NumWrapper<T>
where
    T: std::num::ZeroablePrimitive + Add<Output = T>,
{
    type Output = NumWrapper<T>;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        NumWrapper(self.0.add(rhs))
    }
}
impl<T> AddAssign<T> for NumWrapper<T>
where
    T: std::num::ZeroablePrimitive + Add<Output = T> + AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.0.add_assign(rhs);
    }
}
impl<T> AddAssign<NumWrapper<T>> for NumWrapper<T>
where
    T: std::num::ZeroablePrimitive + Add<Output = T> + AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: NumWrapper<T>) {
        self.0.add_assign(rhs.0);
    }
}
