use std::{cmp, ops};

pub use switch::SwitchIter;

pub struct Plus;
pub struct Minus;
pub struct Times;
pub struct Divide;
pub struct Pipe;
pub struct Ampersand;
pub struct Caret;
pub struct Bang;

pub struct EqEq;
pub struct BangEq;
pub struct LessThan;
pub struct LessThanEq;
pub struct GreaterThan;
pub struct GreaterThanEq;

pub struct Tuple;

pub trait UnOp<A> {
    type Output;
    fn operate(&self, a: A) -> Self::Output;
}

pub trait BinOp<A, B> {
    type Output;
    fn operate(&self, a: A, b: B) -> Self::Output;
}

macro_rules! un_op {
    ($($ty: ty, $trayt: ident, $method: ident;)*) => {
        $(
            impl<A> UnOp<A> for $ty
                where A: ops::$trayt
            {
                type Output = A::Output;

                fn operate(&self, a: A) -> Self::Output {
                    ops::$trayt::$method(a)
                }
            }
            )*
    }
}
macro_rules! bin_op {
    ($($ty: ty, $trayt: ident, $method: ident;)*) => {
        $(
            impl<A, B> BinOp<A, B> for $ty
                where A: ops::$trayt<B>
            {
                type Output = A::Output;

                fn operate(&self, a: A, b: B) -> Self::Output {
                    ops::$trayt::$method(a, b)
                }
            }
            )*
    }
}

macro_rules! cmp_op {
    ($($ty: ty, $trayt: ident, $method: ident;)*) => {
        $(
            impl<A, B> BinOp<A, B> for $ty
                where A: cmp::$trayt<B>
            {
                type Output = bool;

                fn operate(&self, a: A, b: B) -> Self::Output {
                    cmp::$trayt::$method(&a, &b)
                }
            }
            )*
    }
}

un_op! {
    Minus, Neg, neg;
    Bang, Not, not;
}
bin_op! {
    Plus, Add, add;
    Minus, Sub, sub;
    Times, Mul, mul;
    Divide, Div, div;
    Pipe, BitOr, bitor;
    Ampersand, BitAnd, bitand;
    Caret, BitXor, bitxor;
}
cmp_op! {
    EqEq, PartialEq, eq;
    BangEq, PartialEq, ne;
    LessThan, PartialOrd, lt;
    LessThanEq, PartialOrd, le;
    GreaterThan, PartialOrd, gt;
    GreaterThanEq, PartialOrd, ge;
}

impl<A, B> BinOp<A,B> for Tuple {
    type Output = (A, B);

    fn operate(&self, a: A, b: B) -> Self::Output {
        (a, b)
    }
}

pub struct Unary<Op, X> {
    op: Op,
    x: X,
}

impl<Op, X> Unary<Op, X>
    where Op: UnOp<X::Item>,
          X: Iterator
{
    pub fn new(op: Op, x: X) -> Self {
        Unary {
            op: op,
            x: x,
        }
    }
}

impl<Op, X> Iterator for Unary<Op, X>
    where Op: UnOp<X::Item>,
          X: Iterator
{
    type Item = Op::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.x.next().map(|a| self.op.operate(a))
    }
}

impl<Op, X> DoubleEndedIterator for Unary<Op, X>
    where Op: UnOp<X::Item>,
          X: DoubleEndedIterator
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.x.next_back().map(|a| self.op.operate(a))
    }
}

pub struct Binary<Op, X, Y> {
    op: Op,
    x: X,
    y: Y,
}

impl<Op, X, Y> Binary<Op, X, Y>
    where Op: BinOp<X::Item, Y::Item>,
          X: Iterator, Y: Iterator
{
    pub fn new(op: Op, x: X, y: Y) -> Self {
        Binary {
            op: op,
            x: x,
            y: y,
        }
    }
}

impl<Op, X, Y> Iterator for Binary<Op, X, Y>
    where Op: BinOp<X::Item, Y::Item>,
          X: Iterator, Y: Iterator
{
    type Item = Op::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.x.next().and_then(|a| self.y.next().map(|b| self.op.operate(a, b)))
    }
}

impl<Op, X, Y> DoubleEndedIterator for Binary<Op, X, Y>
    where Op: BinOp<X::Item, Y::Item>,
          X: DoubleEndedIterator, Y: DoubleEndedIterator
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.x.next_back().and_then(|a| self.y.next().map(|b| self.op.operate(a, b)))
    }
}
