use std::{iter, slice};
use {Expression, Length};

#[derive(Copy, Clone)]
pub struct Constant<T>(pub T);

impl<'a, T: 'a + Clone> Expression for Constant<T> {
    type Element = T;
    type Values = iter::Repeat<T>;

    fn len(&self) -> Length {
        Length::Infinite
    }

    fn values(self) -> Self::Values {
        iter::repeat(self.0)
    }

    fn split(self) -> (Self, Self) {
        (self.clone(), self)
    }
}

#[derive(Copy, Clone)]
pub struct Value<T>(pub T);

impl<'a,T: 'a + Clone> Expression for Value<&'a [T]> {
    type Element = T;
    type Values = iter::Cloned<slice::Iter<'a, T>>;

    fn len(&self) -> Length {
        Length::Finite(self.0.len())
    }

    fn values(self) -> Self::Values {
        self.0.iter().cloned()
    }

    fn split(self) -> (Self, Self) {
        let (lo, hi) = self.0.split_at(self.0.len() / 2);
        (Value(lo), Value(hi))
    }
}

impl<'a, T: 'a + Clone> Expression for &'a [T] {
    type Element = T;
    type Values = iter::Cloned<slice::Iter<'a, T>>;

    fn len(&self) -> Length {
        Length::Finite((*self).len())
    }

    fn values(self) -> Self::Values {
        self.iter().cloned()
    }

    fn split(self) -> (Self, Self) {
        self.split_at(self.len() / 2)
    }
}
