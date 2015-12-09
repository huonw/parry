extern crate rayon;

use std::{cmp, iter, slice};

mod operator;
pub use operator::{Neg, Not,
                   Add, Sub, Mul, Div,
                   BitOr, BitAnd, BitXor};
mod raw;
pub use raw::{Zip, Map};

pub mod iterators;

const MIN_THRESHOLD: usize = 1000;
const MAX_COUNT: usize = 20;

fn join<F: Send + FnOnce(), G: Send + FnOnce()>(f: F, g: G) {
    rayon::join(f, g);
}

pub fn evaluate<E>(dst: &mut [E::Element], e: E)
    where E: Expression + Send, E::Element: Send
{
    let len = dst.len();

    eval_inner(dst, e, cmp::max(len / MAX_COUNT, MIN_THRESHOLD));
}

fn eval_inner<E>(dst: &mut [E::Element], e: E, threshold: usize)
    where E: Expression + Send, E::Element: Send
{
    let len = dst.len();
    assert!(e.len().compatible(Length::Finite(len)));

    if len > threshold {
        let (low, high) = dst.split_at_mut(len / 2);
        let (e_low, e_high) = e.split();
        join(|| eval_inner(low, e_low, threshold),
             || eval_inner(high, e_high, threshold));
    } else {
        for (o, i) in dst.iter_mut().zip(e.values()) {
            *o = i;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Length {
    Finite(usize),
    Infinite,
}

impl Length {
    fn compatible(self, other: Length) -> bool {
        match (self, other) {
            (Length::Finite(a), Length::Finite(b)) => a == b,
            (Length::Infinite, _) | (_, Length::Infinite) => true,
        }
    }
}

pub trait Expression {
    type Element;
    type Values: Iterator<Item = Self::Element>;

    fn len(&self) -> Length;

    fn values(self) -> Self::Values;

    fn split(self) -> (Self, Self);

    // TODO: this need to handle SIMD etc.
    fn zip<E2: Expression>(self, e: E2) -> Zip<Self, E2> where Self: Sized {
        raw::make_zip(self, e)
    }
    // TODO: this needs to handle SIMD etc.
    fn map<O, F: FnMut(Self::Element) -> O>(self, f: F) -> Map<Self, F> where Self: Sized {
        raw::make_map(self, f)
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
