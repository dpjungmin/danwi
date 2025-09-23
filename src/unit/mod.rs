#![allow(non_upper_case_globals)]

use crate::{
    dimension::{Dimensions, generated::*},
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
    second (s): Time,
    meter (m): Length,
    gram (g): Mass, // kilogram is the base unit
    ampere (A): ElectricCurrent,
    kelvin (K): ThermodynamicTemperature,
    mole (mol): AmountOfSubstance,
    candela (cd): LuminousIntensity,

    // common derived units
    hertz (Hz): Frequency,
    newton (N): Force,
    joule (J): Energy,
    watt (W): Power,
    pascal (Pa): Pressure,

    // electrical units
    volt (V): Voltage,
    ohms (Ohm): Resistance,
    siemens (S): Conductance,
    coulomb (C): ElectricCharge,
    farad (F): Capacitance,
    henry (H): Inductance,
    tesla (T): MagneticFluxDensity,
    weber (Wb): MagneticFlux,

    // kinematic units
    meter_per_second (mps): Velocity,
    meter_per_second_squared (mps2): Acceleration,
}
