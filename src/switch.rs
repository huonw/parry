use simd::*;
use {Expression, Length};
use std::cmp;

pub trait SwitchOn<T> {
    fn switch(self, t: T, e: T) -> T;
}

impl<T> SwitchOn<T> for bool {
    fn switch(self, t: T, e: T) -> T {
        if self { t } else { e }
    }
}

macro_rules! switch_simd {
    ($($val: ident, $bool: ident;)*) => {
        $(
            impl SwitchOn<$val> for $bool {
                fn switch(self, t: $val, e: $val) -> $val {
                    self.select(t, e)
                }
            }
            )*
    }
}
switch_simd! {
    i8x16, bool8ix16;
    i16x8, bool16ix8;
    i32x4, bool32ix4;
    u8x16, bool8ix16;
    u16x8, bool16ix8;
    u32x4, bool32ix4;
    f32x4, bool32fx4;
}

macro_rules! switch_simd_scalar {
    ($($val: ident, $bool: ident;)*) => {
        $(
            impl SwitchOn<$val> for $bool {
                fn switch(self, t: $val, e: $val) -> $val {
                    if self.into() { t } else { e }
                }
            }
            )*
    }
}
switch_simd_scalar! {
    i8, bool8i;
    i16, bool16i;
    i32, bool32i;
    u8, bool8i;
    u16, bool16i;
    u32, bool32i;
    f32, bool32f;
}

pub struct SwitchIter<B, T, E> {
    cond: B,
    then: T,
    else_: E,
}

impl<B, T, E> SwitchIter<B, T, E>
    where B: Iterator, B::Item: SwitchOn<T::Item>, T: Iterator, E: Iterator<Item = T::Item>
{
    pub fn new(cond: B, then: T, else_: E) -> SwitchIter<B, T, E> {
        SwitchIter { cond: cond, then: then, else_: else_ }
    }
}

impl<B, T, E> Iterator for SwitchIter<B, T, E>
    where B: Iterator, B::Item: SwitchOn<T::Item>, T: Iterator, E: Iterator<Item = T::Item>
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.cond.next(), self.then.next(), self.else_.next()) {
            (Some(c), Some(t), Some(e)) => Some(c.switch(t, e)),
            _ => None
        }
    }
}

impl<B, T, E> DoubleEndedIterator for SwitchIter<B, T, E>
    where B: DoubleEndedIterator, B::Item: SwitchOn<T::Item>, T: DoubleEndedIterator, E: DoubleEndedIterator<Item = T::Item>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match (self.cond.next(), self.then.next(), self.else_.next()) {
            (Some(c), Some(t), Some(e)) => Some(c.switch(t, e)),
            _ => None
        }
    }
}


#[derive(Copy, Clone)]
pub struct Switch<B, T, E>(B, T, E);
pub fn make_switch<B, T, E>(b: B, t: T, e: E) -> Switch<B, T, E>
    where B: Expression,
          B::Element: SwitchOn<T::Element>,
          B::Simd128Element: SwitchOn<T::Simd128Element>,
          T: Expression,
          E: Expression<Element = T::Element>,
{
    Switch(b, t, e)
}

impl<B, T, E> Expression for Switch<B, T, E>
    // this should be, like, "bool"-like or something
    where B: Expression,
          B::Element: SwitchOn<T::Element>,
          B::Simd128Element: SwitchOn<T::Simd128Element>,
          T: Expression,
          E: Expression<Element = T::Element, Simd128Element = T::Simd128Element>,
{
    type Element = T::Element;
    type Values = SwitchIter<B::Values, T::Values, E::Values>;
    type Simd128Element = T::Simd128Element;
    type Simd128Values = SwitchIter<B::Simd128Values, T::Simd128Values, E::Simd128Values>;
    type Rev = Switch<B::Rev, T::Rev, E::Rev>;

    fn length(&self) -> Length {
        let len_b = self.0.length();
        let len_t = self.1.length();
        let len_e = self.2.length();
        debug_assert!(len_b.compatible(len_t) && len_b.compatible(len_e) && len_t.compatible(len_e));
        cmp::min(cmp::min(len_b, len_t), len_e)
    }

    fn values(self) -> Self::Values {
        SwitchIter::new(self.0.values(), self.1.values(), self.2.values())
    }
    fn simd128_values(self) -> (Self::Simd128Values, Self::Values) {
        let (b_l, b_h) = self.0.simd128_values();
        let (t_l, t_h) = self.1.simd128_values();
        let (e_l, e_h) = self.2.simd128_values();
        (SwitchIter::new(b_l, t_l, e_l),
         SwitchIter::new(b_h, t_h, e_h))
    }

    fn split(self, round_up: bool) -> (Self, Self) {
        let (b1, b2) = self.0.split(round_up);
        let (t1, t2) = self.1.split(round_up);
        let (e1, e2) = self.2.split(round_up);

        (Switch(b1, t1, e1), Switch(b2, t2, e2))
    }

    fn rev(self) -> Self::Rev {
        Switch(self.0.rev(), self.1.rev(), self.2.rev())
    }

    fn split_at(self, n: usize) -> (Self, Self) {
        let (b1, b2) = self.0.split_at(n);
        let (t1, t2) = self.1.split_at(n);
        let (e1, e2) = self.2.split_at(n);

        (Switch(b1, t1, e1), Switch(b2, t2, e2))
    }
}
