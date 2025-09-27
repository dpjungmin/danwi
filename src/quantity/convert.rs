use super::Quantity;
use crate::{dimension::Dimensions, scalar::Scalar};

impl<S: Scalar, D: Dimensions> From<S> for Quantity<S, D> {
    fn from(value: S) -> Self {
        Self::new(value)
    }
}

#[cfg(feature = "f32")]
impl<D: Dimensions> Quantity<f32, D> {
    #[inline]
    pub const fn from_f32_const(value: f32) -> Self {
        Self::new(value)
    }
}

#[cfg(feature = "f64")]
impl<D: Dimensions> Quantity<f64, D> {
    #[inline]
    pub const fn from_f64_const(value: f64) -> Self {
        Self::new(value)
    }
}
