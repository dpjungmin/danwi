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
                pub const [<G $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::GIGA);
                pub const [<M $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::MEGA);
                pub const [<k $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::KILO);
                pub const $symbol: Unit<$dimension> = Unit::base();
                pub const [<c $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::CENTI);
                pub const [<m $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::MILLI);
                pub const [<u $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::MICRO);
                pub const [<n $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::NANO);

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
                        fn [<G $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<M $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<k $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn $symbol(self) -> Quantity<F32Scalar, $dimension>;
                        fn [<m $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<u $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<n $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                    )*
                }

                pub trait F64QuantityExt {
                    $(
                        fn [<G $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<M $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<k $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn $symbol(self) -> Quantity<F64Scalar, $dimension>;
                        fn [<m $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<u $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<n $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                    )*
                }
            }

            paste::paste! {
                impl F32QuantityExt for f32 {
                    $(
                        fn [<G $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            // F32Scalar::new(self) * [<G $symbol>]
                            Quantity::with_unit(F32Scalar::new(self), Unit::with_prefix(prefix::GIGA))
                        }
                        fn [<M $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            Quantity::with_unit(F32Scalar::new(self), Unit::with_prefix(prefix::MEGA))
                        }
                        fn [<k $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            Quantity::with_unit(F32Scalar::new(self), Unit::with_prefix(prefix::KILO))
                        }
                        fn $symbol(self) -> Quantity<F32Scalar, $dimension> {
                            Quantity::new(F32Scalar::new(self))
                        }
                        fn [<m $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            Quantity::with_unit(F32Scalar::new(self), Unit::with_prefix(prefix::MILLI))
                        }
                        fn [<u $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            Quantity::with_unit(F32Scalar::new(self), Unit::with_prefix(prefix::MICRO))
                        }
                        fn [<n $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            Quantity::with_unit(F32Scalar::new(self), Unit::with_prefix(prefix::NANO))
                        }
                    )*
                }

                impl F64QuantityExt for f64 {
                    $(
                        fn [<G $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            Quantity::with_unit(F64Scalar::new(self), Unit::with_prefix(prefix::GIGA))
                        }
                        fn [<M $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            Quantity::with_unit(F64Scalar::new(self), Unit::with_prefix(prefix::MEGA))
                        }
                        fn [<k $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            Quantity::with_unit(F64Scalar::new(self), Unit::with_prefix(prefix::KILO))
                        }
                        fn $symbol(self) -> Quantity<F64Scalar, $dimension> {
                            Quantity::new(F64Scalar::new(self))
                        }
                        fn [<m $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            Quantity::with_unit(F64Scalar::new(self), Unit::with_prefix(prefix::MILLI))
                        }
                        fn [<u $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            Quantity::with_unit(F64Scalar::new(self), Unit::with_prefix(prefix::MICRO))
                        }
                        fn [<n $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            Quantity::with_unit(F64Scalar::new(self), Unit::with_prefix(prefix::NANO))
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
