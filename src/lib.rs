extern crate rayon;
extern crate num;

use std::ops;

mod operator;
pub use operator::{Neg, Not,
                   Add, Sub, Mul, Div,
                   BitOr, BitAnd, BitXor,
                   Eq, Ne, Lt, Le, Gt, Ge};
mod raw;
pub use raw::{Zip, Map};

pub mod iterators;

mod simple;
pub use simple::{E, Constant, Switch, Rev};

mod evaluation;

pub fn evaluate<E>(dst: &mut [E::Element], e: E)
    where E: Expression
{
    evaluation::evaluate(e, evaluation::SetArray(dst))
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
    type Values: Iterator<Item = Self::Element> + DoubleEndedIterator;
    type Rev: Expression<Element = Self::Element>;

    fn length(&self) -> Length;

    fn values(self) -> Self::Values;

    fn split(self, round_up: bool) -> (Self, Self);

    fn rev(self) -> Self::Rev;

    fn sum<T>(self) -> Self::Element
        where Self: Sized + Expression<Element = T>, T: num::Zero + ops::Add<T, Output = T>
    {
        evaluation::evaluate(self, evaluation::Sum)
    }

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
