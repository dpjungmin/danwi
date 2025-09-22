use super::Unit;
use crate::{
    dimension::Dimensions,
    quantity::Quantity,
    scalar::{F32Scalar, F64Scalar},
};
use core::ops::Mul;

impl<D: Dimensions> Mul<Unit<D>> for f32 {
    type Output = Quantity<F32Scalar, D>;

    fn mul(self, unit: Unit<D>) -> Self::Output {
        Quantity::with_unit(F32Scalar::new(self), unit)
    }
}

impl<D: Dimensions> Mul<Unit<D>> for f64 {
    type Output = Quantity<F64Scalar, D>;

    fn mul(self, unit: Unit<D>) -> Self::Output {
        Quantity::with_unit(F64Scalar::new(self), unit)
    }
}

impl<D: Dimensions> Mul<Unit<D>> for F32Scalar {
    type Output = Quantity<F32Scalar, D>;

    fn mul(self, rhs: Unit<D>) -> Self::Output {
        Quantity::with_unit(self, rhs)
    }
}

impl<D: Dimensions> Mul<Unit<D>> for F64Scalar {
    type Output = Quantity<F64Scalar, D>;

    fn mul(self, rhs: Unit<D>) -> Self::Output {
        Quantity::with_unit(self, rhs)
    }
}
