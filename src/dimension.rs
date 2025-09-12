//! Type-level dimensions with compile-time arithmetics.

use core::ops::{Add, Neg, Sub};
use typenum::{Diff, Integer, Negate, P1, P2, Prod, Sum, Z0};

/// Type-level dimension representation.
///
/// Type parameters represent exponents for each SI base unit:
/// - T: Time (second, s)
/// - L: Length (metre, m)
/// - M: Mass (kilogram, kg)
/// - I: Electric Current (ampere, A)
/// - K: Thermodynamic Temperature (kelvin, K)
/// - N: Amount of Substance (mole, mol)
/// - J: Luminous Intensity (candela, cd)
pub type Dimension<T, L, M, I, K, N, J> = (T, L, M, I, K, N, J);

/// Trait to extract type parameters from a Dimension.
pub trait Dimensions {
    type T: Integer;
    type L: Integer;
    type M: Integer;
    type I: Integer;
    type K: Integer;
    type N: Integer;
    type J: Integer;
}

impl<T: Integer, L: Integer, M: Integer, I: Integer, K: Integer, N: Integer, J: Integer> Dimensions
    for Dimension<T, L, M, I, K, N, J>
{
    type T = T;
    type L = L;
    type M = M;
    type I = I;
    type K = K;
    type N = N;
    type J = J;
}

/// Add two dimensions.
pub type DimensionAdd<D1, D2> = Dimension<
    Sum<<D1 as Dimensions>::T, <D2 as Dimensions>::T>,
    Sum<<D1 as Dimensions>::L, <D2 as Dimensions>::L>,
    Sum<<D1 as Dimensions>::M, <D2 as Dimensions>::M>,
    Sum<<D1 as Dimensions>::I, <D2 as Dimensions>::I>,
    Sum<<D1 as Dimensions>::K, <D2 as Dimensions>::K>,
    Sum<<D1 as Dimensions>::N, <D2 as Dimensions>::N>,
    Sum<<D1 as Dimensions>::J, <D2 as Dimensions>::J>,
>;

/// Subtract two dimensions.
pub type DimensionSub<D1, D2> = Dimension<
    Diff<<D1 as Dimensions>::T, <D2 as Dimensions>::T>,
    Diff<<D1 as Dimensions>::L, <D2 as Dimensions>::L>,
    Diff<<D1 as Dimensions>::M, <D2 as Dimensions>::M>,
    Diff<<D1 as Dimensions>::I, <D2 as Dimensions>::I>,
    Diff<<D1 as Dimensions>::K, <D2 as Dimensions>::K>,
    Diff<<D1 as Dimensions>::N, <D2 as Dimensions>::N>,
    Diff<<D1 as Dimensions>::J, <D2 as Dimensions>::J>,
>;

/// Multiply two dimensions (add exponents).
pub type DimensionMul<D1, D2> = DimensionAdd<D1, D2>;

/// Divide two dimensions (subtract exponents).
pub type DimensionDiv<D1, D2> = DimensionSub<D1, D2>;

/// Raise dimension to a power (multiply all exponents).
pub type DimensionPow<D, E> = Dimension<
    Prod<<D as Dimensions>::T, E>,
    Prod<<D as Dimensions>::L, E>,
    Prod<<D as Dimensions>::M, E>,
    Prod<<D as Dimensions>::I, E>,
    Prod<<D as Dimensions>::K, E>,
    Prod<<D as Dimensions>::N, E>,
    Prod<<D as Dimensions>::J, E>,
>;

/// Reciprocal of a dimension (negate all exponents).
pub type DimensionRecip<D> = Dimension<
    Negate<<D as Dimensions>::T>,
    Negate<<D as Dimensions>::L>,
    Negate<<D as Dimensions>::M>,
    Negate<<D as Dimensions>::I>,
    Negate<<D as Dimensions>::K>,
    Negate<<D as Dimensions>::N>,
    Negate<<D as Dimensions>::J>,
>;

/// Helper trait for dimension multiplication operations.
/// Encapsulates all the trait bounds needed for multiplying two dimensions.
pub trait CanMultiplyWith<Rhs: Dimensions>: Dimensions {
    type Output: Dimensions;
}

// Blanket implementation for all valid dimension combinations
impl<Lhs, Rhs> CanMultiplyWith<Rhs> for Lhs
where
    Lhs: Dimensions,
    Rhs: Dimensions,
    <Lhs as Dimensions>::T: Add<<Rhs as Dimensions>::T>,
    <Lhs as Dimensions>::L: Add<<Rhs as Dimensions>::L>,
    <Lhs as Dimensions>::M: Add<<Rhs as Dimensions>::M>,
    <Lhs as Dimensions>::I: Add<<Rhs as Dimensions>::I>,
    <Lhs as Dimensions>::K: Add<<Rhs as Dimensions>::K>,
    <Lhs as Dimensions>::N: Add<<Rhs as Dimensions>::N>,
    <Lhs as Dimensions>::J: Add<<Rhs as Dimensions>::J>,
    Sum<<Lhs as Dimensions>::T, <Rhs as Dimensions>::T>: Integer,
    Sum<<Lhs as Dimensions>::L, <Rhs as Dimensions>::L>: Integer,
    Sum<<Lhs as Dimensions>::M, <Rhs as Dimensions>::M>: Integer,
    Sum<<Lhs as Dimensions>::I, <Rhs as Dimensions>::I>: Integer,
    Sum<<Lhs as Dimensions>::K, <Rhs as Dimensions>::K>: Integer,
    Sum<<Lhs as Dimensions>::N, <Rhs as Dimensions>::N>: Integer,
    Sum<<Lhs as Dimensions>::J, <Rhs as Dimensions>::J>: Integer,
{
    type Output = DimensionMul<Lhs, Rhs>;
}

/// Helper trait for dimension division operations.
/// Encapsulates all the trait bounds needed for dividing two dimensions.
pub trait CanDivideBy<Rhs: Dimensions>: Dimensions {
    type Output: Dimensions;
}

// Blanket implementation for all valid dimension combinations
impl<Lhs, Rhs> CanDivideBy<Rhs> for Lhs
where
    Lhs: Dimensions,
    Rhs: Dimensions,
    <Lhs as Dimensions>::T: Sub<<Rhs as Dimensions>::T>,
    <Lhs as Dimensions>::L: Sub<<Rhs as Dimensions>::L>,
    <Lhs as Dimensions>::M: Sub<<Rhs as Dimensions>::M>,
    <Lhs as Dimensions>::I: Sub<<Rhs as Dimensions>::I>,
    <Lhs as Dimensions>::K: Sub<<Rhs as Dimensions>::K>,
    <Lhs as Dimensions>::N: Sub<<Rhs as Dimensions>::N>,
    <Lhs as Dimensions>::J: Sub<<Rhs as Dimensions>::J>,
    Diff<<Lhs as Dimensions>::T, <Rhs as Dimensions>::T>: Integer,
    Diff<<Lhs as Dimensions>::L, <Rhs as Dimensions>::L>: Integer,
    Diff<<Lhs as Dimensions>::M, <Rhs as Dimensions>::M>: Integer,
    Diff<<Lhs as Dimensions>::I, <Rhs as Dimensions>::I>: Integer,
    Diff<<Lhs as Dimensions>::K, <Rhs as Dimensions>::K>: Integer,
    Diff<<Lhs as Dimensions>::N, <Rhs as Dimensions>::N>: Integer,
    Diff<<Lhs as Dimensions>::J, <Rhs as Dimensions>::J>: Integer,
{
    type Output = DimensionDiv<Lhs, Rhs>;
}

/// Helper trait for dimensions that can be reciprocated (1/D).
pub trait CanReciprocate: Dimensions {
    type Output: Dimensions;
}

impl<D> CanReciprocate for D
where
    D: Dimensions,
    <D as Dimensions>::T: Neg,
    <D as Dimensions>::L: Neg,
    <D as Dimensions>::M: Neg,
    <D as Dimensions>::I: Neg,
    <D as Dimensions>::K: Neg,
    <D as Dimensions>::N: Neg,
    <D as Dimensions>::J: Neg,
    Negate<<D as Dimensions>::T>: Integer,
    Negate<<D as Dimensions>::L>: Integer,
    Negate<<D as Dimensions>::M>: Integer,
    Negate<<D as Dimensions>::I>: Integer,
    Negate<<D as Dimensions>::K>: Integer,
    Negate<<D as Dimensions>::N>: Integer,
    Negate<<D as Dimensions>::J>: Integer,
{
    type Output = DimensionRecip<D>;
}

/// The dimensionless unit (pure number).
pub type Dimensionless = Dimension<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

pub mod base {
    use super::*;

    /// Time (second, s)
    ///
    /// T
    pub type Time = Dimension<P1, Z0, Z0, Z0, Z0, Z0, Z0>;

    /// Length (metre, m)
    ///
    /// L
    pub type Length = Dimension<Z0, P1, Z0, Z0, Z0, Z0, Z0>;

    /// Mass (kilogram, kg)
    ///
    /// M
    pub type Mass = Dimension<Z0, Z0, P1, Z0, Z0, Z0, Z0>;

    /// Electric current (ampere, A)
    ///
    /// A
    pub type ElectricCurrent = Dimension<Z0, Z0, Z0, P1, Z0, Z0, Z0>;

    /// Thermodynamic temperature (kelvin, K)
    ///
    /// K
    pub type ThermodynamicTemperature = Dimension<Z0, Z0, Z0, Z0, P1, Z0, Z0>;

    /// Amount of substance (mole, mol)
    ///
    /// mol
    pub type AmountOfSubstance = Dimension<Z0, Z0, Z0, Z0, Z0, P1, Z0>;

    /// Luminous intensity (candela, cd)
    ///
    /// cd
    pub type LuminousIntensity = Dimension<Z0, Z0, Z0, Z0, Z0, Z0, P1>;
}

pub mod derived {
    use super::{base::*, *};

    /// Frequency (hertz, Hz)
    ///
    /// T⁻¹
    pub type Frequency = DimensionRecip<Time>;

    /// Velocity (metre per second, m/s)
    ///
    /// L·T⁻¹
    pub type Velocity = DimensionDiv<Length, Time>;

    /// Acceleration (metre per second squared, m/s²)
    ///
    /// L·T⁻²
    pub type Acceleration = DimensionDiv<Velocity, Time>;

    /// Force (newton, N)
    ///
    /// M·L·T⁻²
    pub type Force = DimensionMul<Mass, Acceleration>;

    /// Energy, work, heat (joule, J)
    ///
    /// M·L²·T⁻²
    pub type Energy = DimensionMul<Force, Length>;

    /// Power (watt, W)
    ///
    /// M·L²·T⁻³
    pub type Power = DimensionDiv<Energy, Time>;

    /// Electric potential, voltage (volt, V)
    ///
    /// M·L²·T⁻³·A⁻¹
    pub type Voltage = DimensionDiv<Power, ElectricCurrent>;

    /// Electrical resistance (ohm, Ω)
    ///
    /// M·L²·T⁻³·A⁻²
    pub type Resistance = DimensionDiv<Voltage, ElectricCurrent>;

    /// Electrical conductance (siemens, S)
    ///
    /// M⁻¹·L⁻²·T³·A²
    pub type Conductance = DimensionRecip<Resistance>;

    /// Electric charge (coulomb, C)
    ///
    /// T·A
    pub type ElectricCharge = DimensionMul<Time, ElectricCurrent>;

    /// Electrical capacitance (farad, F)
    ///
    /// M⁻¹·L⁻²·T⁴·A²
    pub type Capacitance = DimensionDiv<ElectricCharge, Voltage>;

    /// Magnetic flux (weber, Wb)
    ///
    /// M·L²·T⁻²·A⁻¹
    pub type MagneticFlux = DimensionMul<Voltage, Time>;

    /// Electrical inductance (henry, H)
    ///
    /// M·L²·T⁻²·A⁻²
    pub type Inductance = DimensionDiv<MagneticFlux, ElectricCurrent>;

    /// Magnetic flux density (tesla, T)
    ///
    /// M·T⁻²·A⁻¹
    pub type MagneticFluxDensity = DimensionDiv<MagneticFlux, DimensionPow<Length, P2>>;

    /// Pressure (pascal, Pa)
    ///
    /// M·L⁻¹·T⁻²
    pub type Pressure = DimensionDiv<Force, DimensionPow<Length, P2>>;
}
