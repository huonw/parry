pub use iterators::{Binary, Unary, UnOp, BinOp,
                    Bang,
                    Plus, Minus, Times, Divide, Pipe, Ampersand, Caret,
                    EqEq, BangEq, LessThan, LessThanEq, GreaterThan, GreaterThanEq};
use generic_simd::SimdVector;
use {Expression, Constant, E, Length, Switch, Rev};
use raw::{Zip, Map};
use std::{cmp, ops};

macro_rules! un_op_struct {
    ($($name: ident, $op: ident;)*) => {
        $(
            #[derive(Copy, Clone)]
            pub struct $name<X>(X);

            impl<X> Expression for $name<X>
                where X: Expression,
                      $op: UnOp<X::Element>,
                      $op: UnOp<X::Simd128Element>,
                      <$op as UnOp<X::Simd128Element>>::Output: SimdVector<Element = <$op as UnOp<X::Element>>::Output>
            {
                type Element = <$op as UnOp<X::Element>>::Output;
                type Values = Unary<$op, X::Values>;

                type Simd128Element = <$op as UnOp<X::Simd128Element>>::Output;
                type Simd128Values = Unary<$op, X::Simd128Values>;

                type Rev = $name<X::Rev>;

                fn length(&self) -> Length {
                    self.0.length()
                }

                fn values(self) -> Self::Values {
                    Unary::new($op, self.0.values())
                }

                fn simd128_values(self) -> (Self::Simd128Values, Self::Values) {
                    let (lo, hi) = self.0.simd128_values();
                    (Unary::new($op, lo),
                     Unary::new($op, hi))
                }

                fn split(self, round_up: bool) -> (Self, Self) {
                    let (x1, x2) = self.0.split(round_up);
                    ($name(x1), $name(x2))
                }

                fn split_at(self, n: usize) -> (Self, Self) {
                    let (x1, x2) = self.0.split_at(n);
                    ($name(x1), $name(x2))
                }

                fn rev(self) -> Self::Rev {
                    $name(self.0.rev())
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
    ($($name: ident, $module: ident :: $trayt: ident, $op: ident;)*) => {
        $(
            #[derive(Copy, Clone)]
            pub struct $name<X, Y>(X, Y);

            impl<X, Y> Expression for $name<X, Y>
                where X: Expression,
                      Y: Expression,
                      $op: BinOp<X::Element, Y::Element>,
                      $op: BinOp<X::Simd128Element, Y::Simd128Element>,
                      <$op as BinOp<X::Simd128Element, Y::Simd128Element>>::Output: SimdVector<Element = <$op as BinOp<X::Element, Y::Element>>::Output>,
            {
                type Element = <$op as BinOp<X::Element, Y::Element>>::Output;
                type Values = Binary<$op, X::Values, Y::Values>;

                type Simd128Element = <$op as BinOp<X::Simd128Element, Y::Simd128Element>>::Output;
                type Simd128Values = Binary<$op, X::Simd128Values, Y::Simd128Values>;

                type Rev = $name<X::Rev, Y::Rev>;

                fn length(&self) -> Length {
                    let len1 = self.0.length();
                    let len2 = self.1.length();
                    debug_assert!(len1.compatible(len2));
                    cmp::min(len1, len2)
                }

                fn values(self) -> Self::Values {
                    Binary::new($op, self.0.values(), self.1.values())
                }

                fn simd128_values(self) -> (Self::Simd128Values, Self::Values) {
                    let (lo0, hi0) = self.0.simd128_values();
                    let (lo1, hi1) = self.1.simd128_values();
                    (Binary::new($op, lo0, lo1),
                     Binary::new($op, hi0, hi1))
                }

                fn split(self, round_up: bool) -> (Self, Self) {
                    let (x1, x2) = self.0.split(round_up);
                    let (y1, y2) = self.1.split(round_up);
                    ($name(x1, y1), $name(x2, y2))
                }

                fn rev(self) -> Self::Rev {
                    $name(self.0.rev(),
                          self.1.rev())
                }

                fn split_at(self, n: usize) -> (Self, Self) {
                    let (x1, x2) = self.0.split_at(n);
                    let (y1, y2) = self.1.split_at(n);
                    ($name(x1, y1), $name(x2, y2))
                }
            }
            )*
    }
}

bin_op_struct! {
    Add, ops::Add, Plus;
    Sub, ops::Sub, Minus;
    Mul, ops::Mul, Times;
    Div, ops::Div, Divide;
    BitOr, ops::BitOr, Pipe;
    BitAnd, ops::BitAnd, Ampersand;
    BitXor, ops::BitXor, Caret;
    Eq, cmp::PartialEq, EqEq;
    Ne, cmp::PartialEq, BangEq;
    Lt, cmp::PartialOrd, LessThan;
    Le, cmp::PartialOrd, LessThanEq;
    Gt, cmp::PartialOrd, GreaterThan;
    Ge, cmp::PartialOrd, GreaterThanEq;
}

macro_rules! cmp_ctor {
    ($($name: ident, $fn_name: ident;)*) => {
        $(
            pub fn $fn_name<E1: Expression, E2: Expression>(e1: E1, e2: E2) -> $name<E1, E2> {
                $name(e1, e2)
            }
            )*
    }
}

cmp_ctor! {
    Eq, make_eq;
    Ne, make_ne;
    Lt, make_lt;
    Le, make_le;
    Gt, make_gt;
    Ge, make_ge;
}

macro_rules! item { ($i: item) => { $i } }

macro_rules! make_impl {
    (binary, $name: ident, $method: ident, <$($param: tt),*> $for_: ty) => {
        item! {
            impl<$($param,)* Rhs> ops::$name<Rhs> for $for_
                where $for_: Expression, Rhs: Expression,
                      <$for_ as Expression>::Element: ops::$name<Rhs::Element>,
            {
                type Output = $name<Self, Rhs>;

                fn $method(self, other: Rhs) -> Self::Output {
                    assert!(self.length().compatible(other.length()));
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
    <T> Constant<T>,
    <T> E<T>,
    <T> Rev<T>,
    <X, Y> Add<X, Y>,
    <X, Y> Sub<X, Y>,
    <X, Y> Mul<X, Y>,
    <X, Y> Div<X, Y>,
    <X, Y> BitOr<X, Y>,
    <X, Y> BitAnd<X, Y>,
    <X, Y> BitXor<X, Y>,
    <X, Y> Zip<X, Y>,
    <X, F> Map<X, F>,
    <B, T, E_> Switch<B, T, E_>,
}
