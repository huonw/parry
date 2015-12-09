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
fn add() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = Value(a) + Value(b) + Value(a);
    test(c, &[1 + 4 + 1,
              2 + 5 + 2,
              3 + 6 + 3]);
}

#[test]
fn mul() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = Value(a) * Value(b) * Value(a);
    test(c, &[1 * 4 * 1,
              2 * 5 * 2,
              3 * 6 * 3]);
}

#[test]
fn mul_add() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = Value(a) * Value(b) + Value(a);
    test(c, &[1 * 4 + 1,
              2 * 5 + 2,
              3 * 6 + 3]);

}

#[test]
fn add_mul() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = Value(a) * (Value(b) + Value(a));
    test(c, &[1 * (4 + 1),
              2 * (5 + 2),
              3 * (6 + 3)]);

}

#[test]
fn long() {
    let a = (0..1_000_000_i64).collect::<Vec<_>>();
    let b = a.clone();

    let c = Value(&a[..]) + Value(&b[..]) * Value(&a[..]);
    test(c, &a.iter().map(|&x| x + x * x).collect::<Vec<_>>());
}

#[test]
fn zip() {
    let a = (0..100).collect::<Vec<_>>();
    let b = a.clone();

    let c = Value(&a[..]).zip(Value(&b[..]));

    test(c, &a.iter().map(|&x| (x, x)).collect::<Vec<_>>());
}

#[test]
fn map() {
    let a = (0..100).collect::<Vec<_>>();

    let f = |x| x as f32 + 1.0;
    let c = Value(&a[..]).map(&f);

    test(c, &a.iter().map(|&x| x as f32 + 1.0).collect::<Vec<_>>());
}
