#[macro_use]
mod macros;

use crate::{
    Sealed,
    dimension::{self, Dimension},
    prefixes,
    quantity::Quantity,
    scalar::{F32Scalar, F64Scalar},
};
use core::{marker::PhantomData, ops::Mul};
use paste::paste;

/// Marker trait for physical units.
pub trait Unit: 'static + Copy {
    /// The physical dimension of this unit.
    const DIMENSION: Dimension;
    /// The SI prefix power of 10 (e.g., kilo = 3, milli = -3).
    const PREFIX: i8;
}

/// Trait for compile-time dimension checking.
pub trait SameDimension<U1: Unit, U2: Unit> {}

/// Type-level proof that two units have the same dimension.
pub struct DimensionEq<U1: Unit, U2: Unit>(PhantomData<(U1, U2)>);
impl<U: Unit> SameDimension<U, U> for DimensionEq<U, U> {}
impl<U1: Unit, U2: Unit> Sealed for DimensionEq<U1, U2> {}

/// Trait for units with a base unit (without prefix).
pub trait BaseUnit: Unit {
    /// The base unit type (e.g., MilliAmpere -> Ampere).
    type Base: Unit;
}

/// Trait for multiplying two units to get a result unit.
pub trait Multiply<Rhs: Unit>: Unit {
    /// The resulting unit type.
    type Output: Unit;
}

/// Trait for dividing two units to get a result unit.
pub trait Divide<Rhs: Unit>: Unit {
    /// The resulting unit type.
    type Output: Unit;
}

define_units! {
    base {
        // Base SI units
        Second (s): dimension::TIME,
        Ampere (A): dimension::ELECTRIC_CURRENT,

        // Frequency
        Hertz (Hz): dimension::FREQUENCY,

        // Core electrical
        Volt (V): dimension::VOLTAGE,
        Ohms (Ohm): dimension::RESISTANCE,
        Siemens (S): dimension::CONDUCTANCE,

        // Charge and storage
        Coulomb (C): dimension::ELECTRIC_CHARGE,
        Farad (F): dimension::CAPACITANCE,

        // Magnetic
        Weber (Wb): dimension::MAGNETIC_FLUX,
        Henry (H): dimension::INDUCTANCE,
        Tesla (T): dimension::MAGNETIC_FLUX_DENSITY,

        // Energy and power
        Watt (W): dimension::POWER,
        Joule (J): dimension::ENERGY,
    }

    mul {
        // Ohm's Law
        Volt = Ampere * Ohms,       // V = I * R

        // Charge
        Coulomb = Ampere * Second,  // Q = I * t

        // Power
        Watt = Volt * Ampere,       // P = V * I

        // Energy
        Joule = Watt * Second,      // E = P * t
        Joule = Volt * Coulomb,     // E = Q * V

        // Magnetic flux
        Weber = Volt * Second,      // Φ = V * t
        Weber = Henry * Ampere,     // Φ = L * I

        // Capacitor
        Coulomb = Farad * Volt,     // Q = C * V

        // Conductance
        Ampere = Siemens * Volt,    // I = G * V

        // Impedance
        Ohms = Henry * Hertz,       // Z = L * ω

        // Time constant
        Second = Farad * Ohms,      // τ = R * C
    }

    div {
        // Ohm's Law
        Ampere = Volt / Ohms,       // I = V/R
        Ohms = Volt / Ampere,       // R = V/I
        Volt = Watt / Ampere,       // V = P/I

        // Current definitions
        Ampere = Coulomb / Second,  // I = Q/t
        Ampere = Watt / Volt,       // I = P/V

        // Conductance
        Siemens = Ampere / Volt,    // G = 1/R

        // Voltage derivations
        Volt = Joule / Coulomb,     // V = E/Q
        Volt = Weber / Second,      // V = dΦ/dt
        Volt = Coulomb / Farad,     // V = Q/C

        // Capacitance
        Farad = Coulomb / Volt,     // C = Q/V
        Farad = Second / Ohms,      // C = τ/R

        // Inductance
        Henry = Weber / Ampere,     // L = Φ/I
        Henry = Volt / Hertz,       // L from impedance

        // Time constants
        Second = Henry / Ohms,      // τ = L/R
        Ohms = Henry / Second,      // R = L/τ

        // Power and energy
        Watt = Joule / Second,      // P = E/t
        Second = Joule / Watt,      // t = E/P

        // Frequency relations
        Hertz = Watt / Joule,       // f from energy
        Hertz = Ampere / Coulomb,   // f from charge

        // Time derivations
        Second = Coulomb / Ampere,  // t = Q/I
        Second = Weber / Volt,      // t = Φ/V
    }
}
