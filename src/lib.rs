extern crate rayon;

use std::cmp;

mod operator;
pub use operator::{Neg, Not,
                   Add, Sub, Mul, Div,
                   BitOr, BitAnd, BitXor,
                   Eq, Ne, Lt, Le, Gt, Ge};
mod raw;
pub use raw::{Zip, Map};

pub mod iterators;

mod simple;
pub use simple::{E, Constant, Switch};

const MIN_THRESHOLD: usize = 1024;
const MAX_COUNT: usize = 32;

fn join<F: Send + FnOnce(), G: Send + FnOnce()>(f: F, g: G) {
    rayon::join(f, g);
}

pub fn evaluate<E>(dst: &mut [E::Element], e: E)
    where E: Expression
{
    let len = dst.len();

    eval_inner(dst, e, cmp::max(len / MAX_COUNT, MIN_THRESHOLD));
}

fn eval_inner<E>(dst: &mut [E::Element], e: E, threshold: usize)
    where E: Expression
{
    let len = dst.len();
    assert!(e.length().compatible(Length::Finite(len)));

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

pub trait Expression: Send {
    type Element: Send;
    type Values: Iterator<Item = Self::Element>;

    fn length(&self) -> Length;

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

    fn switch<T, E>(self, then: T, else_: E) -> Switch<Self, T, E>
        where Self: Sized + Expression<Element = bool>, T: Expression, E: Expression<Element = T::Element>
    {
        simple::make_switch(self, then, else_)
    }

    fn eq<E>(self, other: E) -> Eq<Self, E>
        where Self: Sized, E: Expression, Self::Element: PartialEq<E::Element>,
    {
        operator::make_eq(self, other)
    }

    fn ne<E>(self, other: E) -> Ne<Self, E>
        where Self: Sized, E: Expression, Self::Element: PartialEq<E::Element>,
    {
        operator::make_ne(self, other)
    }

    fn lt<E>(self, other: E) -> Lt<Self, E>
        where Self: Sized, E: Expression, Self::Element: PartialOrd<E::Element>,
    {
        operator::make_lt(self, other)
    }

    fn le<E>(self, other: E) -> Le<Self, E>
        where Self: Sized, E: Expression, Self::Element: PartialOrd<E::Element>,
    {
        operator::make_le(self, other)
    }

    fn gt<E>(self, other: E) -> Gt<Self, E>
        where Self: Sized, E: Expression, Self::Element: PartialOrd<E::Element>,
    {
        operator::make_gt(self, other)
    }

    fn ge<E>(self, other: E) -> Ge<Self, E>
        where Self: Sized, E: Expression, Self::Element: PartialOrd<E::Element>,
    {
        operator::make_ge(self, other)
    }
}
