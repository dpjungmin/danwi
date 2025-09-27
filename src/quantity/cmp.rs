use super::Quantity;
use crate::{dimension::*, scalar::Scalar};
use core::cmp::Ordering;

impl<S: Scalar, D: Dimensions> PartialEq for Quantity<S, D> {
    fn eq(&self, other: &Self) -> bool {
        let lhs_base = self.value.scale_by_power_of_10(self.unit.prefix);
        let rhs_base = other.value.scale_by_power_of_10(other.unit.prefix);
        lhs_base == rhs_base
    }
}

#[cfg(feature = "f32")]
impl PartialEq<f32> for Quantity<f32, Dimensionless> {
    fn eq(&self, other: &f32) -> bool {
        self.value == *other
    }
}

#[cfg(feature = "f64")]
impl PartialEq<f64> for Quantity<f64, Dimensionless> {
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}

impl<S: Scalar> PartialOrd for Quantity<S, Dimensionless> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs_base = self.value.scale_by_power_of_10(self.unit.prefix);
        let rhs_base = other.value.scale_by_power_of_10(other.unit.prefix);
        lhs_base.partial_cmp(&rhs_base)
    }
}
