use super::Unit;
use crate::{dimension::Dimensions, quantity::Quantity};
use core::ops::Mul;

#[cfg(feature = "f32")]
impl<D: Dimensions> Mul<Unit<D>> for f32 {
    type Output = Quantity<Self, D>;

    fn mul(self, unit: Unit<D>) -> Self::Output {
        Quantity::with_unit(self, unit)
    }
}

#[cfg(feature = "f64")]
impl<D: Dimensions> Mul<Unit<D>> for f64 {
    type Output = Quantity<Self, D>;

    fn mul(self, unit: Unit<D>) -> Self::Output {
        Quantity::with_unit(self, unit)
    }
}
