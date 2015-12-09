#![cfg(feature = "test-everything")]
extern crate parry;
use parry::{Expression, Value};
use std::fmt::Debug;

fn test<E>(e: E, expected: &[E::Element])
    where E: Expression + Send, E::Element: Clone + PartialEq + Debug + Send

{
    let mut out = expected.to_owned();
    parry::evaluate(&mut out, e);
    assert_eq!(out, expected);
}

#[test]
fn integer() {
    let a = (0..100).collect::<Vec<_>>();
    let a = &a[..];

    let f = |(x, y)| x + y;
    let c = Value(a) + a ^ (Value(a) * !Value(a) / -Value(a) - a & a | a).zip(a).map(&f);

    test(c,
         &a.iter().map(|&x| x + x ^ ((x * !x / -x - x & x | x) + x)).collect::<Vec<_>>())
}

#[test]
fn float() {
    let a = (0..100).map(|x| x as f64).collect::<Vec<_>>();
    let a = &a[..];

    let f = |(x, y)| x + y;
    let c = (Value(a) + Value(a) * a / -Value(a) - a).zip(a).map(&f);

    test(c,
         &a.iter().map(|&x| x + (x * x / -x - x) + x).collect::<Vec<_>>())
}
