//! Quantity implementation for dimension combinations.

use crate::storage::{F32Storage, F64Storage, RationalStorage, Storage};
use std::{
    fmt,
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// Dimension trait for marker types
pub trait Dimension: Clone + Copy + fmt::Debug + 'static {
    const SYMBOL: &'static str;
}

/// A physical quantity with compile-time dimensional analysis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quantity<S: Storage + Copy, D: Dimension> {
    storage: S,
    _phantom: PhantomData<D>,
}

impl<S: Storage + Copy, D: Dimension> Quantity<S, D> {
    pub const fn new(storage: S) -> Self {
        Self {
            storage,
            _phantom: PhantomData,
        }
    }

    pub fn value(&self) -> S::Value {
        self.storage.raw_value()
    }

    pub fn storage(&self) -> &S {
        &self.storage
    }
}

impl<D: Dimension> Quantity<F32Storage, D> {
    pub const fn from_f32(value: f32) -> Self {
        Self::new(F32Storage::new(value))
    }
}

impl<D: Dimension> Quantity<F64Storage, D> {
    pub const fn from_f64(value: f64) -> Self {
        Self::new(F64Storage::new(value))
    }
}

impl<D: Dimension> Quantity<RationalStorage, D> {
    pub fn from_ratio(numerator: i128, denominator: u128) -> Self {
        Self::new(RationalStorage::from_ratio(numerator, denominator))
    }

    pub fn from_int(value: i128) -> Self {
        Self::new(RationalStorage::from_int(value))
    }
}

impl<S: Storage + Copy, D: Dimension> Add for Quantity<S, D> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.storage.add(&other.storage))
    }
}

impl<S: Storage + Copy, D: Dimension> Sub for Quantity<S, D> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.storage.sub(&other.storage))
    }
}

impl<S: Storage + Copy, D: Dimension> Neg for Quantity<S, D> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.storage.neg())
    }
}

// quantity * scalar
impl<S: Storage + Copy, D: Dimension> Mul<f64> for Quantity<S, D>
where
    S: From<f64>,
{
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.storage.mul(&S::from(scalar)))
    }
}

// scalar * quantity
impl<S: Storage + Copy, D: Dimension> Mul<Quantity<S, D>> for f64
where
    S: From<f64>,
{
    type Output = Quantity<S, D>;

    fn mul(self, quantity: Quantity<S, D>) -> Self::Output {
        quantity * self
    }
}

impl<S: Storage + Copy, D: Dimension> Div<f64> for Quantity<S, D>
where
    S: From<f64>,
{
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.storage.div(&S::from(scalar)))
    }
}

impl<S: Storage + Copy, D: Dimension> fmt::Display for Quantity<S, D>
where
    S::Value: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.storage.raw_value(), D::SYMBOL)
    }
}

/// Macro for defining dimensions and their arithmetic relationships.
///
/// This macro generates dimension types and implements multiplication/division
/// rules between them to ensure compile-time dimensional correctness.
#[macro_export]
macro_rules! define_dimensions {
    (
        base_dimensions: {
            $($base:ident: $base_symbol:literal),* $(,)?
        }
        derived_dimensions: {
            $(
                $derived:ident: $derived_symbol:literal = $derived_expr:expr
            ),* $(,)?
        }
        multiplication_rules: {
            $(
                $lhs:ident * $rhs:ident = $result:ident
            ),* $(,)?
        }
        division_rules: {
            $(
                $num:ident / $den:ident = $quot:ident
            ),* $(,)?
        }
    ) => {
        // Define base dimension marker types
        $(
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct $base;

            impl Dimension for $base {
                const SYMBOL: &'static str = $base_symbol;
            }
        )*

        // Define derived dimension marker types
        $(
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct $derived;

            impl Dimension for $derived {
                const SYMBOL: &'static str = $derived_symbol;
            }
        )*

        // Implement multiplication rules
        $(
            impl<S: Storage + Copy> Mul<Quantity<S, $rhs>> for Quantity<S, $lhs> {
                type Output = Quantity<S, $result>;

                fn mul(self, other: Quantity<S, $rhs>) -> Self::Output {
                    Quantity::new(self.storage.mul(&other.storage))
                }
            }
        )*

        // Implement division rules
        $(
            impl<S: Storage + Copy> Div<Quantity<S, $den>> for Quantity<S, $num> {
                type Output = Quantity<S, $quot>;

                fn div(self, other: Quantity<S, $den>) -> Self::Output {
                    Quantity::new(self.storage.div(&other.storage))
                }
            }
        )*
    };
}

define_dimensions! {
    base_dimensions: {
        Length: "m",
        Mass: "kg",
        Time: "s",
        ElectricCurrent: "A",
        Temperature: "K",
        AmountOfSubstance: "mol",
        LuminousIntensity: "cd",
        Dimensionless: "",
    }

    derived_dimensions: {
        Area: "m²" = "L²",
        Volume: "m³" = "L³",
        Velocity: "m/s" = "L/T",
        Acceleration: "m/s²" = "L/T²",
        Force: "N" = "M·L/T²",
        Energy: "J" = "M·L²/T²",
        Power: "W" = "M·L²/T³",
        Pressure: "Pa" = "M/(L·T²)",
        ElectricCharge: "C" = "I·T",
        Voltage: "V" = "M·L²/(T³·I)",
        Capacitance: "F" = "I²·T⁴/(M·L²)",
        Resistance: "Ω" = "M·L²/(T³·I²)",
        Frequency: "Hz" = "1/T",
        Momentum: "kg·m/s" = "M·L/T",
        Density: "kg/m³" = "M/L³",
        AngularVelocity: "rad/s" = "1/T",
    }

    multiplication_rules: {
        // Basic mechanics
        Mass * Acceleration = Force,
        Force * Length = Energy,
        Mass * Velocity = Momentum,
        Momentum * Velocity = Energy,
        Force * Time = Momentum,

        // Velocity and time
        Velocity * Time = Length,
        Acceleration * Time = Velocity,

        // Density and volume
        Density * Volume = Mass,

        // Angular mechanics

        // Electrical
        ElectricCurrent * Resistance = Voltage,
        Voltage * ElectricCurrent = Power,
        ElectricCurrent * Voltage = Power,  // Both orderings
        ElectricCurrent * Time = ElectricCharge,
        Resistance * Capacitance = Time,

        // Energy from power
        Power * Time = Energy,

        // Frequency
        Frequency * Time = Dimensionless,

        // Areas and volumes
        Length * Length = Area,
        Area * Length = Volume,
        Length * Area = Volume
    }

    division_rules: {
        // Basic mechanics
        Force / Mass = Acceleration,
        Energy / Force = Length,
        Power / Energy = Frequency,
        Pressure / Force = Area,
        Energy / Length = Force,
        Momentum / Mass = Velocity,
        Momentum / Velocity = Mass,
        Energy / Velocity = Momentum,
        Momentum / Time = Force,

        // Density
        Mass / Volume = Density,
        Mass / Density = Volume,

        // Velocity and distance
        Length / Time = Velocity,
        Velocity / Time = Acceleration,
        Length / Velocity = Time,
        Velocity / Acceleration = Time,

        // Angular mechanics

        // Electrical
        Voltage / ElectricCurrent = Resistance,
        Voltage / Resistance = ElectricCurrent,
        Power / Voltage = ElectricCurrent,
        Power / ElectricCurrent = Voltage,
        ElectricCharge / Time = ElectricCurrent,
        Time / Resistance = Capacitance,

        // Frequency
        Dimensionless / Time = Frequency,
        Dimensionless / Frequency = Time,

        // Areas and volumes
        Area / Length = Length,
        Volume / Area = Length,
        Volume / Length = Area
    }
}
