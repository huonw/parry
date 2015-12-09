extern crate rayon;

use std::{cmp, iter, slice};

mod operator;
pub use operator::{Add, Mul};
mod raw;
pub use raw::{Zip, Map};

mod iterators;
pub use iterators::{Binary, Plus, Times, Tuple};

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
    assert_eq!(len, e.len());

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

pub trait Expression {
    type Element;
    type Values: Iterator<Item = Self::Element>;

    fn len(&self) -> usize;

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

    fn len(&self) -> usize {
        self.0.len()
    }

    fn values(self) -> Self::Values {
        self.0.iter().cloned()
    }

    fn split(self) -> (Self, Self) {
        let len = self.0.len();
        let half = len/2;
        (Value(&self.0[..half]),
         Value(&self.0[half..]))
    }
}
