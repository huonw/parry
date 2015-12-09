#![feature(test)]
extern crate test;

extern crate crossbeam;
extern crate rayon;
extern crate parry;
use parry::{Expression, Value};

use test::Bencher as B;

fn add_seq(b: &mut B, n: i64) {
    let x = (0..n).collect::<Vec<_>>();
    let y = x.clone();
    let mut out = x.clone();
    b.iter(|| {
        let values = parry::iterators::Binary::new(parry::iterators::Plus,
                                                   x.iter().cloned(), y.iter().cloned())
            .map(|x| 1_000_000 / (x + 1));
        for (o, i) in out.iter_mut().zip(values) {
            *o = i;
        }
    })
}
fn add_par(b: &mut B, n: i64) {
    rayon::initialize();
    let x = (0..n).collect::<Vec<_>>();
    let y = x.clone();
    let mut out = x.clone();
    b.iter(|| {
        let e = Value(&test::black_box(&x)[..]) + Value(&test::black_box(&y)[..]);
        let f = |x| 1_000_000 / (x + 1);
        parry::evaluate(&mut out, e.map(&f));
    })
}

macro_rules! tests {
    ($par: ident, $seq: ident: $($par_name: ident, $seq_name: ident;)*) => {
        $(
            #[bench]
            fn $par_name(b: &mut B) {
                let n = stringify!($par_name).split('_').nth(1).unwrap().parse().unwrap();
                add_par(b, n)
            }
            #[bench]
            fn $seq_name(b: &mut B) {
                let n = stringify!($par_name).split('_').nth(1).unwrap().parse().unwrap();
                add_seq(b, n)
            }
            )*
    }
}

tests! {
    add_par, add_seq:
    add_0001000_p, add_0001000_s;
    add_0010000_p, add_0010000_s;
    add_0100000_p, add_0100000_s;
    add_1000000_p, add_1000000_s;
}
