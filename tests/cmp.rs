extern crate parry;
use parry::{Expression, E};
use std::fmt::Debug;

fn test<E>(e: E, expected: &[E::Element])
    where E: Expression + Send, E::Element: Clone + PartialEq + Debug + Send

{
    let mut out = expected.to_owned();
    e.write(&mut out[..]);
    assert_eq!(out, expected);
}

#[test]
fn eq() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[1, 0, 3, 4] as &[_];

    test(E(a).eq(b), &[true, false, true, true]);
}

#[test]
fn ne() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[1, 0, 3, 0] as &[_];

    test(E(a).ne(b), &[false, true, false, true]);
}

#[test]
fn lt() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[4, 3, 2, 1] as &[_];

    test(E(a).lt(b), &[true, true, false, false]);
}

#[test]
fn le() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[4, 3, 3, 1] as &[_];

    test(E(a).le(b), &[true, true, true, false]);
}

#[test]
fn gt() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[4, 3, 2, 1] as &[_];

    test(E(a).gt(b), &[false, false, true, true]);
}

#[test]
fn ge() {
    let a = &[1, 2, 3, 4] as &[_];
    let b = &[4, 3, 3, 1] as &[_];

    test(E(a).ge(b), &[false, false, true, true]);
}
