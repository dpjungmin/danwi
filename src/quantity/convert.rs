use super::Quantity;
use crate::{Unit, dimension::Dimensions, scalar::Scalar};

impl<S: Scalar, D: Dimensions> From<S> for Quantity<S, D> {
    fn from(value: S) -> Self {
        Self::new(value)
    }
}

#[cfg(feature = "f32")]
impl<D: Dimensions> Quantity<f32, D> {
    #[inline]
    pub const fn from_f32(value: f32) -> Self {
        Self::new(value)
    }

    #[inline]
    pub const fn from_f32_with_unit(value: f32, unit: Unit<D>) -> Self {
        Self { value, unit }
    }
}

#[cfg(feature = "f64")]
impl<D: Dimensions> Quantity<f64, D> {
    #[inline]
    pub const fn from_f64(value: f64) -> Self {
        Self::new(value)
    }

    #[inline]
    pub const fn from_f64_with_unit(value: f64, unit: Unit<D>) -> Self {
        Self { value, unit }
    }
}
