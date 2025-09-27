mod float;

use core::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Sub},
};

pub trait Scalar:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Copy
    + Clone
    + Debug
    + Display
    + PartialEq
    + PartialOrd
    + Default
    + Sized
{
    fn zero() -> Self;
    fn scale_by_power_of_10(&self, exponent: i8) -> Self;
}
