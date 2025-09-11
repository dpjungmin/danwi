use crate::{
    scalar::{F32Scalar, F64Scalar, Scalar},
    units::{BaseUnit, DimensionEq, Divide, Multiply, SameDimension, Unit},
};
use core::{
    marker::PhantomData,
    ops::{Div, Mul},
};

#[derive(Debug, Clone, Copy)]
pub struct Quantity<S: Scalar, U: Unit> {
    pub(crate) value: S,
    _phantom: PhantomData<U>,
}

impl<S: Scalar, U: Unit> Quantity<S, U> {
    pub const fn new(value: S) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    pub fn value(&self) -> S::Value {
        self.value.get()
    }

    pub fn to<T: Unit>(self) -> Quantity<S, T>
    where
        DimensionEq<U, T>: SameDimension<U, T>,
    {
        let prefix_diff = U::PREFIX - T::PREFIX;
        let scaled_value = self.value.scale_by_power_of_10(prefix_diff);
        Quantity::new(scaled_value)
    }
}

impl<U: Unit> From<f64> for Quantity<F64Scalar, U> {
    fn from(value: f64) -> Self {
        Self::new(F64Scalar::new(value))
    }
}

impl<U: Unit> From<f32> for Quantity<F32Scalar, U> {
    fn from(value: f32) -> Self {
        Self::new(F32Scalar::new(value))
    }
}

type BaseOf<U> = <U as BaseUnit>::Base;
type ProductOf<U1, U2> = <U1 as Multiply<U2>>::Output;
type QuotientOf<U1, U2> = <U1 as Divide<U2>>::Output;

impl<S, U1, U2> Mul<Quantity<S, U2>> for Quantity<S, U1>
where
    S: Scalar,
    U1: Unit + BaseUnit,
    U2: Unit + BaseUnit,
    BaseOf<U1>: Multiply<BaseOf<U2>>,
    DimensionEq<U1, BaseOf<U1>>: SameDimension<U1, BaseOf<U1>>,
    DimensionEq<U2, BaseOf<U2>>: SameDimension<U2, BaseOf<U2>>,
{
    type Output = Quantity<S, ProductOf<BaseOf<U1>, BaseOf<U2>>>;

    fn mul(self, rhs: Quantity<S, U2>) -> Self::Output {
        let base_lhs = self.to::<<U1 as BaseUnit>::Base>();
        let base_rhs = rhs.to::<<U2 as BaseUnit>::Base>();
        Quantity::new(base_lhs.value.mul(&base_rhs.value))
    }
}

impl<S, U1, U2> Div<Quantity<S, U2>> for Quantity<S, U1>
where
    S: Scalar,
    U1: Unit + BaseUnit,
    U2: Unit + BaseUnit,
    BaseOf<U1>: Divide<BaseOf<U2>>,
    DimensionEq<U1, BaseOf<U1>>: SameDimension<U1, BaseOf<U1>>,
    DimensionEq<U2, BaseOf<U2>>: SameDimension<U2, BaseOf<U2>>,
{
    type Output = Quantity<S, QuotientOf<BaseOf<U1>, BaseOf<U2>>>;

    fn div(self, rhs: Quantity<S, U2>) -> Self::Output {
        let base_lhs = self.to::<<U1 as BaseUnit>::Base>();
        let base_rhs = rhs.to::<<U2 as BaseUnit>::Base>();
        Quantity::new(base_lhs.value.div(&base_rhs.value))
    }
}

impl<S, U1, U2> PartialEq<Quantity<S, U2>> for Quantity<S, U1>
where
    S: Scalar + PartialEq + Copy,
    U1: Unit + BaseUnit,
    U2: Unit + BaseUnit,
    DimensionEq<BaseOf<U1>, BaseOf<U2>>: SameDimension<BaseOf<U1>, BaseOf<U2>>,
    DimensionEq<U1, BaseOf<U1>>: SameDimension<U1, BaseOf<U1>>,
    DimensionEq<U2, BaseOf<U2>>: SameDimension<U2, BaseOf<U2>>,
{
    fn eq(&self, other: &Quantity<S, U2>) -> bool {
        let base_lhs = self.to::<BaseOf<U1>>();
        let base_rhs = other.to::<BaseOf<U2>>();
        base_lhs.value == base_rhs.value
    }
}

// Available when either testing or when the "approx" feature is enabled
#[cfg(any(test, feature = "approx"))]
mod approx_impl {
    use super::*;
    use approx::{AbsDiffEq, RelativeEq};

    impl<U1, U2> AbsDiffEq<Quantity<F64Scalar, U2>> for Quantity<F64Scalar, U1>
    where
        U1: Unit + BaseUnit,
        U2: Unit + BaseUnit,
        BaseOf<U1>: Unit,
        BaseOf<U2>: Unit,
        DimensionEq<U1, U2>: SameDimension<U1, U2>,
        DimensionEq<U1, BaseOf<U1>>: SameDimension<U1, BaseOf<U1>>,
        DimensionEq<U2, BaseOf<U2>>: SameDimension<U2, BaseOf<U2>>,
        DimensionEq<BaseOf<U1>, BaseOf<U2>>: SameDimension<BaseOf<U1>, BaseOf<U2>>,
    {
        type Epsilon = f64;

        fn default_epsilon() -> Self::Epsilon {
            f64::EPSILON
        }

        fn abs_diff_eq(&self, other: &Quantity<F64Scalar, U2>, epsilon: Self::Epsilon) -> bool {
            let self_base = self.to::<BaseOf<U1>>();
            let other_base = other.to::<BaseOf<U2>>();
            (self_base.value.get() - other_base.value.get()).abs() <= epsilon
        }
    }

    impl<U1, U2> RelativeEq<Quantity<F64Scalar, U2>> for Quantity<F64Scalar, U1>
    where
        U1: Unit + BaseUnit,
        U2: Unit + BaseUnit,
        BaseOf<U1>: Unit,
        BaseOf<U2>: Unit,
        DimensionEq<U1, U2>: SameDimension<U1, U2>,
        DimensionEq<U1, BaseOf<U1>>: SameDimension<U1, BaseOf<U1>>,
        DimensionEq<U2, BaseOf<U2>>: SameDimension<U2, BaseOf<U2>>,
        DimensionEq<BaseOf<U1>, BaseOf<U2>>: SameDimension<BaseOf<U1>, BaseOf<U2>>,
    {
        fn default_max_relative() -> Self::Epsilon {
            f64::EPSILON
        }

        fn relative_eq(
            &self,
            other: &Quantity<F64Scalar, U2>,
            epsilon: Self::Epsilon,
            max_relative: Self::Epsilon,
        ) -> bool {
            let self_base = self.to::<BaseOf<U1>>();
            let other_base = other.to::<BaseOf<U2>>();

            let diff = (self_base.value.get() - other_base.value.get()).abs();
            if diff <= epsilon {
                return true;
            }

            let abs_self = self_base.value.get().abs();
            let abs_other = other_base.value.get().abs();
            let largest = abs_self.max(abs_other);

            diff <= largest * max_relative
        }
    }

    impl<U1, U2> AbsDiffEq<Quantity<F32Scalar, U2>> for Quantity<F32Scalar, U1>
    where
        U1: Unit + BaseUnit,
        U2: Unit + BaseUnit,
        BaseOf<U1>: Unit,
        BaseOf<U2>: Unit,
        DimensionEq<U1, U2>: SameDimension<U1, U2>,
        DimensionEq<U1, BaseOf<U1>>: SameDimension<U1, BaseOf<U1>>,
        DimensionEq<U2, BaseOf<U2>>: SameDimension<U2, BaseOf<U2>>,
        DimensionEq<BaseOf<U1>, BaseOf<U2>>: SameDimension<BaseOf<U1>, BaseOf<U2>>,
    {
        type Epsilon = f32;

        fn default_epsilon() -> Self::Epsilon {
            f32::EPSILON
        }

        fn abs_diff_eq(&self, other: &Quantity<F32Scalar, U2>, epsilon: Self::Epsilon) -> bool {
            let self_base = self.to::<BaseOf<U1>>();
            let other_base = other.to::<BaseOf<U2>>();
            (self_base.value.get() - other_base.value.get()).abs() <= epsilon
        }
    }

    impl<U1, U2> RelativeEq<Quantity<F32Scalar, U2>> for Quantity<F32Scalar, U1>
    where
        U1: Unit + BaseUnit,
        U2: Unit + BaseUnit,
        BaseOf<U1>: Unit,
        BaseOf<U2>: Unit,
        DimensionEq<U1, U2>: SameDimension<U1, U2>,
        DimensionEq<U1, BaseOf<U1>>: SameDimension<U1, BaseOf<U1>>,
        DimensionEq<U2, BaseOf<U2>>: SameDimension<U2, BaseOf<U2>>,
        DimensionEq<BaseOf<U1>, BaseOf<U2>>: SameDimension<BaseOf<U1>, BaseOf<U2>>,
    {
        fn default_max_relative() -> Self::Epsilon {
            f32::EPSILON
        }

        fn relative_eq(
            &self,
            other: &Quantity<F32Scalar, U2>,
            epsilon: Self::Epsilon,
            max_relative: Self::Epsilon,
        ) -> bool {
            let self_base = self.to::<BaseOf<U1>>();
            let other_base = other.to::<BaseOf<U2>>();

            let diff = (self_base.value.get() - other_base.value.get()).abs();
            if diff <= epsilon {
                return true;
            }

            let abs_self = self_base.value.get().abs();
            let abs_other = other_base.value.get().abs();
            let largest = abs_self.max(abs_other);

            diff <= largest * max_relative
        }
    }
}
