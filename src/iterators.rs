use std::ops;

pub struct Plus;
pub struct Times;
pub struct Tuple;

trait BinOp<A, B> {
    type Output;
    fn operate(&self, a: A, b: B) -> Self::Output;
}

impl<A, B> BinOp<A, B> for Plus
    where A: ops::Add<B>
{
    type Output = A::Output;

    fn operate(&self, a: A, b: B) -> Self::Output {
        a + b
    }
}

impl<A, B> BinOp<A, B> for Times
    where A: ops::Mul<B>
{
    type Output = A::Output;

    fn operate(&self, a: A, b: B) -> Self::Output {
        a * b
    }
}
impl<A, B> BinOp<A,B> for Tuple {
    type Output = (A, B);

    fn operate(&self, a: A, b: B) -> Self::Output {
        (a, b)
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
