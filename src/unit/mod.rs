#![allow(non_upper_case_globals)]

use crate::{
    dimension::{Dimensions, base, derived},
    quantity::Quantity,
    scalar::{F32Scalar, F64Scalar},
};
use core::marker::PhantomData;

#[macro_use]
mod macros;
mod ops;
mod prefix;

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
