use {Expression, Value};
use iterators::{Binary, Plus, Times};
use std::ops;

#[derive(Copy, Clone)]
pub struct Add<X, Y>(X, Y);
impl<X, Y> Expression for Add<X, Y>
    where X: Expression,
          Y: Expression,
          X::Element: ops::Add<Y::Element> + Clone,
          Y::Element: Clone
{
    type Element = <X::Element as ops::Add<Y::Element>>::Output;
    type Values = Binary<Plus, X::Values, Y::Values>;

    fn len(&self) -> usize {
        let len = self.0.len();
        debug_assert_eq!(len, self.1.len());
        len
    }

    fn values(self) -> Self::Values {
        Binary::new(Plus, self.0.values(), self.1.values())
    }

    fn split(self) -> (Self, Self) {
        let (x1, x2) = self.0.split();
        let (y1, y2) = self.1.split();
        (Add(x1, y1), Add(x2, y2))
    }
}

#[derive(Copy, Clone)]
pub struct Mul<X, Y>(X, Y);
impl<X, Y> Expression for Mul<X, Y>
    where X: Expression,
          Y: Expression,
          X::Element: ops::Mul<Y::Element> + Clone,
          Y::Element: Clone
{
    type Element = <X::Element as ops::Mul<Y::Element>>::Output;
    type Values = Binary<Times, X::Values, Y::Values>;

    fn len(&self) -> usize {
        let len = self.0.len();
        debug_assert_eq!(len, self.1.len());
        len
    }

    fn values(self) -> Self::Values {
        Binary::new(Times, self.0.values(), self.1.values())
    }

    fn split(self) -> (Self, Self) {
        let (x1, x2) = self.0.split();
        let (y1, y2) = self.1.split();
        (Mul(x1, y1), Mul(x2, y2))
    }
}

macro_rules! item { ($i: item) => { $i } }

macro_rules! make_impl {
    ($name: ident, $method: ident, <$($param: tt),*> $for_: ty) => {
        item! {
            impl<$($param,)* E> ops::$name<E> for $for_
                where $for_: Expression, E: Expression,
                      <$for_ as Expression>::Element: ops::$name<E::Element>,
            {
                type Output = $name<Self, E>;

                fn $method(self, other: E) -> Self::Output {
                    assert_eq!(self.len(), other.len());
                    $name(self, other)
                }
            }
        }
    }
}
macro_rules! impls {
    ($($name: ident, $method: ident: $(<$($param: tt),*> $for_: ty),*;)*) => {
        $(
            $(make_impl!($name, $method, <$($param),*> $for_);)*
                )*
    }
}

impls! {
    Add, add: <'a, T> Value<&'a [T]>, <X, Y> Add<X, Y>, <X, Y> Mul<X, Y>;
    Mul, mul: <'a, T> Value<&'a [T]>, <X, Y> Add<X, Y>, <X, Y> Mul<X, Y>;
}
