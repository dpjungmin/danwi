//! Quantity implementation for dimension combinations.

use crate::{
    sealed::Sealed,
    storage::{F32Storage, F64Storage, RationalStorage, Storage},
};
use core::{
    fmt,
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// Dimension trait for marker types
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait Dimension: Clone + Copy + fmt::Debug + 'static + Sealed {
    const SYMBOL: &'static str;
}

/// Context trait for semantic meaning of quantities
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait Context: Clone + Copy + fmt::Debug + 'static + Sealed {
    /// Optional name for the context (e.g., "Oscillatory", "Rotational")
    const NAME: &'static str;
}

/// Default context for quantities without semantic distinctions
impl Sealed for () {}
impl Context for () {
    const NAME: &'static str = "";
}

/// Context for oscillatory quantities (e.g., frequency)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OscillatoryContext;

impl Sealed for OscillatoryContext {}
impl Context for OscillatoryContext {
    const NAME: &'static str = "Oscillatory";
}

/// Context for rotational quantities (e.g., angular velocity)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RotationalContext;

impl Sealed for RotationalContext {}
impl Context for RotationalContext {
    const NAME: &'static str = "Rotational";
}

/// Context for linear energy (work, kinetic energy)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearContext;

impl Sealed for LinearContext {}
impl Context for LinearContext {
    const NAME: &'static str = "Linear";
}

/// Context for torque (rotational force)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TorqueContext;

impl Sealed for TorqueContext {}
impl Context for TorqueContext {
    const NAME: &'static str = "Torque";
}

/// A physical quantity with compile-time dimensional analysis and optional
/// semantic context.
///
/// # Examples
///
/// ```compile_fail
/// use danwi::prelude::*;
///
/// let freq = Frequency::from_f64(50.0);
/// let angular = AngularVelocity::from_f64(100.0);
///
/// // This will not compile - different contexts!
/// let invalid = freq + angular;  // ERROR: type mismatch
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quantity<S: Storage + Copy, D: Dimension, C: Context = ()> {
    storage: S,
    _phantom: PhantomData<(D, C)>,
}

impl<S: Storage + Copy, D: Dimension, C: Context> Quantity<S, D, C> {
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

    pub fn without_context(self) -> Quantity<S, D, ()> {
        Quantity::new(self.storage)
    }

    pub fn with_context<C2: Context>(self) -> Quantity<S, D, C2> {
        Quantity::new(self.storage)
    }
}

impl<D: Dimension, C: Context> Quantity<F32Storage, D, C> {
    pub const fn from_f32(value: f32) -> Self {
        Self::new(F32Storage::new(value))
    }
}

impl<D: Dimension, C: Context> Quantity<F64Storage, D, C> {
    pub const fn from_f64(value: f64) -> Self {
        Self::new(F64Storage::new(value))
    }
}

impl<D: Dimension, C: Context> Quantity<RationalStorage, D, C> {
    pub fn from_ratio(numerator: i128, denominator: u128) -> Self {
        Self::new(RationalStorage::from_ratio(numerator, denominator))
    }

    pub fn from_int(value: i128) -> Self {
        Self::new(RationalStorage::from_int(value))
    }
}

impl<S: Storage + Copy, D: Dimension, C: Context> Add for Quantity<S, D, C> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.storage.add(&other.storage))
    }
}

impl<S: Storage + Copy, D: Dimension, C: Context> Sub for Quantity<S, D, C> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.storage.sub(&other.storage))
    }
}

impl<S: Storage + Copy, D: Dimension, C: Context> Neg for Quantity<S, D, C> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.storage.neg())
    }
}

// quantity * scalar
impl<S: Storage + Copy, D: Dimension, C: Context> Mul<f64> for Quantity<S, D, C>
where
    S: From<f64>,
{
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.storage.mul(&S::from(scalar)))
    }
}

// scalar * quantity
impl<S: Storage + Copy, D: Dimension, C: Context> Mul<Quantity<S, D, C>> for f64
where
    S: From<f64>,
{
    type Output = Quantity<S, D, C>;

    fn mul(self, quantity: Quantity<S, D, C>) -> Self::Output {
        quantity * self
    }
}

impl<S: Storage + Copy, D: Dimension, C: Context> Div<f64> for Quantity<S, D, C>
where
    S: From<f64>,
{
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.storage.div(&S::from(scalar)))
    }
}

impl<S: Storage + Copy, D: Dimension, C: Context> fmt::Display for Quantity<S, D, C>
where
    S::Value: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if C::NAME.is_empty() {
            write!(f, "{} {}", self.storage.raw_value(), D::SYMBOL)
        } else {
            write!(
                f,
                "{} {} ({})",
                self.storage.raw_value(),
                D::SYMBOL,
                C::NAME
            )
        }
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

            impl $crate::sealed::Sealed for $base {}
            impl Dimension for $base {
                const SYMBOL: &'static str = $base_symbol;
            }
        )*

        // Define derived dimension marker types
        $(
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct $derived;

            impl $crate::sealed::Sealed for $derived {}
            impl Dimension for $derived {
                const SYMBOL: &'static str = $derived_symbol;
            }
        )*

        // Implement multiplication rules
        // When multiplying quantities with contexts, the result has no context (default)
        $(
            impl<S: Storage + Copy, C1: Context, C2: Context> Mul<Quantity<S, $rhs, C2>> for Quantity<S, $lhs, C1> {
                type Output = Quantity<S, $result, ()>;

                fn mul(self, other: Quantity<S, $rhs, C2>) -> Self::Output {
                    Quantity::new(self.storage.mul(&other.storage))
                }
            }
        )*

        // Implement division rules
        // When dividing quantities with contexts, the result has no context (default)
        $(
            impl<S: Storage + Copy, C1: Context, C2: Context> Div<Quantity<S, $den, C2>> for Quantity<S, $num, C1> {
                type Output = Quantity<S, $quot, ()>;

                fn div(self, other: Quantity<S, $den, C2>) -> Self::Output {
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
        InverseTime: "1/T" = "1/T",  // Base for both Frequency and AngularVelocity
        Momentum: "kg·m/s" = "M·L/T",
        Density: "kg/m³" = "M/L³",
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

        // InverseTime (Frequency/AngularVelocity)
        InverseTime * Time = Dimensionless,

        // Areas and volumes
        Length * Length = Area,
        Area * Length = Volume,
        Length * Area = Volume
    }

    division_rules: {
        // Basic mechanics
        Force / Mass = Acceleration,
        Energy / Force = Length,
        Power / Energy = InverseTime,
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

        // InverseTime
        Dimensionless / Time = InverseTime,
        Dimensionless / InverseTime = Time,

        // Areas and volumes
        Area / Length = Length,
        Volume / Area = Length,
        Volume / Length = Area
    }
}
