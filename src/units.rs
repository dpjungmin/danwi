//! Common SI units.

use crate::{
    quantity::{self, OscillatoryContext, Quantity, RotationalContext},
    storage::{F32Storage, F64Storage, RationalStorage},
};

// Base quantities
pub type Length = Quantity<F64Storage, quantity::Length>;
pub type Mass = Quantity<F64Storage, quantity::Mass>;
pub type Time = Quantity<F64Storage, quantity::Time>;
pub type ElectricCurrent = Quantity<F64Storage, quantity::ElectricCurrent>;
pub type Temperature = Quantity<F64Storage, quantity::Temperature>;
pub type AmountOfSubstance = Quantity<F64Storage, quantity::AmountOfSubstance>;
pub type LuminousIntensity = Quantity<F64Storage, quantity::LuminousIntensity>;

// Derived quantities
pub type Area = Quantity<F64Storage, quantity::Area>;
pub type Volume = Quantity<F64Storage, quantity::Volume>;
pub type Velocity = Quantity<F64Storage, quantity::Velocity>;
pub type Acceleration = Quantity<F64Storage, quantity::Acceleration>;
pub type Force = Quantity<F64Storage, quantity::Force>;
pub type Energy = Quantity<F64Storage, quantity::Energy>;
pub type Power = Quantity<F64Storage, quantity::Power>;
pub type Pressure = Quantity<F64Storage, quantity::Pressure>;
pub type ElectricCharge = Quantity<F64Storage, quantity::ElectricCharge>;
pub type Voltage = Quantity<F64Storage, quantity::Voltage>;
pub type Capacitance = Quantity<F64Storage, quantity::Capacitance>;
pub type Resistance = Quantity<F64Storage, quantity::Resistance>;
pub type Frequency = Quantity<F64Storage, quantity::InverseTime, OscillatoryContext>;
pub type Momentum = Quantity<F64Storage, quantity::Momentum>;
pub type Density = Quantity<F64Storage, quantity::Density>;
pub type AngularVelocity = Quantity<F64Storage, quantity::InverseTime, RotationalContext>;

// Type aliases for F32 storage variants
pub type LengthF32 = Quantity<F32Storage, quantity::Length>;
pub type MassF32 = Quantity<F32Storage, quantity::Mass>;
pub type TimeF32 = Quantity<F32Storage, quantity::Time>;
pub type ElectricCurrentF32 = Quantity<F32Storage, quantity::ElectricCurrent>;
pub type TemperatureF32 = Quantity<F32Storage, quantity::Temperature>;
pub type AmountOfSubstanceF32 = Quantity<F32Storage, quantity::AmountOfSubstance>;
pub type LuminousIntensityF32 = Quantity<F32Storage, quantity::LuminousIntensity>;

// F32 derived quantities
pub type AreaF32 = Quantity<F32Storage, quantity::Area>;
pub type VolumeF32 = Quantity<F32Storage, quantity::Volume>;
pub type VelocityF32 = Quantity<F32Storage, quantity::Velocity>;
pub type AccelerationF32 = Quantity<F32Storage, quantity::Acceleration>;
pub type ForceF32 = Quantity<F32Storage, quantity::Force>;
pub type EnergyF32 = Quantity<F32Storage, quantity::Energy>;
pub type PowerF32 = Quantity<F32Storage, quantity::Power>;
pub type PressureF32 = Quantity<F32Storage, quantity::Pressure>;
pub type ElectricChargeF32 = Quantity<F32Storage, quantity::ElectricCharge>;
pub type VoltageF32 = Quantity<F32Storage, quantity::Voltage>;
pub type CapacitanceF32 = Quantity<F32Storage, quantity::Capacitance>;
pub type ResistanceF32 = Quantity<F32Storage, quantity::Resistance>;
pub type FrequencyF32 = Quantity<F32Storage, quantity::InverseTime, OscillatoryContext>;
pub type MomentumF32 = Quantity<F32Storage, quantity::Momentum>;
pub type DensityF32 = Quantity<F32Storage, quantity::Density>;
pub type AngularVelocityF32 = Quantity<F32Storage, quantity::InverseTime, RotationalContext>;

// Type aliases for Rational storage variants
pub type LengthRational = Quantity<RationalStorage, quantity::Length>;
pub type MassRational = Quantity<RationalStorage, quantity::Mass>;
pub type TimeRational = Quantity<RationalStorage, quantity::Time>;
pub type ElectricCurrentRational = Quantity<RationalStorage, quantity::ElectricCurrent>;
pub type TemperatureRational = Quantity<RationalStorage, quantity::Temperature>;
pub type AmountOfSubstanceRational = Quantity<RationalStorage, quantity::AmountOfSubstance>;
pub type LuminousIntensityRational = Quantity<RationalStorage, quantity::LuminousIntensity>;

// Rational derived quantities
pub type AreaRational = Quantity<RationalStorage, quantity::Area>;
pub type VolumeRational = Quantity<RationalStorage, quantity::Volume>;
pub type VelocityRational = Quantity<RationalStorage, quantity::Velocity>;
pub type AccelerationRational = Quantity<RationalStorage, quantity::Acceleration>;
pub type ForceRational = Quantity<RationalStorage, quantity::Force>;
pub type EnergyRational = Quantity<RationalStorage, quantity::Energy>;
pub type PowerRational = Quantity<RationalStorage, quantity::Power>;
pub type PressureRational = Quantity<RationalStorage, quantity::Pressure>;
pub type ElectricChargeRational = Quantity<RationalStorage, quantity::ElectricCharge>;
pub type VoltageRational = Quantity<RationalStorage, quantity::Voltage>;
pub type CapacitanceRational = Quantity<RationalStorage, quantity::Capacitance>;
pub type ResistanceRational = Quantity<RationalStorage, quantity::Resistance>;
pub type FrequencyRational = Quantity<RationalStorage, quantity::InverseTime, OscillatoryContext>;
pub type MomentumRational = Quantity<RationalStorage, quantity::Momentum>;
pub type DensityRational = Quantity<RationalStorage, quantity::Density>;
pub type AngularVelocityRational =
    Quantity<RationalStorage, quantity::InverseTime, RotationalContext>;

// Conversion methods for semantic contexts
use core::f64::consts::TAU;

impl<S: crate::storage::Storage + Copy> Quantity<S, quantity::InverseTime, OscillatoryContext> {
    /// Convert frequency to angular velocity (ω = 2πf)
    pub fn to_angular_velocity(self) -> Quantity<S, quantity::InverseTime, RotationalContext>
    where
        S: From<f64>,
    {
        let new_storage = self.storage().mul(&S::from(TAU));
        Quantity::new(new_storage)
    }
}

impl<S: crate::storage::Storage + Copy> Quantity<S, quantity::InverseTime, RotationalContext> {
    /// Convert angular velocity to frequency (f = ω/2π)
    pub fn to_frequency(self) -> Quantity<S, quantity::InverseTime, OscillatoryContext>
    where
        S: From<f64>,
    {
        let new_storage = self.storage().div(&S::from(TAU));
        Quantity::new(new_storage)
    }
}

/// SI base unit constants for meter
pub mod meter {
    /// One meter (SI base unit of length)
    pub const METER: f64 = 1.0;

    // Metric prefixes
    pub const KILOMETER: f64 = 1000.0;
    pub const HECTOMETER: f64 = 100.0;
    pub const DECAMETER: f64 = 10.0;
    pub const DECIMETER: f64 = 0.1;
    pub const CENTIMETER: f64 = 0.01;
    pub const MILLIMETER: f64 = 0.001;
    pub const MICROMETER: f64 = 1e-6;
    pub const NANOMETER: f64 = 1e-9;

    // Common imperial units
    pub const INCH: f64 = 0.0254;
    pub const FOOT: f64 = 0.3048;
    pub const YARD: f64 = 0.9144;
    pub const MILE: f64 = 1609.344;
}

/// SI base unit constants for kilogram
pub mod kilogram {
    /// One kilogram (SI base unit of mass)
    pub const KILOGRAM: f64 = 1.0;

    // Metric units
    pub const GRAM: f64 = 0.001;
    pub const MILLIGRAM: f64 = 1e-6;
    pub const MICROGRAM: f64 = 1e-9;
    pub const METRIC_TON: f64 = 1000.0;

    // Imperial units
    pub const POUND: f64 = 0.45359237;
    pub const OUNCE: f64 = 0.028349523125;
}

/// SI base unit constants for second
pub mod second {
    /// One second (SI base unit of time)
    pub const SECOND: f64 = 1.0;

    // Time units
    pub const MILLISECOND: f64 = 0.001;
    pub const MICROSECOND: f64 = 1e-6;
    pub const NANOSECOND: f64 = 1e-9;
    pub const MINUTE: f64 = 60.0;
    pub const HOUR: f64 = 3600.0;
    pub const DAY: f64 = 86400.0;
}

/// SI base unit constants for ampere
pub mod ampere {
    /// One ampere (SI base unit of electric current)
    pub const AMPERE: f64 = 1.0;

    // Common prefixes
    pub const MILLIAMPERE: f64 = 0.001;
    pub const MICROAMPERE: f64 = 1e-6;
}

/// SI base unit constants for kelvin
pub mod kelvin {
    /// One kelvin (SI base unit of thermodynamic temperature)
    pub const KELVIN: f64 = 1.0;

    /// Celsius to Kelvin offset
    pub const CELSIUS_OFFSET: f64 = 273.15;
}

// Helper functions for creating quantities
impl Length {
    pub fn meters(value: f64) -> Self {
        Self::from_f64(value)
    }

    pub fn kilometers(value: f64) -> Self {
        Self::from_f64(value * meter::KILOMETER)
    }

    pub fn centimeters(value: f64) -> Self {
        Self::from_f64(value * meter::CENTIMETER)
    }

    pub fn millimeters(value: f64) -> Self {
        Self::from_f64(value * meter::MILLIMETER)
    }

    pub fn inches(value: f64) -> Self {
        Self::from_f64(value * meter::INCH)
    }

    pub fn feet(value: f64) -> Self {
        Self::from_f64(value * meter::FOOT)
    }

    pub fn miles(value: f64) -> Self {
        Self::from_f64(value * meter::MILE)
    }
}

impl Mass {
    pub fn kilograms(value: f64) -> Self {
        Self::from_f64(value)
    }

    pub fn grams(value: f64) -> Self {
        Self::from_f64(value * kilogram::GRAM)
    }

    pub fn milligrams(value: f64) -> Self {
        Self::from_f64(value * kilogram::MILLIGRAM)
    }

    pub fn pounds(value: f64) -> Self {
        Self::from_f64(value * kilogram::POUND)
    }
}

impl Time {
    pub fn seconds(value: f64) -> Self {
        Self::from_f64(value)
    }

    pub fn milliseconds(value: f64) -> Self {
        Self::from_f64(value * second::MILLISECOND)
    }

    pub fn microseconds(value: f64) -> Self {
        Self::from_f64(value * second::MICROSECOND)
    }

    pub fn minutes(value: f64) -> Self {
        Self::from_f64(value * second::MINUTE)
    }

    pub fn hours(value: f64) -> Self {
        Self::from_f64(value * second::HOUR)
    }
}

impl ElectricCurrent {
    pub fn amperes(value: f64) -> Self {
        Self::from_f64(value)
    }

    pub fn milliamperes(value: f64) -> Self {
        Self::from_f64(value * ampere::MILLIAMPERE)
    }

    pub fn microamperes(value: f64) -> Self {
        Self::from_f64(value * ampere::MICROAMPERE)
    }
}

impl Voltage {
    pub fn volts(value: f64) -> Self {
        Self::from_f64(value)
    }

    pub fn millivolts(value: f64) -> Self {
        Self::from_f64(value * 0.001)
    }

    pub fn kilovolts(value: f64) -> Self {
        Self::from_f64(value * 1000.0)
    }
}

impl Resistance {
    pub fn ohms(value: f64) -> Self {
        Self::from_f64(value)
    }

    pub fn kilohms(value: f64) -> Self {
        Self::from_f64(value * 1000.0)
    }

    pub fn megohms(value: f64) -> Self {
        Self::from_f64(value * 1e6)
    }
}

impl Velocity {
    pub fn meters_per_second(value: f64) -> Self {
        Self::from_f64(value)
    }

    pub fn kilometers_per_hour(value: f64) -> Self {
        Self::from_f64(value * meter::KILOMETER / second::HOUR)
    }

    pub fn miles_per_hour(value: f64) -> Self {
        Self::from_f64(value * meter::MILE / second::HOUR)
    }
}

impl Force {
    pub fn newtons(value: f64) -> Self {
        Self::from_f64(value)
    }

    pub fn kilonewtons(value: f64) -> Self {
        Self::from_f64(value * 1000.0)
    }
}

impl Energy {
    pub fn joules(value: f64) -> Self {
        Self::from_f64(value)
    }

    pub fn kilojoules(value: f64) -> Self {
        Self::from_f64(value * 1000.0)
    }

    pub fn megajoules(value: f64) -> Self {
        Self::from_f64(value * 1e6)
    }

    pub fn kilowatt_hours(value: f64) -> Self {
        Self::from_f64(value * 3.6e6)
    }

    pub fn calories(value: f64) -> Self {
        Self::from_f64(value * 4.184)
    }

    pub fn kilocalories(value: f64) -> Self {
        Self::from_f64(value * 4184.0)
    }
}

impl Power {
    pub fn watts(value: f64) -> Self {
        Self::from_f64(value)
    }

    pub fn milliwatts(value: f64) -> Self {
        Self::from_f64(value * 0.001)
    }

    pub fn kilowatts(value: f64) -> Self {
        Self::from_f64(value * 1000.0)
    }

    pub fn megawatts(value: f64) -> Self {
        Self::from_f64(value * 1e6)
    }

    pub fn horsepower(value: f64) -> Self {
        Self::from_f64(value * 745.7)
    }
}

impl Acceleration {
    pub fn meters_per_second_squared(value: f64) -> Self {
        Self::from_f64(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_constructors_work() {
        let m = Length::meters(1.0);
        let km = Length::kilometers(1.0);
        let cm = Length::centimeters(100.0);

        // 1 km = 1000 m
        assert!((km.value() - 1000.0).abs() < 1e-10);
        // 100 cm = 1 m
        assert!((cm.value() - 1.0).abs() < 1e-10);
        // 1 m = 1 m
        assert!((m.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn unit_addition_works() {
        let a = Length::meters(3.0);
        let b = Length::meters(2.0);
        let sum = a + b;
        assert_eq!(sum.value(), 5.0);
    }

    #[test]
    fn unit_conversion_in_operations() {
        let km = Length::kilometers(1.0);
        let m = Length::meters(500.0);
        let total = km + m;

        // 1 km + 500 m = 1500 m
        assert!((total.value() - 1500.0).abs() < 1e-10);
    }

    #[test]
    fn dimension_arithmetic_works() {
        // Test velocity calculation
        let distance = Length::meters(100.0);
        let time = Time::seconds(10.0);
        let velocity = distance / time;
        assert_eq!(velocity.value(), 10.0);

        // Test force calculation
        let mass = Mass::kilograms(10.0);
        let accel = Acceleration::meters_per_second_squared(9.8);
        let force = mass * accel;
        assert_eq!(force.value(), 98.0);
    }
}
