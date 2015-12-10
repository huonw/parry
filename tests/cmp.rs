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
fn eq() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[1, 0, 3, 4] as &[_];

    test(Value(a).eq(b), &[true, false, true, true]);
}

#[test]
fn ne() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[1, 0, 3, 0] as &[_];

    test(Value(a).ne(b), &[false, true, false, true]);
}

#[test]
fn lt() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[4, 3, 2, 1] as &[_];

    test(Value(a).lt(b), &[true, true, false, false]);
}

#[test]
fn le() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[4, 3, 3, 1] as &[_];

    test(Value(a).le(b), &[true, true, true, false]);
}

#[test]
fn gt() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[4, 3, 2, 1] as &[_];

    test(Value(a).gt(b), &[false, false, true, true]);
}

#[test]
fn ge() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[4, 3, 3, 1] as &[_];

    test(Value(a).ge(b), &[false, false, true, true]);
}
