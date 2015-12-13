use std::ops;
use num::Zero;
use {Expression, Length};

pub trait Reduce<T: Send>: Send {
    type Output: Send;
    type Scalar: ReduceScalar<Self::Output>;

    fn expected_length(&self) -> Length;
    fn split(self) -> (Self, Self, Self::Scalar);
    fn reduce<E>(self, E) -> Self::Output
        where E: Expression<Element = T>;
}

pub trait ReduceScalar<X> {
    fn combine(self, X, X) -> X;
}

impl ReduceScalar<()> for () {
    fn combine(self, _: (), _: ()) -> () {
        ()
    }
}

pub struct Write<E>(pub E);
impl<'a, E, T> Reduce<T> for Write<E>
    where T: 'a + Send, E: Expression<Element = &'a mut T>
{
    type Output = ();
    type Scalar = ();

    fn expected_length(&self) -> Length {
        self.0.length()
    }

    fn split(self) -> (Self, Self, ()) {
        let (lo, hi) = self.0.split(false);
        (Write(lo), Write(hi), ())
    }

    fn reduce<E_>(self, e: E_) -> Self::Output
        where E_: Expression<Element = T>
    {
        for (o, i) in self.0.values().zip(e.values()) {
            *o = i;
        }
    }
}

macro_rules! repeated {
    ($name: ident) => {
        fn expected_length(&self) -> Length { Length::Infinite }
        fn split(self) -> ($name, $name, $name) { ($name, $name, $name) }
    }
}

pub struct Sum;
impl<T> Reduce<T> for Sum
    where T: Send + Zero + ops::Add<T, Output = T>
{
    type Output = T;
    type Scalar = Sum;

    repeated!(Sum);

    fn reduce<E>(self, e: E) -> Self::Output
        where E: Expression<Element = T>
    {
        e.values().fold(Zero::zero(), |x, y| x + y)
    }
}
impl<T> ReduceScalar<T> for Sum
    where T: ops::Add<T, Output = T>
{
    fn combine(self, a: T, b: T) -> T {
        a + b
    }
}

macro_rules! minmax {
    ($name: ident, $method: ident: $($f: ty),*) => {
        pub struct $name;
        $(
            impl Reduce<$f> for $name {
                type Output = Option<$f>;
                type Scalar = $name;

                repeated!($name);

                fn reduce<E>(self, e: E) -> Self::Output
                    where E: Expression<Element = $f>,
                {
                    let mut vals = e.values();
                    vals.next().map(|first| vals.fold(first, |x, y| x.$method(y)))
                }
            }
            impl ReduceScalar<Option<$f>> for $name {
                fn combine(self, a: Option<$f>, b: Option<$f>) -> Option<$f> {
                    match (a, b) {
                        (Some(x), Some(y)) => Some(x.$method(y)),
                        (x, None) => x,
                        (None, y) => y,
                    }
                }
            }
            )*
    }
}

minmax!(Min, min: f32, f64);
minmax!(Max, max: f32, f64);
