use std::{cmp, ops};
use rayon;
use {Expression, Length};

const MIN_THRESHOLD: usize = 1024;
const MAX_COUNT: usize = 32;

pub trait Reduce<T>: Send {
    type Output: Send;
    type Scalar: ReduceScalar<Self::Output>;

    fn expected_length(&self) -> Length;
    fn split(self) -> (Self, Self, Self::Scalar);
    fn reduce<X>(&mut self, X) -> Self::Output
        where X: Iterator<Item = T>;
}

pub trait ReduceScalar<X> {
    fn combine(self, X, X) -> X;
}

fn get_len(l1: Length, l2: Length) -> usize {
    match cmp::min(l1, l2) {
        Length::Finite(x) => x,
        Length::Infinite => panic!("trying to reduce an infinite expression into an infinite reducer")
    }
}

pub fn evaluate<E, R>(e: E, reduce: R) -> R::Output
    where E: Expression, R: Reduce<E::Element>
{
    let len = get_len(e.length(), reduce.expected_length());

    eval_inner(e, cmp::max(len / MAX_COUNT, MIN_THRESHOLD), reduce)
}

fn eval_inner<E, R>(e: E, threshold: usize, mut reduce: R) -> R::Output
    where E: Expression, R: Reduce<E::Element>
{
    let len = get_len(e.length(), reduce.expected_length());
    assert!(e.length().compatible(reduce.expected_length()));

    if len > threshold {
        let (low, high, scalar) = reduce.split();
        let (e_low, e_high) = e.split();

        let (a, b) = rayon::join(|| eval_inner(e_low, threshold, low),
                                 || eval_inner(e_high, threshold, high));

        scalar.combine(a, b)
    } else {
        reduce.reduce(e.values())
    }
}

impl ReduceScalar<()> for () {
    fn combine(self, _: (), _: ()) -> () {
        ()
    }
}

pub struct SetArray<'a, T: 'a>(pub &'a mut [T]);
impl<'a, T> Reduce<T> for SetArray<'a, T>
    where T: Send
{
    type Output = ();
    type Scalar = ();

    fn expected_length(&self) -> Length {
        Length::Finite(self.0.len())
    }

    fn split(self) -> (Self, Self, ()) {
        let half = self.0.len() / 2;
        let (lo, hi) = self.0.split_at_mut(half);
        (SetArray(lo), SetArray(hi), ())
    }

    fn reduce<X>(&mut self, vals: X) -> Self::Output
        where X: Iterator<Item = T>
    {
        for (o, i) in self.0.iter_mut().zip(vals) {
            *o = i;
        }
    }
}
