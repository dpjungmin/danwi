use crate::{dimension::Dimensions, scalar::Scalar, unit::Unit};

mod cmp;
mod convert;
mod fmt;
mod ops;

#[derive(Debug, Clone, Copy)]
pub struct Quantity<S, D>
where
    S: Scalar,
    D: Dimensions,
{
    pub(crate) value: S,
    pub(crate) unit: Unit<D>,
}

impl<S, D> Quantity<S, D>
where
    S: Scalar,
    D: Dimensions,
{
    #[inline]
    pub const fn with_unit(value: S, unit: Unit<D>) -> Self {
        Self { value, unit }
    }

    #[inline]
    pub const fn new(value: S) -> Self {
        Self::with_unit(value, Unit::base())
    }

    #[inline]
    pub fn value(&self) -> S {
        self.value
    }

    #[inline]
    pub fn to(&self, target_unit: Unit<D>) -> Self {
        let prefix_diff = self.unit.prefix - target_unit.prefix;
        let scaled_value = self.value.scale_by_power_of_10(prefix_diff);
        Self::with_unit(scaled_value, target_unit)
    }
}

impl<D: Dimensions> Quantity<f32, D> {
    #[inline]
    pub const fn from_f32(value: f32, unit: Unit<D>) -> Self {
        Self { value, unit }
    }
}

impl<D: Dimensions> Quantity<f64, D> {
    #[inline]
    pub const fn from_f64(value: f64, unit: Unit<D>) -> Self {
        Self { value, unit }
    }
}
