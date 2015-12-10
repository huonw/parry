use std::{cmp, iter, slice};
use {Expression, Length};
use iterators::SwitchIter;

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
}

#[derive(Copy, Clone)]
pub struct Switch<B, T, E>(B, T, E);
pub fn make_switch<B, T, E>(b: B, t: T, e: E) -> Switch<B, T, E>
    where B: Expression<Element = bool>,
          T: Expression,
          E: Expression<Element = T::Element>,
{
    Switch(b, t, e)
}

impl<B, T, E> Expression for Switch<B, T, E>
    // this should be, like, "bool"-like or something
    where B: Expression<Element = bool>,
          T: Expression,
          E: Expression<Element = T::Element>,
{
    type Element = T::Element;
    type Values = SwitchIter<B::Values, T::Values, E::Values>;
    type Rev = Switch<B::Rev, T::Rev, E::Rev>;

    fn length(&self) -> Length {
        let len_b = self.0.length();
        let len_t = self.1.length();
        let len_e = self.2.length();
        debug_assert!(len_b.compatible(len_t) && len_b.compatible(len_e) && len_t.compatible(len_e));
        cmp::min(cmp::min(len_b, len_t), len_e)
    }

    fn values(self) -> Self::Values {
        SwitchIter::new(self.0.values(), self.1.values(), self.2.values())
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        let (b1, b2) = self.0.split(round_up);
        let (t1, t2) = self.1.split(round_up);
        let (e1, e2) = self.2.split(round_up);

        (Switch(b1, t1, e1), Switch(b2, t2, e2))
    }

    fn rev(self) -> Self::Rev {
        Switch(self.0.rev(), self.1.rev(), self.2.rev())
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
}
