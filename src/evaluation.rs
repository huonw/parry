use std::cmp;

use rayon;
use {Expression, Length};
use reduce::{Reduce, ReduceScalar};

const MIN_THRESHOLD: usize = 4096;
const MAX_COUNT: usize = 32;

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

fn eval_inner<E, R>(e: E, threshold: usize, reduce: R) -> R::Output
    where E: Expression, R: Reduce<E::Element>
{
    let len = get_len(e.length(), reduce.expected_length());
    assert!(e.length().compatible(reduce.expected_length()));

    if len > threshold {
        let (low, high, scalar) = reduce.split();
        let (e_low, e_high) = e.split(false);

        let (a, b) = rayon::join(|| eval_inner(e_low, threshold, low),
                                 || eval_inner(e_high, threshold, high));

        scalar.combine(a, b)
    } else {
        reduce.reduce(e)
    }
}
