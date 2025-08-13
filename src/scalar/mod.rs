mod float;

use crate::sealed::Sealed;
use core::fmt::Debug;

pub use float::{F32Scalar, F64Scalar};

pub trait Scalar: Clone + Debug + PartialEq + Sized + Sealed {
    type Value: Clone + Debug + PartialEq;

    fn get(&self) -> Self::Value;
    fn scale_by_power_of_10(&self, exponent: i8) -> Self;

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn neg(&self) -> Self;
}
