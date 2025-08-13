//! Unit constants for natural equation syntax.
//!
//! Constants follow SI conventions exactly, making equations look like textbook
//! math.

#![allow(non_upper_case_globals)]

use crate::units::*;

// LENGTH

/// Meter - SI base unit of length
pub const m: Length = Length::from_f64(1.0);

/// Kilometer - 1000 meters
pub const km: Length = Length::from_f64(1000.0);

/// Centimeter - 0.01 meters
pub const cm: Length = Length::from_f64(0.01);

/// Millimeter - 0.001 meters
pub const mm: Length = Length::from_f64(0.001);

/// Micrometer - 10^-6 meters
pub const um: Length = Length::from_f64(1e-6);

/// Nanometer - 10^-9 meters
pub const nm: Length = Length::from_f64(1e-9);

/// Foot - 0.3048 meters
pub const ft: Length = Length::from_f64(0.3048);

/// Inch - 0.0254 meters
pub const inch: Length = Length::from_f64(0.0254);

/// Mile - 1609.344 meters
pub const mi: Length = Length::from_f64(1609.344);

// MASS

/// Kilogram - SI base unit of mass
pub const kg: Mass = Mass::from_f64(1.0);

/// Gram - 0.001 kilograms
pub const g: Mass = Mass::from_f64(0.001);

/// Milligram - 10^-6 kilograms
pub const mg: Mass = Mass::from_f64(1e-6);

/// Microgram - 10^-9 kilograms
pub const ug: Mass = Mass::from_f64(1e-9);

/// Metric ton - 1000 kilograms
pub const t: Mass = Mass::from_f64(1000.0);

/// Pound - 0.45359237 kilograms
pub const lb: Mass = Mass::from_f64(0.45359237);

/// Ounce - 0.028349523125 kilograms
pub const oz: Mass = Mass::from_f64(0.028349523125);

// TIME

/// Second - SI base unit of time
pub const s: Time = Time::from_f64(1.0);

/// Millisecond - 0.001 seconds
pub const ms: Time = Time::from_f64(0.001);

/// Microsecond - 10^-6 seconds
pub const us: Time = Time::from_f64(1e-6);

/// Nanosecond - 10^-9 seconds
pub const ns: Time = Time::from_f64(1e-9);

/// Minute - 60 seconds
pub const minute: Time = Time::from_f64(60.0);

/// Hour - 3600 seconds
pub const h: Time = Time::from_f64(3600.0);

/// Hour - 3600 seconds (alternative)
pub const hr: Time = Time::from_f64(3600.0);

/// Day - 86400 seconds
pub const day: Time = Time::from_f64(86400.0);

// ELECTRIC CURRENT

/// Ampere - SI base unit of electric current
pub const A: ElectricCurrent = ElectricCurrent::from_f64(1.0);

/// Milliampere - 0.001 amperes
pub const mA: ElectricCurrent = ElectricCurrent::from_f64(0.001);

/// Microampere - 10^-6 amperes
pub const uA: ElectricCurrent = ElectricCurrent::from_f64(1e-6);

/// Nanoampere - 10^-9 amperes
pub const nA: ElectricCurrent = ElectricCurrent::from_f64(1e-9);

// TEMPERATURE

/// Kelvin - SI base unit of thermodynamic temperature
pub const K: Temperature = Temperature::from_f64(1.0);

// Note: Celsius and Fahrenheit need special handling due to offset
// Users should use .degC() and .degF() methods instead
// Planning to add support in the near future.

// VOLTAGE

/// Volt - SI derived unit of electric potential
pub const V: Voltage = Voltage::from_f64(1.0);

/// Millivolt - 0.001 volts
pub const mV: Voltage = Voltage::from_f64(0.001);

/// Microvolt - 10^-6 volts
pub const uV: Voltage = Voltage::from_f64(1e-6);

/// Kilovolt - 1000 volts
pub const kV: Voltage = Voltage::from_f64(1000.0);

/// Megavolt - 10^6 volts
pub const MV: Voltage = Voltage::from_f64(1e6);

// RESISTANCE

/// Ohm - SI derived unit of electrical resistance
pub const Ohm: Resistance = Resistance::from_f64(1.0);

/// Kiloohm - 1000 ohms
pub const kOhm: Resistance = Resistance::from_f64(1000.0);

/// Megaohm - 10^6 ohms
pub const MOhm: Resistance = Resistance::from_f64(1e6);

/// Gigaohm - 10^9 ohms
pub const GOhm: Resistance = Resistance::from_f64(1e9);

// POWER

/// Watt - SI derived unit of power
pub const W: Power = Power::from_f64(1.0);

/// Milliwatt - 0.001 watts
pub const mW: Power = Power::from_f64(0.001);

/// Kilowatt - 1000 watts
pub const kW: Power = Power::from_f64(1000.0);

/// Megawatt - 10^6 watts
pub const MW: Power = Power::from_f64(1e6);

/// Gigawatt - 10^9 watts
pub const GW: Power = Power::from_f64(1e9);

/// Horsepower - 745.7 watts
pub const hp: Power = Power::from_f64(745.7);

// ENERGY

/// Joule - SI derived unit of energy
pub const J: Energy = Energy::from_f64(1.0);

/// Kilojoule - 1000 joules
pub const kJ: Energy = Energy::from_f64(1000.0);

/// Megajoule - 10^6 joules
pub const MJ: Energy = Energy::from_f64(1e6);

/// Gigajoule - 10^9 joules
pub const GJ: Energy = Energy::from_f64(1e9);

/// Calorie - 4.184 joules
pub const cal: Energy = Energy::from_f64(4.184);

/// Kilocalorie - 4184 joules
pub const kcal: Energy = Energy::from_f64(4184.0);

/// Kilowatt-hour - 3.6×10^6 joules
pub const kWh: Energy = Energy::from_f64(3.6e6);

/// Electron-volt - 1.602176634×10^-19 joules
pub const eV: Energy = Energy::from_f64(1.602176634e-19);

// FORCE

/// Newton - SI derived unit of force
pub const N: Force = Force::from_f64(1.0);

/// Kilonewton - 1000 newtons
pub const kN: Force = Force::from_f64(1000.0);

/// Meganewton - 10^6 newtons
pub const MN: Force = Force::from_f64(1e6);

/// Pound-force - 4.448222 newtons
pub const lbf: Force = Force::from_f64(4.448222);

// PRESSURE

/// Pascal - SI derived unit of pressure
pub const Pa: Pressure = Pressure::from_f64(1.0);

/// Kilopascal - 1000 pascals
pub const kPa: Pressure = Pressure::from_f64(1000.0);

/// Megapascal - 10^6 pascals
pub const MPa: Pressure = Pressure::from_f64(1e6);

/// Gigapascal - 10^9 pascals
pub const GPa: Pressure = Pressure::from_f64(1e9);

/// Bar - 10^5 pascals
pub const bar: Pressure = Pressure::from_f64(1e5);

/// Millibar - 100 pascals
pub const mbar: Pressure = Pressure::from_f64(100.0);

/// Atmosphere - 101325 pascals
pub const atm: Pressure = Pressure::from_f64(101325.0);

/// Pounds per square inch - 6894.76 pascals
pub const psi: Pressure = Pressure::from_f64(6894.76);

// FREQUENCY

/// Hertz - SI derived unit of frequency
pub const Hz: Frequency = Frequency::from_f64(1.0);

/// Kilohertz - 1000 hertz
pub const kHz: Frequency = Frequency::from_f64(1000.0);

/// Megahertz - 10^6 hertz
pub const MHz: Frequency = Frequency::from_f64(1e6);

/// Gigahertz - 10^9 hertz
pub const GHz: Frequency = Frequency::from_f64(1e9);

/// Terahertz - 10^12 hertz
pub const THz: Frequency = Frequency::from_f64(1e12);

// CAPACITANCE

/// Farad - SI derived unit of capacitance
pub const F: Capacitance = Capacitance::from_f64(1.0);

/// Millifarad - 0.001 farads
pub const mF: Capacitance = Capacitance::from_f64(0.001);

/// Microfarad - 10^-6 farads
pub const uF: Capacitance = Capacitance::from_f64(1e-6);

/// Nanofarad - 10^-9 farads
pub const nF: Capacitance = Capacitance::from_f64(1e-9);

/// Picofarad - 10^-12 farads
pub const pF: Capacitance = Capacitance::from_f64(1e-12);

// VELOCITY

/// Meter per second - SI derived unit of velocity
pub const m_per_s: Velocity = Velocity::from_f64(1.0);

/// Kilometer per hour - 0.277778 m/s
pub const km_per_h: Velocity = Velocity::from_f64(0.277778);

/// Mile per hour - 0.44704 m/s
pub const mph: Velocity = Velocity::from_f64(0.44704);

// ACCELERATION

/// Meter per second squared - SI derived unit of acceleration
pub const m_per_s2: Acceleration = Acceleration::from_f64(1.0);

/// Standard gravity - 9.80665 m/s²
pub const g0: Acceleration = Acceleration::from_f64(9.80665);

// AREA

/// Square meter - SI derived unit of area
pub const m2: Area = Area::from_f64(1.0);

/// Square kilometer - 10^6 m²
pub const km2: Area = Area::from_f64(1e6);

/// Square centimeter - 10^-4 m²
pub const cm2: Area = Area::from_f64(1e-4);

/// Square millimeter - 10^-6 m²
pub const mm2: Area = Area::from_f64(1e-6);

// VOLUME

/// Cubic meter - SI derived unit of volume
pub const m3: Volume = Volume::from_f64(1.0);

/// Liter - 0.001 m³
pub const L: Volume = Volume::from_f64(0.001);

/// Milliliter - 10^-6 m³
pub const mL: Volume = Volume::from_f64(1e-6);

/// Cubic centimeter - 10^-6 m³
pub const cm3: Volume = Volume::from_f64(1e-6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoketest() {
        assert_eq!((2.0 * mA) * (1000.0 * Ohm), 2.0 * V);
        assert_eq!((2.0 * mA) * (1.0 * kOhm), 2.0 * V);
        assert_eq!(12.0 * V * 0.5 * A, 6.0 * W);
        assert_eq!(100.0 * W * 3.0 * h, 100.0 * 3.0 * 3600.0 * J);
    }
}
