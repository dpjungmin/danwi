use super::Quantity;
use crate::{
    dimension::{CanDivideBy, CanMultiplyWith, CanReciprocate, Dimensions},
    scalar::Scalar,
};
use core::ops::{Add, Div, Mul, Neg, Sub};

// Quantity + Quantity
impl<S: Scalar, D: Dimensions> Add<Quantity<S, D>> for Quantity<S, D> {
    type Output = Quantity<S, D>;

    fn add(self, rhs: Quantity<S, D>) -> Self::Output {
        let lhs_base = self.value.scale_by_power_of_10(self.unit.prefix);
        let rhs_base = rhs.value.scale_by_power_of_10(rhs.unit.prefix);
        Quantity::new(lhs_base + rhs_base)
    }
}

// Quantity - Quantity
impl<S: Scalar, D: Dimensions> Sub<Quantity<S, D>> for Quantity<S, D> {
    type Output = Quantity<S, D>;

    fn sub(self, rhs: Quantity<S, D>) -> Self::Output {
        let lhs_base = self.value.scale_by_power_of_10(self.unit.prefix);
        let rhs_base = rhs.value.scale_by_power_of_10(rhs.unit.prefix);
        Quantity::new(lhs_base - rhs_base)
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
        Quantity::new(lhs_base * rhs_base)
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
        Quantity::new(lhs_base / rhs_base)
    }
}

// Quantity * f32
#[cfg(feature = "f32")]
impl<D: Dimensions> Mul<f32> for Quantity<f32, D> {
    type Output = Quantity<f32, D>;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::with_unit(self.value * rhs, self.unit)
    }
}

// Quantity * f64
#[cfg(feature = "f64")]
impl<D: Dimensions> Mul<f64> for Quantity<f64, D> {
    type Output = Quantity<f64, D>;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::with_unit(self.value * rhs, self.unit)
    }
}

// f32 * Quantity
#[cfg(feature = "f32")]
impl<D: Dimensions> Mul<Quantity<f32, D>> for f32 {
    type Output = Quantity<f32, D>;

    #[inline]
    fn mul(self, rhs: Quantity<f32, D>) -> Self::Output {
        Quantity::with_unit(self * rhs.value, rhs.unit)
    }
}

// f64 * Quantity
#[cfg(feature = "f64")]
impl<D: Dimensions> Mul<Quantity<f64, D>> for f64 {
    type Output = Quantity<f64, D>;

    #[inline]
    fn mul(self, rhs: Quantity<f64, D>) -> Self::Output {
        Quantity::with_unit(self * rhs.value, rhs.unit)
    }
}

// Quantity / f32
#[cfg(feature = "f32")]
impl<D: Dimensions> Div<f32> for Quantity<f32, D> {
    type Output = Quantity<f32, D>;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::with_unit(self.value / rhs, self.unit)
    }
}

// Quantity / f64
#[cfg(feature = "f64")]
impl<D: Dimensions> Div<f64> for Quantity<f64, D> {
    type Output = Quantity<f64, D>;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self::with_unit(self.value / rhs, self.unit)
    }
}

// f32 / Quantity
#[cfg(feature = "f32")]
impl<D> Div<Quantity<f32, D>> for f32
where
    D: CanReciprocate,
{
    type Output = Quantity<f32, <D as CanReciprocate>::Output>;

    #[inline]
    fn div(self, rhs: Quantity<f32, D>) -> Self::Output {
        let rhs_base = rhs.value.scale_by_power_of_10(rhs.unit.prefix);
        Quantity::new(self / rhs_base)
    }
}

// f64 / Quantity
#[cfg(feature = "f64")]
impl<D> Div<Quantity<f64, D>> for f64
where
    D: CanReciprocate,
{
    type Output = Quantity<f64, <D as CanReciprocate>::Output>;

    #[inline]
    fn div(self, rhs: Quantity<f64, D>) -> Self::Output {
        let rhs_base = rhs.value.scale_by_power_of_10(rhs.unit.prefix);
        Quantity::new(self / rhs_base)
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
        Self::with_unit(-self.value, self.unit)
    }
}
