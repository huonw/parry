pub use iterators::{Binary, Unary,
                    Bang,
                    Plus, Minus, Times, Divide, Pipe, Ampersand, Caret};
use {Expression, Value};
use raw::{Zip, Map};
use std::ops;

macro_rules! un_op_struct {
    ($($name: ident, $op: ident;)*) => {
        $(
            #[derive(Copy, Clone)]
            pub struct $name<X>(X);

            impl<X> Expression for $name<X>
                where X: Expression,
                      X::Element: ops::$name + Clone,
            {
                type Element = <X::Element as ops::$name>::Output;
                type Values = Unary<$op, X::Values>;

                fn len(&self) -> usize {
                    self.0.len()
                }

                fn values(self) -> Self::Values {
                    Unary::new($op, self.0.values())
                }

                fn split(self) -> (Self, Self) {
                    let (x1, x2) = self.0.split();
                    ($name(x1), $name(x2))
                }
            }
            )*
    }
}

un_op_struct! {
    Not, Bang;
    Neg, Minus;
}

macro_rules! bin_op_struct {
    ($($name: ident, $op: ident;)*) => {
        $(
            #[derive(Copy, Clone)]
            pub struct $name<X, Y>(X, Y);

            impl<X, Y> Expression for $name<X, Y>
                where X: Expression,
                      Y: Expression,
                      X::Element: ops::$name<Y::Element> + Clone,
                      Y::Element: Clone
            {
                type Element = <X::Element as ops::$name<Y::Element>>::Output;
                type Values = Binary<$op, X::Values, Y::Values>;

                fn len(&self) -> usize {
                    let len = self.0.len();
                    debug_assert_eq!(len, self.1.len());
                    len
                }

                fn values(self) -> Self::Values {
                    Binary::new($op, self.0.values(), self.1.values())
                }

                fn split(self) -> (Self, Self) {
                    let (x1, x2) = self.0.split();
                    let (y1, y2) = self.1.split();
                    ($name(x1, y1), $name(x2, y2))
                }
            }
            )*
    }
}

bin_op_struct! {
    Add, Plus;
    Sub, Minus;
    Mul, Times;
    Div, Divide;
    BitOr, Pipe;
    BitAnd, Ampersand;
    BitXor, Caret;
}

macro_rules! item { ($i: item) => { $i } }

macro_rules! make_impl {
    (binary, $name: ident, $method: ident, <$($param: tt),*> $for_: ty) => {
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
    };
    (unary, $name: ident, $method: ident, <$($param: tt),*> $for_: ty) => {
        item! {
            impl<$($param,)*> ops::$name for $for_
                where $for_: Expression,
                      <$for_ as Expression>::Element: ops::$name,
            {
                type Output = $name<Self>;

                fn $method(self) -> Self::Output {
                    $name(self)
                }
            }
        }
    }
}
macro_rules! impls {
    ([] $($__: tt)*) => {};

    ([
        $kind: ident, $trayt: ident, $method: ident;
        $($kind_rest: ident, $trayt_rest: ident, $method_rest: ident;)*
            ]
     $(<$($param: tt),*> $for_: ty,)*) => {
        $(
            make_impl!($kind, $trayt, $method, <$($param),*> $for_);
                )*
            impls!([$($kind_rest, $trayt_rest, $method_rest;)*]
                   $(<$($param),*> $for_,)*);
    }
}


impls! {
    [
        unary, Not, not;
        unary, Neg, neg;
        binary, Add, add;
        binary, Sub, sub;
        binary, Mul, mul;
        binary, Div, div;
        binary, BitOr, bitor;
        binary, BitAnd, bitand;
        binary, BitXor, bitxor;
    ]
    <'a, T> Value<&'a [T]>,
    <X, Y> Add<X, Y>,
    <X, Y> Sub<X, Y>,
    <X, Y> Mul<X, Y>,
    <X, Y> Div<X, Y>,
    <X, Y> BitOr<X, Y>,
    <X, Y> BitAnd<X, Y>,
    <X, Y> BitXor<X, Y>,
    <X, Y> Zip<X, Y>,
    <X, F> Map<X, F>,
}
