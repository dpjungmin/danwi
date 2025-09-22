use super::Quantity;
use crate::{
    dimension::{Dimensionless, Dimensions},
    scalar::{F32Scalar, F64Scalar, Scalar},
};
use core::cmp::Ordering;

impl<S, D> PartialEq for Quantity<S, D>
where
    S: Scalar + PartialEq,
    D: Dimensions,
{
    fn eq(&self, other: &Self) -> bool {
        let lhs_base = self.value.scale_by_power_of_10(self.unit.prefix);
        let rhs_base = other.value.scale_by_power_of_10(other.unit.prefix);
        lhs_base == rhs_base
    }
}

impl PartialEq<f32> for Quantity<F32Scalar, Dimensionless> {
    fn eq(&self, other: &f32) -> bool {
        self.value.get() == *other
    }
}

impl PartialEq<f64> for Quantity<F64Scalar, Dimensionless> {
    fn eq(&self, other: &f64) -> bool {
        self.value.get() == *other
    }
}

impl<S, D> PartialOrd for Quantity<S, D>
where
    S: Scalar + PartialOrd,
    D: Dimensions,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs_base = self.value.scale_by_power_of_10(self.unit.prefix);
        let rhs_base = other.value.scale_by_power_of_10(other.unit.prefix);
        lhs_base.partial_cmp(&rhs_base)
    }
}
