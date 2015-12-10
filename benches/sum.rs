#![feature(test)]
extern crate test;

extern crate crossbeam;
extern crate rayon;
extern crate parry;
use parry::{Expression, E};

use test::Bencher as B;

fn seq(b: &mut B, n: i64) {
    let x = (0..n).collect::<Vec<_>>();
    b.iter(|| {
        x.iter().cloned().fold(0, |x, y| x + y)
    })
}
fn par(b: &mut B, n: i64) {
    rayon::initialize();
    let x = (0..n).collect::<Vec<_>>();
    b.iter(|| {
        E(&x[..]).sum()
    })
}

macro_rules! tests {
    ($($par_name: ident, $seq_name: ident;)*) => {
        $(
            #[bench]
            fn $par_name(b: &mut B) {
                let n = stringify!($par_name).split('_').nth(1).unwrap().parse().unwrap();
                par(b, n)
            }
            #[bench]
            fn $seq_name(b: &mut B) {
                let n = stringify!($par_name).split('_').nth(1).unwrap().parse().unwrap();
                seq(b, n)
            }
            )*
    }
}

tests! {
    sum_00001000_p, sum_00001000_s;
    sum_00010000_p, sum_00010000_s;
    sum_00100000_p, sum_00100000_s;
    sum_01000000_p, sum_01000000_s;
    sum_10000000_p, sum_10000000_s;
}
