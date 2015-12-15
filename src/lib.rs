extern crate rayon;
extern crate num;
extern crate simd;

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
pub use simple::{E, Constant, Rev};

mod switch;
pub use switch::{Switch, SwitchOn};

mod evaluation;
pub mod reduce;

pub mod generic_simd;

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

    type Simd128Element: Send + generic_simd::SimdVector<Element = Self::Element>;
    type Simd128Values: Iterator<Item = Self::Simd128Element> + DoubleEndedIterator;

    type Rev: Expression<Element = Self::Element, Simd128Element = Self::Simd128Element>;

    fn length(&self) -> Length;

    fn values(self) -> Self::Values;

    fn simd128_values(self) -> (Self::Values, Self::Simd128Values, Self::Values);

    fn split(self, round_up: bool) -> (Self, Self);

    fn rev(self) -> Self::Rev;

    fn split_at(self, n: usize) -> (Self, Self);

    fn write<'a, E>(self, out: E)
        where Self: Sized, Self::Element: 'a, E: Expression<Element = &'a mut Self::Element>
    {
        evaluation::evaluate(self, reduce::Write(out))
    }

    fn sum<T>(self) -> Self::Element
        where Self: Sized + Expression<Element = T>, T: num::Zero + ops::Add<T, Output = T>
    {
        evaluation::evaluate(self, reduce::Sum)
    }

    fn max(self) -> <reduce::Max as reduce::Reduce<Self::Element>>::Output
        where Self: Sized, reduce::Max: reduce::Reduce<Self::Element>
    {
        evaluation::evaluate(self, reduce::Max)
    }
    fn min(self) -> <reduce::Min as reduce::Reduce<Self::Element>>::Output
        where Self: Sized, reduce::Min: reduce::Reduce<Self::Element>
    {
        evaluation::evaluate(self, reduce::Min)
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
        where Self: Sized + Expression, Self::Element: SwitchOn<T::Element>, Self::Simd128Element: SwitchOn<T::Simd128Element>, T: Expression, E: Expression<Element = T::Element>
    {
        switch::make_switch(self, then, else_)
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
