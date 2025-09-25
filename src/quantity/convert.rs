use super::Quantity;
use crate::{
    dimension::Dimensions,
    scalar::{F32Scalar, F64Scalar, Scalar},
};

// Quantity from Scalar
impl<S: Scalar, D: Dimensions> From<S> for Quantity<S, D> {
    fn from(value: S) -> Self {
        Self::new(value)
    }
}

#[cfg(feature = "f32")]
impl<D: Dimensions> From<f32> for Quantity<F32Scalar, D> {
    fn from(value: f32) -> Self {
        Self::new(F32Scalar::new(value))
    }
}

#[cfg(feature = "f64")]
impl<D: Dimensions> From<f64> for Quantity<F64Scalar, D> {
    fn from(value: f64) -> Self {
        Self::new(F64Scalar::new(value))
    }
}

#[cfg(feature = "f32")]
impl<D: Dimensions> Quantity<F32Scalar, D> {
    #[inline]
    pub const fn from_f32_const(value: f32) -> Self {
        Self::new(F32Scalar(value))
    }
}

#[cfg(feature = "f64")]
impl<D: Dimensions> Quantity<F64Scalar, D> {
    #[inline]
    pub const fn from_f64_const(value: f64) -> Self {
        Self::new(F64Scalar(value))
    }
}
