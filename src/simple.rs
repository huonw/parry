use std::{iter, slice};
use {Expression, Length};

#[derive(Copy, Clone)]
pub struct Constant<T>(pub T);

impl<'a, T: 'a + Send + Clone> Expression for Constant<T> {
    type Element = T;
    type Values = iter::Repeat<T>;
    type Rev = Self;

    fn length(&self) -> Length {
        Length::Infinite
    }

    fn values(self) -> Self::Values {
        iter::repeat(self.0)
    }

    fn split(self, _round_up: bool) -> (Self, Self) {
        (self.clone(), self)
    }

    fn rev(self) -> Self {
        self
    }

    fn split_at(self, _n: usize) -> (Self, Self) {
        (self.clone(), self)
    }
}

#[derive(Copy, Clone)]
pub struct E<T>(pub T);

impl<X: Expression> Expression for E<X> {
    type Element = X::Element;
    type Values = X::Values;
    type Rev = E<X::Rev>;

    fn length(&self) -> Length {
        self.0.length()
    }

    fn values(self) -> Self::Values {
        self.0.values()
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        let (lo, hi) = self.0.split(round_up);
        (E(lo), E(hi))
    }

    fn rev(self) -> Self::Rev {
        E(self.0.rev())
    }

    fn split_at(self, n: usize) -> (Self, Self) {
        let (lo, hi) = self.0.split_at(n);
        (E(lo), E(hi))
    }
}

impl<'a, T: 'a + Sync + Send + Clone> Expression for &'a [T] {
    type Element = T;
    type Values = iter::Cloned<slice::Iter<'a, T>>;
    type Rev = Rev<&'a [T]>;

    fn length(&self) -> Length {
        Length::Finite((*self).len())
    }

    fn values(self) -> Self::Values {
        self.iter().cloned()
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        self.split_at((self.len() + round_up as usize) / 2)
    }

    fn rev(self) -> Self::Rev {
        Rev(self)
    }

    fn split_at(self, n: usize) -> (Self, Self) {
        (*self).split_at(n)
    }
}

impl<'a, T: 'a + Send> Expression for &'a mut [T] {
    type Element = &'a mut T;
    type Values = slice::IterMut<'a, T>;
    type Rev = Rev<&'a mut [T]>;

    fn length(&self) -> Length {
        Length::Finite((**self).len())
    }

    fn values(self) -> Self::Values {
        self.iter_mut()
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        let len = (*self).len();
        self.split_at_mut((len + round_up as usize) / 2)
    }

    fn rev(self) -> Self::Rev {
        Rev(self)
    }

    fn split_at(self, n: usize) -> (Self, Self) {
        (*self).split_at_mut(n)
    }
}

#[derive(Copy, Clone)]
pub struct Rev<T>(T);

impl<T: Expression> Expression for Rev<T> {
    type Element = T::Element;
    type Values = iter::Rev<T::Values>;
    type Rev = T;

    fn length(&self) -> Length {
        self.0.length()
    }

    fn values(self) -> Self::Values {
        self.0.values().rev()
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        let (a, b) = self.0.split(!round_up);
        (Rev(b), Rev(a))
    }

    fn rev(self) -> T {
        self.0
    }

    fn split_at(self, n: usize) -> (Self, Self) {
        let len = match self.length() {
            Length::Finite(len) => len,
            Length::Infinite => n // whatever
        };
        let (a, b) = self.0.split_at(len - n);
        (Rev(b), Rev(a))
    }
}
