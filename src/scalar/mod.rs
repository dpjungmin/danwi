mod float;

use core::fmt::{Debug, Display};

#[cfg(feature = "f32")]
pub use float::F32Scalar;

#[cfg(feature = "f64")]
pub use float::F64Scalar;

pub trait Scalar: Clone + Debug + PartialEq + Sized {
    type Value: Clone + Debug + PartialEq + Display;

    fn get(&self) -> Self::Value;
    fn scale_by_power_of_10(&self, exponent: i8) -> Self;

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn neg(&self) -> Self;
}
