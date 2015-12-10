use std::{cmp, iter, slice};
use {Expression, Length};
use iterators::SwitchIter;

#[derive(Copy, Clone)]
pub struct Constant<T>(pub T);

impl<'a, T: 'a + Send + Clone> Expression for Constant<T> {
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

impl<'a, T: 'a + Sync + Send + Clone> Expression for Value<&'a [T]> {
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

impl<'a, T: 'a + Sync + Send + Clone> Expression for &'a [T] {
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

    fn len(&self) -> Length {
        let len_b = self.0.len();
        let len_t = self.1.len();
        let len_e = self.2.len();
        debug_assert!(len_b.compatible(len_t) && len_b.compatible(len_e) && len_t.compatible(len_e));
        cmp::min(cmp::min(len_b, len_t), len_e)
    }

    fn values(self) -> Self::Values {
        SwitchIter::new(self.0.values(), self.1.values(), self.2.values())
    }

    fn split(self) -> (Self, Self) {
        let (b1, b2) = self.0.split();
        let (t1, t2) = self.1.split();
        let (e1, e2) = self.2.split();

        (Switch(b1, t1, e1), Switch(b2, t2, e2))
    }
}
