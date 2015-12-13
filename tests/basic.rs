extern crate parry;
use parry::{Expression, E};
use std::fmt::Debug;

fn test<E>(e: E, expected: &[E::Element])
    where E: Expression + Send + Clone, E::Element: Clone + PartialEq + Debug + Send

{
    let mut out = expected.to_owned();
    e.clone().write(&mut out[..]);
    assert_eq!(out, expected);

    e.clone().rev().write(&mut out[..]);
    out.reverse();
    assert_eq!(out, expected);

    e.rev().write(out[..].rev());
    assert_eq!(out, expected);
}


#[test]
fn neg() {
    let a = &[1, 2, 3] as &[_];
    let c = -E(a);
    test(c, &[-1,
              -2,
              -3]);
}

#[test]
fn not() {
    let a = &[1, 2, 3] as &[_];
    let c = !E(a);
    test(c, &[!1,
              !2,
              !3]);
}

#[test]
fn add() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = E(a) + b + a;
    test(c, &[1 + 4 + 1,
              2 + 5 + 2,
              3 + 6 + 3]);
}

#[test]
fn sub() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = E(a) - b - a;
    test(c, &[1 - 4 - 1,
              2 - 5 - 2,
              3 - 6 - 3]);
}

#[test]
fn mul() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = E(a) * b * a;
    test(c, &[1 * 4 * 1,
              2 * 5 * 2,
              3 * 6 * 3]);
}

#[test]
fn div() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = E(b) / a;
    test(c, &[4 / 1,
              5 / 2,
              6 / 3]);
}

#[test]
fn bitor() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = E(a) | b | a;
    test(c, &[1 | 4 | 1,
              2 | 5 | 2,
              3 | 6 | 3]);
}

#[test]
fn bitand() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = E(a) & b & a;
    test(c, &[1 & 4 & 1,
              2 & 5 & 2,
              3 & 6 & 3]);
}

#[test]
fn bitxor() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = E(a) ^ b ^ a;
    test(c, &[1 ^ 4 ^ 1,
              2 ^ 5 ^ 2,
              3 ^ 6 ^ 3]);
}

#[test]
fn mul_add() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = E(a) * b + a;
    test(c, &[1 * 4 + 1,
              2 * 5 + 2,
              3 * 6 + 3]);

}

#[test]
fn add_mul() {
    let a = &[1, 2, 3] as &[_];
    let b = &[4, 5, 6] as &[_];
    let c = E(a) * (E(b) + a);
    test(c, &[1 * (4 + 1),
              2 * (5 + 2),
              3 * (6 + 3)]);
}

#[test]
fn long() {
    let a = (0..1_000_000_i64).collect::<Vec<_>>();
    let b = a.clone();

    let c = E(&a[..]) + E(&b[..]) * &a[..];
    test(c, &a.iter().map(|&x| x + x * x).collect::<Vec<_>>());
}

#[test]
fn long_forward_rev() {
    let a = (0..1_000_000_i64).collect::<Vec<_>>();
    let b = a.clone();

    let c = E(&a[..]) + b.rev();
    test(c, &a.iter().map(|_| 999_999).collect::<Vec<_>>());
}

#[test]
fn zip() {
    let a = (0..100).collect::<Vec<_>>();
    let b = a.clone();

    let c = E(&a[..]).zip(&b[..]);

    test(c, &a.iter().map(|&x| (x, x)).collect::<Vec<_>>());
}

#[test]
fn map() {
    let a = (0..100).collect::<Vec<_>>();

    let f = |x| x as f32 + 1.0;
    let c = E(&a[..]).map(&f);

    test(c, &a.iter().map(|&x| x as f32 + 1.0).collect::<Vec<_>>());
}

#[test]
fn add_constant() {
    let a = &[0, 1, 2, 3] as &[_];

    let c = parry::Constant(10) + a;

    test(c, &[0 + 10,
              1 + 10,
              2 + 10,
              3 + 10]);
}

#[test]
fn switch() {
    let cond = &[true, true, false, true] as &[_];
    let a = &[0, 1, 2, 3] as &[_];
    let b = &[10, 11, 12, 13] as &[_];

    let c = E(cond).switch(a, b);

    test(c.clone(), &[0, 1, 12, 3]);
}

#[test]
fn sum() {
    let a = &[0, 1, 2, 3, 4, 5, 6, 7] as &[_];

    assert_eq!(E(a).sum(), (0..8).fold(0, |a, b| a + b));
}

#[test]
fn max() {
    let a = &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0] as &[_];
    assert_eq!(E(a).max(), Some(5.0));
    assert_eq!((-E(a)).max(), Some(0.0));
}

#[test]
fn min() {
    let a = &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0] as &[_];
    assert_eq!(E(a).min(), Some(0.0));
    assert_eq!((-E(a)).min(), Some(-5.0));
}

#[test]
fn split_at() {
    let a = &[0, 1, 2, 3, 4, 5] as &[_];
    let (b1, b2) = a.split_at(2);
    test(b1, &[0, 1]);
    test(b2, &[2, 3, 4, 5]);

    let (b1, b2) = a.rev().split_at(2);
    test(b1, &[5, 4]);
    test(b2, &[3, 2, 1, 0]);
}
