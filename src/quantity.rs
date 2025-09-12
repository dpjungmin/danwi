use crate::{
    dimension::{CanDivideBy, CanMultiplyWith, CanReciprocate, Dimensionless, Dimensions},
    scalar::{F32Scalar, F64Scalar, Scalar},
    unit::{Unit, UnitKind},
};
use core::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

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
    pub(crate) fn with_unit(value: S, unit: Unit<D>) -> Self {
        Self { value, unit }
    }

    #[inline]
    pub fn new(value: S) -> Self {
        Self::with_unit(value, Unit::base())
    }

    #[inline]
    pub fn value(&self) -> S::Value {
        self.value.get()
    }

    #[inline]
    pub fn to<U: UnitKind<Dimension = D>>(&self) -> Self {
        let prefix_diff = self.unit.prefix - U::PREFIX;
        let scaled_value = self.value.scale_by_power_of_10(prefix_diff);
        Self::with_unit(scaled_value, Unit::with_prefix(U::PREFIX))
    }
}

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

// Quantity from Scalar
impl<S: Scalar, D: Dimensions> From<S> for Quantity<S, D> {
    fn from(value: S) -> Self {
        Self::new(value)
    }
}

// Quantity from f32
impl<D: Dimensions> From<f32> for Quantity<F32Scalar, D> {
    fn from(value: f32) -> Self {
        Self::new(F32Scalar::new(value))
    }
}

// Quantity from f64
impl<D: Dimensions> From<f64> for Quantity<F64Scalar, D> {
    fn from(value: f64) -> Self {
        Self::new(F64Scalar::new(value))
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

impl<S, D> fmt::Display for Quantity<S, D>
where
    S: Scalar,
    S::Value: fmt::Display,
    D: Dimensions,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
