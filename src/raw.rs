use std::iter;
use {Expression};
use iterators::{Binary, Tuple};

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

    fn len(&self) -> usize {
        let len = self.0.len();
        debug_assert_eq!(len, self.1.len());
        len
    }

    fn values(self) -> Self::Values {
        Binary::new(Tuple, self.0.values(), self.1.values())
    }

    fn split(self) -> (Self, Self) {
        let (x1, x2) = self.0.split();
        let (y1, y2) = self.1.split();
        (Zip(x1, y1), Zip(x2, y2))
    }
}

impl<X: Expression, O, F: Clone + FnMut(X::Element) -> O> Expression for Map<X, F> {
    type Element = O;
    type Values = iter::Map<X::Values, F>;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn values(self) -> Self::Values {
        self.0.values().map(self.1)
    }

    fn split(self) -> (Self, Self) {
        let (x1, x2) = self.0.split();
        let f2 = self.1.clone();
        (Map(x1, self.1), Map(x2, f2))
    }
}
