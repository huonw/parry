use std::{iter, slice};
use {Expression, Length};
use generic_simd::SimdValue;

#[derive(Copy, Clone)]
pub struct Constant<T>(pub T);

impl<'a, T> Expression for Constant<T>
    where T: 'a + Send + Clone + SimdValue, T::V128: Clone,
{
    type Element = T;
    type Values = iter::Repeat<T>;
    type Simd128Element = T::V128;
    type Simd128Values = iter::Repeat<T::V128>;
    type Rev = Self;

    fn length(&self) -> Length {
        Length::Infinite
    }

    fn values(self) -> Self::Values {
        iter::repeat(self.0)
    }
    fn simd128_values(self) -> (Self::Simd128Values, Self::Values) {
        (iter::repeat(self.0.clone().splat_128()),
         iter::repeat(self.0))
    }

    fn split(self, _round_up: bool) -> (Self, Self) {
        (self.clone(), self)
    }

    fn rev(self) -> Self {
        self
    }

    fn split_at(self, _n: usize) -> (Self, Self) {
        (self.clone(), self)
    }
}

#[derive(Copy, Clone)]
pub struct E<T>(pub T);

impl<X: Expression> Expression for E<X> {
    type Element = X::Element;
    type Values = X::Values;
    type Simd128Element = X::Simd128Element;
    type Simd128Values = X::Simd128Values;
    type Rev = E<X::Rev>;

    fn length(&self) -> Length {
        self.0.length()
    }

    fn values(self) -> Self::Values {
        self.0.values()
    }
    fn simd128_values(self) -> (Self::Simd128Values, Self::Values) {
        self.0.simd128_values()
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        let (lo, hi) = self.0.split(round_up);
        (E(lo), E(hi))
    }

    fn rev(self) -> Self::Rev {
        E(self.0.rev())
    }

    fn split_at(self, n: usize) -> (Self, Self) {
        let (lo, hi) = self.0.split_at(n);
        (E(lo), E(hi))
    }
}

impl<'a, T> Expression for &'a [T]
    where T: 'a + Sync + SimdValue + Clone, T::V128: Clone
{
    type Element = T;
    type Values = iter::Cloned<slice::Iter<'a, T>>;
    type Simd128Element = T::V128;
    type Simd128Values = iter::Cloned<slice::Iter<'a, T::V128>>;
    type Rev = Rev<&'a [T]>;

    fn length(&self) -> Length {
        Length::Finite((*self).len())
    }

    fn values(self) -> Self::Values {
        self.iter().cloned()
    }
    fn simd128_values(self) -> (Self::Simd128Values, Self::Values) {
        use std::{mem, slice};
        let simd_align = mem::align_of::<Self::Simd128Element>();
        let plain_align = mem::align_of::<Self::Element>();
        let plain_size = mem::size_of::<Self::Element>();
        let plain_per_simd = mem::size_of::<Self::Simd128Element>() / plain_size;
        assert!(simd_align % plain_align == 0);

        let start_p = self.as_ptr();
        let start_alignment = start_p as usize % simd_align;
        let start_offset_bytes = if start_alignment == 0 { 0 } else { simd_align - start_alignment };
        let start_offset = start_offset_bytes / plain_size;

        let end_p = unsafe {start_p.offset(self.len() as isize)};
        let end_offset_bytes = end_p as usize % simd_align;
        let end_offset = end_offset_bytes / plain_size;

        let middle_len = self.len() - start_offset - end_offset;

        unsafe {
            let lo: &'a [Self::Element] = slice::from_raw_parts(start_p, start_offset);
            assert_eq!(start_offset, 0);

            let middle_simd_p = start_p.offset(start_offset as isize) as *const Self::Simd128Element;
            let middle_simd_len = middle_len / plain_per_simd;
            let middle: &'a [Self::Simd128Element] = slice::from_raw_parts(middle_simd_p,
                                                                           middle_simd_len);

            let hi_p = start_p.offset((start_offset + middle_len) as isize);
            let hi: &'a [Self::Element] = slice::from_raw_parts(hi_p, end_offset);

            (middle.iter().cloned(), hi.iter().cloned())
        }
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        self.split_at((self.len() + round_up as usize) / 2)
    }

    fn rev(self) -> Self::Rev {
        Rev(self)
    }

    fn split_at(self, n: usize) -> (Self, Self) {
        (*self).split_at(n)
    }
}

impl<'a, T: 'a + Send + SimdValue> Expression for &'a mut [T] {
    type Element = &'a mut T;
    type Values = slice::IterMut<'a, T>;
    type Simd128Element = &'a mut T::V128;
    type Simd128Values = slice::IterMut<'a, T::V128>;
    type Rev = Rev<&'a mut [T]>;

    fn length(&self) -> Length {
        Length::Finite((**self).len())
    }

    fn values(self) -> Self::Values {
        self.iter_mut()
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        let len = (*self).len();
        self.split_at_mut((len + round_up as usize) / 2)
    }


    fn simd128_values(self) -> (Self::Simd128Values, Self::Values) {
        use std::{mem, slice};
        let simd_align = mem::align_of::<Self::Simd128Element>();
        let plain_align = mem::align_of::<Self::Element>();
        let plain_size = mem::size_of::<Self::Element>();
        let plain_per_simd = mem::size_of::<Self::Simd128Element>() / plain_size;
        assert!(simd_align % plain_align == 0);

        let start_p = self.as_mut_ptr();
        let start_alignment = start_p as usize % simd_align;
        let start_offset_bytes = if start_alignment == 0 { 0 } else { simd_align - start_alignment };
        let start_offset = start_offset_bytes / plain_size;

        let end_p = unsafe {start_p.offset(self.len() as isize)};
        let end_offset_bytes = end_p as usize % simd_align;
        let end_offset = end_offset_bytes / plain_size;

        let middle_len = self.len() - start_offset - end_offset;

        unsafe {
            let lo: &'a mut [T] = slice::from_raw_parts_mut(start_p, start_offset);
            assert_eq!(start_offset, 0);

            let middle_simd_p = start_p.offset(start_offset as isize) as *mut _;
            let middle_simd_len = middle_len / plain_per_simd;
            let middle: &'a mut [_] = slice::from_raw_parts_mut(middle_simd_p,
                                                                middle_simd_len);

            let hi_p = start_p.offset((start_offset + middle_len) as isize);
            let hi: &'a mut [T] = slice::from_raw_parts_mut(hi_p, end_offset);

            (middle.iter_mut(), hi.iter_mut())
        }
    }

    fn rev(self) -> Self::Rev {
        Rev(self)
    }

    fn split_at(self, n: usize) -> (Self, Self) {
        (*self).split_at_mut(n)
    }
}

#[derive(Copy, Clone)]
pub struct Rev<T>(T);

impl<T: Expression> Expression for Rev<T> {
    type Element = T::Element;
    type Values = iter::Rev<T::Values>;

    type Simd128Element = T::Simd128Element;
    // FIXME: this is wrong (needs to reverse the internal elements)
    type Simd128Values = iter::Rev<T::Simd128Values>;
    type Rev = T;

    fn length(&self) -> Length {
        self.0.length()
    }

    fn values(self) -> Self::Values {
        self.0.values().rev()
    }

    fn simd128_values(self) -> (Self::Simd128Values, Self::Values) {
        unimplemented!()
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        let (a, b) = self.0.split(!round_up);
        (Rev(b), Rev(a))
    }

    fn rev(self) -> T {
        self.0
    }

    fn split_at(self, n: usize) -> (Self, Self) {
        let len = match self.length() {
            Length::Finite(len) => len,
            Length::Infinite => n // whatever
        };
        let (a, b) = self.0.split_at(len - n);
        (Rev(b), Rev(a))
    }
}
