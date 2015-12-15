use std::{iter, mem, slice};
use simd::*;

pub trait SimdValue: Sized + Send {
    type SimdBool: SimdValue + From<bool>;
    type V128: SimdVector<Element = Self>;

    fn splat_128(self) -> Self::V128;
}

pub unsafe trait SimdVector: Sized + Send {
    type Element;
    type BoolV: SimdVector;
    type Elements: Iterator<Item = Self::Element>;
    type Reverse: SimdVector<Element = Self::Element>;

    fn len(&self) -> usize;

    fn reverse(self) -> Self::Reverse;

    fn elements(self) -> Self::Elements;
}

pub struct ElementsCopy<V> {
    v: V,
    idx: isize,
}
impl<V> Iterator for ElementsCopy<V>
    where V: SimdVector, V::Element: Copy,
{
    type Item = V::Element;

    fn next(&mut self) -> Option<V::Element> {
        // FIXME: this probably doesn't optimise so great
        let self_size = mem::size_of::<V>();
        let elem_size = mem::size_of::<V::Element>();
        assert!(self_size % elem_size == 0);
        let len = self_size / elem_size;
        if self.idx < len as isize {
            let ret = unsafe {*(&self.v as *const _ as *const V::Element).offset(self.idx)};
            self.idx += 1;
            Some(ret)
        } else {
            None
        }
    }
}


macro_rules! impls {
    ($($scalar: ty, $bool: ty, $v128: ident, $bool128: ident, $len: expr;)*) => {
        $(
            impl SimdValue for $scalar {
                type V128 = $v128;
                type SimdBool = $bool;

                fn splat_128(self) -> Self::V128 {
                    // $v128::splat(self)
                    unimplemented!()
                }
            }

            unsafe impl SimdVector for $v128 {
                type Element = $scalar;
                type BoolV = $bool128;
                type Elements = ElementsCopy<Self>;
                type Reverse = Self;

                fn len(&self) -> usize {
                    $len
                }

                fn reverse(self) -> Self {
                    let mut ret: Self = unsafe {mem::uninitialized()};

                    for i in 0..$len {
                        ret = ret.replace($len - 1 - i, self.extract(i))
                    }

                    ret
                }

                fn elements(self) -> ElementsCopy<Self> {
                    ElementsCopy {
                        v: self,
                        idx: 0,
                    }
                }
            }
            )*
    }
}

impls! {
    i8, bool8i, i8x16, bool8ix16, 16;
    i16, bool16i, i16x8, bool16ix8, 8;
    i32, bool32i, i32x4, bool32ix4, 4;
    u8, bool8i, u8x16, bool8ix16, 16;
    u16, bool16i, u16x8, bool16ix8, 8;
    u32, bool32i, u32x4, bool32ix4, 4;
    f32, bool32f, f32x4, bool32fx4, 4;

    bool8i, bool8i, bool8ix16, bool8ix16, 16;
    bool16i, bool16i, bool16ix8, bool16ix8, 8;
    bool32i, bool32i, bool32ix4, bool32ix4, 4;
    bool32f, bool32f, bool32fx4, bool32fx4, 4;
}

unsafe impl<'a, V: SimdVector> SimdVector for &'a mut V {
    type Element = &'a mut V::Element;
    type BoolV = V::BoolV;
    type Elements = slice::IterMut<'a, V::Element>;
    type Reverse = RevMutSimd<'a, V>;

    fn len(&self) -> usize {
        (**self).len()
    }

    fn reverse(self) -> Self::Reverse {
        RevMutSimd(self)
    }

    fn elements(self) -> Self::Elements {
        let slice = unsafe {
            slice::from_raw_parts_mut(self as *mut _ as *mut V::Element,
                                      self.len())
        };
        slice.iter_mut()
    }
}

pub struct RevMutSimd<'a, V: 'a>(&'a mut V);

unsafe impl<'a, V: SimdVector> SimdVector for RevMutSimd<'a, V> {
    type Element = &'a mut V::Element;
    type BoolV = V::BoolV;
    type Elements = iter::Rev<slice::IterMut<'a, V::Element>>;
    type Reverse = &'a mut V;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn reverse(self) -> Self::Reverse {
        self.0
    }

    fn elements(self) -> Self::Elements {
        self.0.elements().rev()
    }
}

unsafe impl<V1: SimdVector, V2: SimdVector> SimdVector for (V1, V2) {
    type Element = (V1::Element, V2::Element);
    type BoolV = (V1::BoolV, V2::BoolV);
    type Elements = iter::Zip<V1::Elements, V2::Elements>;
    type Reverse = (V1::Reverse, V2::Reverse);

    fn len(&self) -> usize {
        unimplemented!()
    }

    fn reverse(self) -> Self::Reverse {
        (self.0.reverse(), self.1.reverse())
    }

    fn elements(self) -> Self::Elements {
        self.0.elements().zip(self.1.elements())
    }
}

pub trait SimdEq {
    type Output: Send;

    fn simd_eq(&self, other: &Self) -> Self::Output;
    fn simd_ne(&self, other: &Self) -> Self::Output;
}

pub trait SimdOrd: SimdEq {
    fn simd_lt(&self, other: &Self) -> Self::Output;
    fn simd_le(&self, other: &Self) -> Self::Output;
    fn simd_gt(&self, other: &Self) -> Self::Output {
        other.simd_lt(self)
    }
    fn simd_ge(&self, other: &Self) -> Self::Output {
        other.simd_le(self)
    }
}

impl<E: PartialEq + SimdValue> SimdEq for E {
    type Output = E::SimdBool;

    fn simd_eq(&self, other: &Self) -> Self::Output {
        (*self == *other).into()
    }

    fn simd_ne(&self, other: &Self) -> Self::Output {
        (*self != *other).into()
    }
}

impl<E: PartialOrd + SimdValue> SimdOrd for E {
    fn simd_lt(&self, other: &Self) -> Self::Output {
        (*self <*other).into()
    }

    fn simd_le(&self, other: &Self) -> Self::Output {
        (*self <= *other).into()
    }
}

macro_rules! vector_cmp {
    ($($name: ident;)*) => {
        $(impl SimdEq for $name {
            type Output = <$name as SimdVector>::BoolV;

            fn simd_eq(&self, other: &Self) -> Self::Output {
                (*self).eq(*other)
            }

            fn simd_ne(&self, other: &Self) -> Self::Output {
                (*self).ne(*other)
            }
        }

          impl SimdOrd for $name {
            fn simd_lt(&self, other: &Self) -> Self::Output {
                (*self).lt(*other)
            }

            fn simd_le(&self, other: &Self) -> Self::Output {
                (*self).le(*other)
            }
        }
          )*
    }
}

vector_cmp! {
    i8x16; i16x8; i32x4;
    u8x16; u16x8; u32x4;
    f32x4;

    //bool8ix16; bool16ix8; bool32ix4;
    //bool32fx4;
}
