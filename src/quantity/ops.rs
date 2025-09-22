use super::Quantity;
use crate::{
    dimension::{CanDivideBy, CanMultiplyWith, CanReciprocate, Dimensions},
    scalar::{F32Scalar, F64Scalar, Scalar},
};
use core::ops::{Add, Div, Mul, Neg, Sub};

// Quantity + Quantity
impl<S, D> Add<Quantity<S, D>> for Quantity<S, D>
where
    S: Scalar,
    D: Dimensions,
{
    type Output = Quantity<S, D>;

    fn add(self, rhs: Quantity<S, D>) -> Self::Output {
        let lhs_base = self.value.scale_by_power_of_10(self.unit.prefix);
        let rhs_base = rhs.value.scale_by_power_of_10(rhs.unit.prefix);
        Quantity::new(lhs_base.add(&rhs_base))
    }
}

// Quantity - Quantity
impl<S, D> Sub<Quantity<S, D>> for Quantity<S, D>
where
    S: Scalar,
    D: Dimensions,
{
    type Output = Quantity<S, D>;

    fn sub(self, rhs: Quantity<S, D>) -> Self::Output {
        let lhs_base = self.value.scale_by_power_of_10(self.unit.prefix);
        let rhs_base = rhs.value.scale_by_power_of_10(rhs.unit.prefix);
        Quantity::new(lhs_base.sub(&rhs_base))
    }
}

// Quantity * Quantity
impl<S, D1, D2> Mul<Quantity<S, D2>> for Quantity<S, D1>
where
    S: Scalar,
    D1: CanMultiplyWith<D2>,
    D2: Dimensions,
{
    type Output = Quantity<S, <D1 as CanMultiplyWith<D2>>::Output>;

    fn mul(self, rhs: Quantity<S, D2>) -> Self::Output {
        let lhs_base = self.value.scale_by_power_of_10(self.unit.prefix);
        let rhs_base = rhs.value.scale_by_power_of_10(rhs.unit.prefix);
        Quantity::new(lhs_base.mul(&rhs_base))
    }
}

// Quantity / Quantity
impl<S, D1, D2> Div<Quantity<S, D2>> for Quantity<S, D1>
where
    S: Scalar,
    D1: CanDivideBy<D2>,
    D2: Dimensions,
{
    type Output = Quantity<S, <D1 as CanDivideBy<D2>>::Output>;

    fn div(self, rhs: Quantity<S, D2>) -> Self::Output {
        let lhs_base = self.value.scale_by_power_of_10(self.unit.prefix);
        let rhs_base = rhs.value.scale_by_power_of_10(rhs.unit.prefix);
        Quantity::new(lhs_base.div(&rhs_base))
    }
}

// Quantity * f32
impl<D: Dimensions> Mul<f32> for Quantity<F32Scalar, D> {
    type Output = Quantity<F32Scalar, D>;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::with_unit(F32Scalar::new(self.value.get() * rhs), self.unit)
    }
}

// Quantity * f64
impl<D: Dimensions> Mul<f64> for Quantity<F64Scalar, D> {
    type Output = Quantity<F64Scalar, D>;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::with_unit(F64Scalar::new(self.value.get() * rhs), self.unit)
    }
}

// f32 * Quantity
impl<D: Dimensions> Mul<Quantity<F32Scalar, D>> for f32 {
    type Output = Quantity<F32Scalar, D>;

    #[inline]
    fn mul(self, rhs: Quantity<F32Scalar, D>) -> Self::Output {
        Quantity::with_unit(F32Scalar::new(self * rhs.value.get()), rhs.unit)
    }
}

// f64 * Quantity
impl<D: Dimensions> Mul<Quantity<F64Scalar, D>> for f64 {
    type Output = Quantity<F64Scalar, D>;

    #[inline]
    fn mul(self, rhs: Quantity<F64Scalar, D>) -> Self::Output {
        Quantity::with_unit(F64Scalar::new(self * rhs.value.get()), rhs.unit)
    }
}

// Quantity / f32
impl<D: Dimensions> Div<f32> for Quantity<F32Scalar, D> {
    type Output = Quantity<F32Scalar, D>;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::with_unit(F32Scalar::new(self.value.get() / rhs), self.unit)
    }
}

// Quantity / f64
impl<D: Dimensions> Div<f64> for Quantity<F64Scalar, D> {
    type Output = Quantity<F64Scalar, D>;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self::with_unit(F64Scalar::new(self.value.get() / rhs), self.unit)
    }
}

// f32 / Quantity
impl<D> Div<Quantity<F32Scalar, D>> for f32
where
    D: CanReciprocate,
{
    type Output = Quantity<F32Scalar, <D as CanReciprocate>::Output>;

    #[inline]
    fn div(self, rhs: Quantity<F32Scalar, D>) -> Self::Output {
        let rhs_base = rhs.value.scale_by_power_of_10(rhs.unit.prefix);
        Quantity::new(F32Scalar::new(self).div(&rhs_base))
    }
}

// f64 / Quantity
impl<D> Div<Quantity<F64Scalar, D>> for f64
where
    D: CanReciprocate,
{
    type Output = Quantity<F64Scalar, <D as CanReciprocate>::Output>;

    #[inline]
    fn div(self, rhs: Quantity<F64Scalar, D>) -> Self::Output {
        let rhs_base = rhs.value.scale_by_power_of_10(rhs.unit.prefix);
        Quantity::new(F64Scalar::new(self).div(&rhs_base))
    }
}

/// Negation of a quantity.
///
/// # Examples
///
/// ```
/// # use danwi::prelude::*;
///
/// let force = 10.0_f64 * N;
/// assert_eq!(force, 10.0.N());
/// assert_eq!(-force, -10.0.N());
///
/// let mass = 42.0_f64 * kg;
/// assert_eq!(-(-mass), 42.0.kg());
/// ```
impl<S, D> Neg for Quantity<S, D>
where
    S: Scalar,
    D: Dimensions,
{
    type Output = Quantity<S, D>;

    fn neg(self) -> Self::Output {
        Self::with_unit(self.value.neg(), self.unit)
    }
}
