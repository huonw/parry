use std::{cmp, iter};
use {Expression, Length};
use iterators::{Binary, Tuple};
use generic_simd::SimdValue;

#[derive(Copy, Clone)]
pub struct Zip<X, Y>(X, Y);
pub fn make_zip<X, Y>(x: X, y: Y) -> Zip<X, Y> {
    Zip(x, y)
}

#[derive(Copy, Clone)]
pub struct Map<X, F>(X, F);

pub fn make_map<X, F>(x: X, f: F) -> Map<X, F> {
    Map(x, f)
}


impl<X: Expression, Y: Expression> Expression for Zip<X, Y> {
    type Element = (X::Element, Y::Element);
    type Values = Binary<Tuple, X::Values, Y::Values>;
    type Simd128Element = (X::Simd128Element, Y::Simd128Element);
    type Simd128Values = Binary<Tuple, X::Simd128Values, Y::Simd128Values>;
    type Rev = Zip<X::Rev, Y::Rev>;

    fn length(&self) -> Length {
        let len1 = self.0.length();
        let len2 = self.1.length();
        debug_assert!(len1.compatible(len2));
        cmp::min(len1, len2)
    }

    fn values(self) -> Self::Values {
        Binary::new(Tuple, self.0.values(), self.1.values())
    }

    fn simd128_values(self) -> (Self::Simd128Values, Self::Values) {
        let (lo0, hi0) = self.0.simd128_values();
        let (lo1, hi1) = self.1.simd128_values();
        (Binary::new(Tuple, lo0, lo1),
         Binary::new(Tuple, hi0, hi1))
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        let (x1, x2) = self.0.split(round_up);
        let (y1, y2) = self.1.split(round_up);
        (Zip(x1, y1), Zip(x2, y2))
    }

    fn rev(self) -> Self::Rev {
        (Zip(self.0.rev(), self.1.rev()))
    }

    fn split_at(self, n: usize) -> (Self, Self) {
        let (x1, x2) = self.0.split_at(n);
        let (y1, y2) = self.1.split_at(n);
        (Zip(x1, y1), Zip(x2, y2))
    }
}

impl<X: Expression, O: Send + SimdValue, F: Clone + FnMut(X::Element) -> O + Send> Expression for Map<X, F> {
    type Element = O;
    type Values = iter::Map<X::Values, F>;
    type Simd128Element = O::V128;
    type Simd128Values = Box<DoubleEndedIterator<Item = Self::Simd128Element>>;
    type Rev = Map<X::Rev, F>;

    fn length(&self) -> Length {
        self.0.length()
    }

    fn values(self) -> Self::Values {
        self.0.values().map(self.1)
    }

    fn simd128_values(self) -> (Self::Simd128Values, Self::Values) {
        unimplemented!()
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        let (x1, x2) = self.0.split(round_up);
        let f2 = self.1.clone();
        (Map(x1, self.1), Map(x2, f2))
    }

    fn rev(self) -> Self::Rev {
        Map(self.0.rev(), self.1)
    }

    fn split_at(self, n: usize) -> (Self, Self) {
        let (x1, x2) = self.0.split_at(n);
        let f2 = self.1.clone();
        (Map(x1, self.1), Map(x2, f2))
    }
}
