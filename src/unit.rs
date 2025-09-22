#![allow(non_upper_case_globals)]

use crate::{
    dimension::{Dimensions, base, derived},
    prefix,
    quantity::Quantity,
    scalar::{F32Scalar, F64Scalar},
};
use core::{marker::PhantomData, ops::Mul};

#[derive(Debug, Clone, Copy)]
pub struct Unit<D: Dimensions> {
    pub(crate) prefix: i8,
    _phantom: PhantomData<D>,
}

impl<D: Dimensions> Unit<D> {
    pub const fn with_prefix(prefix: i8) -> Self {
        Self {
            prefix,
            _phantom: PhantomData,
        }
    }

    pub const fn base() -> Self {
        Self::with_prefix(0)
    }
}

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

macro_rules! define_units {
    ($($name:ident ($symbol:ident): $dimension:ty),* $(,)?) => {
        $(
            paste::paste! {
                pub struct [<$name:camel>];

                // constants
                pub const [<Q $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::QUETTA);
                pub const [<R $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::RONNA);
                pub const [<Y $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::YOTTA);
                pub const [<Z $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::ZETTA);
                pub const [<E $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::EXA);
                pub const [<P $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::PETA);
                pub const [<T $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::TERA);
                pub const [<G $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::GIGA);
                pub const [<M $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::MEGA);
                pub const [<k $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::KILO);
                pub const [<h $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::HECTO);
                pub const [<da $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::DECA);
                pub const $symbol: Unit<$dimension> = Unit::base();
                pub const [<d $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::DECI);
                pub const [<c $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::CENTI);
                pub const [<m $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::MILLI);
                pub const [<u $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::MICRO);
                pub const [<n $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::NANO);
                pub const [<p $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::PICO);
                pub const [<f $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::FEMTO);
                // keyword collision for atto second (as)
                pub const [<atto_ $name>]: Unit<$dimension> = Unit::with_prefix(prefix::ATTO);
                pub const [<z $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::ZEPTO);
                pub const [<y $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::YOCTO);
                pub const [<r $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::RONTO);
                pub const [<q $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::QUECTO);

                // types aliases
                pub type [<F32 $name:camel>] = Quantity<F32Scalar, $dimension>;
                pub type [<F64 $name:camel>] = Quantity<F64Scalar, $dimension>;
            }
        )*

        pub mod ext {
            #![allow(non_snake_case)]

            use super::*;

            paste::paste! {
                pub trait F32QuantityExt {
                    $(
                        fn [<Q $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<R $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<Y $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<Z $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<E $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<P $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<T $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<G $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<M $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<k $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<h $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<da $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn $symbol(self) -> Quantity<F32Scalar, $dimension>;
                        fn [<d $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<c $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<m $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<u $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<n $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<p $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<f $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        // keyword collision for atto second (as)
                        fn [<atto_ $name>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<z $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<y $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<r $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<q $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                    )*
                }

                pub trait F64QuantityExt {
                    $(
                        fn [<Q $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<R $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<Y $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<Z $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<E $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<P $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<T $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<G $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<M $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<k $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<h $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<da $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn $symbol(self) -> Quantity<F64Scalar, $dimension>;
                        fn [<d $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<c $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<m $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<u $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<n $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<p $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<f $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        // keyword collision for atto second (as)
                        fn [<atto_ $name>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<z $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<y $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<r $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<q $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                    )*
                }
            }

            paste::paste! {
                impl F32QuantityExt for f32 {
                    $(
                        fn [<Q $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<Q $symbol>]
                        }
                        fn [<R $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<R $symbol>]
                        }
                        fn [<Y $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<Y $symbol>]
                        }
                        fn [<Z $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<Z $symbol>]
                        }
                        fn [<E $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<E $symbol>]
                        }
                        fn [<P $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<P $symbol>]
                        }
                        fn [<T $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<T $symbol>]
                        }
                        fn [<G $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<G $symbol>]
                        }
                        fn [<M $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<M $symbol>]
                        }
                        fn [<k $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<k $symbol>]
                        }
                        fn [<h $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<h $symbol>]
                        }
                        fn [<da $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<da $symbol>]
                        }
                        fn $symbol(self) -> Quantity<F32Scalar, $dimension> {
                            self * $symbol
                        }
                        fn [<d $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<d $symbol>]
                        }
                        fn [<c $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<c $symbol>]
                        }
                        fn [<m $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<m $symbol>]
                        }
                        fn [<u $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<u $symbol>]
                        }
                        fn [<n $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<n $symbol>]
                        }
                        fn [<p $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<p $symbol>]
                        }
                        fn [<f $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<f $symbol>]
                        }
                        fn [<atto_ $name>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<atto_ $name>]
                        }
                        fn [<z $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<z $symbol>]
                        }
                        fn [<y $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<y $symbol>]
                        }
                        fn [<r $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<r $symbol>]
                        }
                        fn [<q $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<q $symbol>]
                        }
                    )*
                }

                impl F64QuantityExt for f64 {
                    $(
                        fn [<Q $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<Q $symbol>]
                        }
                        fn [<R $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<R $symbol>]
                        }
                        fn [<Y $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<Y $symbol>]
                        }
                        fn [<Z $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<Z $symbol>]
                        }
                        fn [<E $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<E $symbol>]
                        }
                        fn [<P $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<P $symbol>]
                        }
                        fn [<T $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<T $symbol>]
                        }
                        fn [<G $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<G $symbol>]
                        }
                        fn [<M $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<M $symbol>]
                        }
                        fn [<k $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<k $symbol>]
                        }
                        fn [<h $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<h $symbol>]
                        }
                        fn [<da $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<da $symbol>]
                        }
                        fn $symbol(self) -> Quantity<F64Scalar, $dimension> {
                            self * $symbol
                        }
                        fn [<d $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<d $symbol>]
                        }
                        fn [<c $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<c $symbol>]
                        }
                        fn [<m $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<m $symbol>]
                        }
                        fn [<u $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<u $symbol>]
                        }
                        fn [<n $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<n $symbol>]
                        }
                        fn [<p $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<p $symbol>]
                        }
                        fn [<f $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<f $symbol>]
                        }
                        fn [<atto_ $name>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<atto_ $name>]
                        }
                        fn [<z $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<z $symbol>]
                        }
                        fn [<y $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<y $symbol>]
                        }
                        fn [<r $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<r $symbol>]
                        }
                        fn [<q $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<q $symbol>]
                        }
                    )*
                }
            }
        }
    };
}

define_units! {
    // base units
    second (s): base::Time,
    metre (m): base::Length,
    kilogram (kg): base::Mass,
    ampere (A): base::ElectricCurrent,
    kelvin (K): base::ThermodynamicTemperature,
    mole (mol): base::AmountOfSubstance,
    candela (cd): base::LuminousIntensity,

    // common derived units
    hertz (Hz): derived::Frequency,
    newton (N): derived::Force,
    joule (J): derived::Energy,
    watt (W): derived::Power,
    pascal (Pa): derived::Pressure,

    // electrical units
    volt (V): derived::Voltage,
    ohms (Ohm): derived::Resistance,
    siemens (S): derived::Conductance,
    coulomb (C): derived::ElectricCharge,
    farad (F): derived::Capacitance,
    henry (H): derived::Inductance,
    tesla (T): derived::MagneticFluxDensity,
    weber (Wb): derived::MagneticFlux,
}
