//! Dimensional analysis for compile-time unit checking.
//!
//! This module provides types and constants for representing physical
//! dimensions using the seven SI base units. Dimensions are tracked as
//! exponents of these base units, enabling compile-time verification of unit
//! compatibility.

/// Type alias for dimension representation.
///
/// Each element represent the exponent of the base SI unit:
/// - 0: Time (second, s)
/// - 1: Length (metre, m)
/// - 2: Mass (kilogram, kg)
/// - 3: Electric Current (ampere, A)
/// - 4: Thermodynamic Temperature (kelvin, K)
/// - 5: Amount of Substance (mole, mol)
/// - 6: Luminous Intensity (candela, cd)
pub type DimensionArray = [i8; 7];

/// Array indices for each SI base dimension.
pub mod indices {
    /// Index for time dimension (T)
    pub const TIME: usize = 0;
    /// Index for length dimension (L)
    pub const LENGTH: usize = 1;
    /// Index for mass dimension (M)
    pub const MASS: usize = 2;
    /// Index for electric current dimension (A)
    pub const ELECTRIC_CURRENT: usize = 3;
    /// Index for thermodynamic temperature dimension (K)
    pub const THERMODYNAMIC_TEMPERATURE: usize = 4;
    /// Index for amount of substance dimension (mol)
    pub const AMOUNT_OF_SUBSTANCE: usize = 5;
    /// Index for luminous intensity dimension (cd)
    pub const LUMINOUS_INTENSITY: usize = 6;
}

/// Compact representation of dimensions as a single 128-bit integer.
pub type PackedDimension = u128;

/// Pack a dimension array into a compact 128-bit representation.
pub const fn pack(d: DimensionArray) -> PackedDimension {
    use indices::*;

    (d[TIME] as u8 as u128)
        | ((d[LENGTH] as u8 as u128) << 8)
        | ((d[MASS] as u8 as u128) << 16)
        | ((d[ELECTRIC_CURRENT] as u8 as u128) << 24)
        | ((d[THERMODYNAMIC_TEMPERATURE] as u8 as u128) << 32)
        | ((d[AMOUNT_OF_SUBSTANCE] as u8 as u128) << 40)
        | ((d[LUMINOUS_INTENSITY] as u8 as u128) << 48)
}

/// Unpack a 128-bit representation back into a dimension array.
pub const fn unpack(p: PackedDimension) -> DimensionArray {
    use indices::*;

    let mut d = [0i8; 7];

    d[TIME] = p as u8 as i8;
    d[LENGTH] = (p >> 8) as u8 as i8;
    d[MASS] = (p >> 16) as u8 as i8;
    d[ELECTRIC_CURRENT] = (p >> 24) as u8 as i8;
    d[THERMODYNAMIC_TEMPERATURE] = (p >> 32) as u8 as i8;
    d[AMOUNT_OF_SUBSTANCE] = (p >> 40) as u8 as i8;
    d[LUMINOUS_INTENSITY] = (p >> 48) as u8 as i8;

    d
}

/// Represents a physical dimension as powers of SI base units.
#[derive(Debug, Clone, Copy)]
pub struct Dimension(pub DimensionArray);

impl Dimension {
    /// Create a new dimension from individual exponents.
    pub const fn new(
        time: i8,
        length: i8,
        mass: i8,
        electric_current: i8,
        thermodynamic_temperature: i8,
        amount_of_substance: i8,
        luminous_intensity: i8,
    ) -> Self {
        Self([
            time,
            length,
            mass,
            electric_current,
            thermodynamic_temperature,
            amount_of_substance,
            luminous_intensity,
        ])
    }

    /// Create a dimension from an array of exponents.
    pub const fn from_array(array: DimensionArray) -> Self {
        Self(array)
    }

    /// Create a dimensionless quantity (all exponents zero).
    pub const fn dimensionless() -> Self {
        Self([0; 7])
    }

    /// Convert to packed 128-bit representation.
    pub const fn pack(self) -> PackedDimension {
        pack(self.0)
    }

    /// Add dimensions (multiply physical quantities).
    pub const fn add(self, other: Self) -> Self {
        use indices::*;

        Self([
            self.0[TIME] + other.0[TIME],
            self.0[LENGTH] + other.0[LENGTH],
            self.0[MASS] + other.0[MASS],
            self.0[ELECTRIC_CURRENT] + other.0[ELECTRIC_CURRENT],
            self.0[THERMODYNAMIC_TEMPERATURE] + other.0[THERMODYNAMIC_TEMPERATURE],
            self.0[AMOUNT_OF_SUBSTANCE] + other.0[AMOUNT_OF_SUBSTANCE],
            self.0[LUMINOUS_INTENSITY] + other.0[LUMINOUS_INTENSITY],
        ])
    }

    /// Subtract dimensions (divide physical quantities).
    pub const fn sub(self, other: Self) -> Self {
        use indices::*;

        Self([
            self.0[TIME] - other.0[TIME],
            self.0[LENGTH] - other.0[LENGTH],
            self.0[MASS] - other.0[MASS],
            self.0[ELECTRIC_CURRENT] - other.0[ELECTRIC_CURRENT],
            self.0[THERMODYNAMIC_TEMPERATURE] - other.0[THERMODYNAMIC_TEMPERATURE],
            self.0[AMOUNT_OF_SUBSTANCE] - other.0[AMOUNT_OF_SUBSTANCE],
            self.0[LUMINOUS_INTENSITY] - other.0[LUMINOUS_INTENSITY],
        ])
    }

    /// Raise dimension to a power (physical quantity to a power).
    pub const fn pow(self, n: i8) -> Self {
        use indices::*;

        Self([
            self.0[TIME] * n,
            self.0[LENGTH] * n,
            self.0[MASS] * n,
            self.0[ELECTRIC_CURRENT] * n,
            self.0[THERMODYNAMIC_TEMPERATURE] * n,
            self.0[AMOUNT_OF_SUBSTANCE] * n,
            self.0[LUMINOUS_INTENSITY] * n,
        ])
    }

    /// Multiply dimensions (same as add).
    pub const fn mul(self, other: Self) -> Self {
        self.add(other)
    }

    /// Divide dimensions (same as sub).
    pub const fn div(self, other: Self) -> Self {
        self.sub(other)
    }

    /// Get reciprocal dimension (1/dimension).
    pub const fn recip(self) -> Self {
        self.pow(-1)
    }

    /// Check if this is a dimensionless quantity.
    pub const fn is_dimensionless(self) -> bool {
        use indices::*;

        self.0[TIME] == 0
            && self.0[LENGTH] == 0
            && self.0[MASS] == 0
            && self.0[ELECTRIC_CURRENT] == 0
            && self.0[THERMODYNAMIC_TEMPERATURE] == 0
            && self.0[AMOUNT_OF_SUBSTANCE] == 0
            && self.0[LUMINOUS_INTENSITY] == 0
    }

    /// Get the underlying array representation.
    pub const fn as_array(self) -> DimensionArray {
        self.0
    }

    /// Check equality with a dimension array.
    pub const fn eq_array(self, other: DimensionArray) -> bool {
        use indices::*;

        self.0[TIME] == other[TIME]
            && self.0[LENGTH] == other[LENGTH]
            && self.0[MASS] == other[MASS]
            && self.0[ELECTRIC_CURRENT] == other[ELECTRIC_CURRENT]
            && self.0[THERMODYNAMIC_TEMPERATURE] == other[THERMODYNAMIC_TEMPERATURE]
            && self.0[AMOUNT_OF_SUBSTANCE] == other[AMOUNT_OF_SUBSTANCE]
            && self.0[LUMINOUS_INTENSITY] == other[LUMINOUS_INTENSITY]
    }
}

/// Operations on dimension arrays.
pub mod array_ops {
    use super::{DimensionArray, indices::*};

    /// Add two dimension arrays (multiply quantities).
    pub const fn add(a: DimensionArray, b: DimensionArray) -> DimensionArray {
        [
            a[TIME] + b[TIME],
            a[LENGTH] + b[LENGTH],
            a[MASS] + b[MASS],
            a[ELECTRIC_CURRENT] + b[ELECTRIC_CURRENT],
            a[THERMODYNAMIC_TEMPERATURE] + b[THERMODYNAMIC_TEMPERATURE],
            a[AMOUNT_OF_SUBSTANCE] + b[AMOUNT_OF_SUBSTANCE],
            a[LUMINOUS_INTENSITY] + b[LUMINOUS_INTENSITY],
        ]
    }

    /// Subtract two dimension arrays (divide quantities).
    pub const fn sub(a: DimensionArray, b: DimensionArray) -> DimensionArray {
        [
            a[TIME] - b[TIME],
            a[LENGTH] - b[LENGTH],
            a[MASS] - b[MASS],
            a[ELECTRIC_CURRENT] - b[ELECTRIC_CURRENT],
            a[THERMODYNAMIC_TEMPERATURE] - b[THERMODYNAMIC_TEMPERATURE],
            a[AMOUNT_OF_SUBSTANCE] - b[AMOUNT_OF_SUBSTANCE],
            a[LUMINOUS_INTENSITY] - b[LUMINOUS_INTENSITY],
        ]
    }

    /// Raise dimension array to a power.
    pub const fn pow(d: DimensionArray, n: i8) -> DimensionArray {
        [
            d[TIME] * n,
            d[LENGTH] * n,
            d[MASS] * n,
            d[ELECTRIC_CURRENT] * n,
            d[THERMODYNAMIC_TEMPERATURE] * n,
            d[AMOUNT_OF_SUBSTANCE] * n,
            d[LUMINOUS_INTENSITY] * n,
        ]
    }

    /// Multiply dimension arrays (same as add).
    pub const fn mul(a: DimensionArray, b: DimensionArray) -> DimensionArray {
        add(a, b)
    }

    /// Divide dimension arrays (same as sub).
    pub const fn div(a: DimensionArray, b: DimensionArray) -> DimensionArray {
        sub(a, b)
    }

    /// Get reciprocal of dimension array.
    pub const fn recip(d: DimensionArray) -> DimensionArray {
        pow(d, -1)
    }
}

/// Operations on packed dimensions.
pub mod packed_ops {
    use super::{PackedDimension, array_ops, pack, unpack};

    /// Add two packed dimensions.
    pub const fn add(a: PackedDimension, b: PackedDimension) -> PackedDimension {
        pack(array_ops::add(unpack(a), unpack(b)))
    }

    /// Subtract two packed dimensions.
    pub const fn sub(a: PackedDimension, b: PackedDimension) -> PackedDimension {
        pack(array_ops::sub(unpack(a), unpack(b)))
    }

    /// Raise packed dimension to a power.
    pub const fn pow(d: PackedDimension, n: i8) -> PackedDimension {
        pack(array_ops::pow(unpack(d), n))
    }

    /// Multiply packed dimensions.
    pub const fn mul(a: PackedDimension, b: PackedDimension) -> PackedDimension {
        add(a, b)
    }

    /// Divide packed dimensions.
    pub const fn div(a: PackedDimension, b: PackedDimension) -> PackedDimension {
        sub(a, b)
    }

    /// Get reciprocal of packed dimension.
    pub const fn recip(d: PackedDimension) -> PackedDimension {
        pow(d, -1)
    }
}

/// The dimensionless unit (pure number).
pub const DIMENSIONLESS: Dimension = Dimension::dimensionless();
const _: () = {
    assert!(DIMENSIONLESS.is_dimensionless());
};

// Base dimensions

/// Time (second, s)
///
/// T
pub const TIME: Dimension = Dimension::new(1, 0, 0, 0, 0, 0, 0);

/// Length (metre, m)
///
/// L
pub const LENGTH: Dimension = Dimension::new(0, 1, 0, 0, 0, 0, 0);

/// Mass (kilogram, kg)
///
/// M
pub const MASS: Dimension = Dimension::new(0, 0, 1, 0, 0, 0, 0);

/// Electric current (ampere, A)
///
/// A
pub const ELECTRIC_CURRENT: Dimension = Dimension::new(0, 0, 0, 1, 0, 0, 0);

/// Thermodynamic temperature (kelvin, K)
///
/// K
pub const THERMODYNAMIC_TEMPERATURE: Dimension = Dimension::new(0, 0, 0, 0, 1, 0, 0);

/// Amount of substance (mole, mol)
///
/// mol
pub const AMOUNT_OF_SUBSTANCE: Dimension = Dimension::new(0, 0, 0, 0, 0, 1, 0);

/// Luminous intensity (candela, cd)
///
/// cd
pub const LUMINOUS_INTENSITY: Dimension = Dimension::new(0, 0, 0, 0, 0, 0, 1);

// Derived dimensions

/// Frequency (hertz, Hz)
///
/// T⁻¹
pub const FREQUENCY: Dimension = TIME.recip();
const _: () = {
    assert!(FREQUENCY.eq_array([-1, 0, 0, 0, 0, 0, 0]));
};

/// Velocity (metre per second, m/s)
///
/// L·T⁻¹
pub const VELOCITY: Dimension = LENGTH.div(TIME);
const _: () = {
    assert!(VELOCITY.eq_array([-1, 1, 0, 0, 0, 0, 0]));
};

/// Acceleration (metre per second squared, m/s²)
///
/// L·T⁻²
pub const ACCELERATION: Dimension = VELOCITY.div(TIME);
const _: () = {
    assert!(ACCELERATION.eq_array([-2, 1, 0, 0, 0, 0, 0]));
};

/// Force (newton, N)
///
/// M·L·T⁻²
pub const FORCE: Dimension = MASS.mul(ACCELERATION);
const _: () = {
    assert!(FORCE.eq_array([-2, 1, 1, 0, 0, 0, 0]));
};

/// Energy, work, heat (joule, J)
///
/// M·L²·T⁻²
pub const ENERGY: Dimension = FORCE.mul(LENGTH);
const _: () = {
    assert!(ENERGY.eq_array([-2, 2, 1, 0, 0, 0, 0]));
};

/// Power (watt, W)
///
/// M·L²·T⁻³
pub const POWER: Dimension = ENERGY.div(TIME);
const _: () = {
    assert!(POWER.eq_array([-3, 2, 1, 0, 0, 0, 0]));
};

/// Electric potential, voltage (volt, V)
///
/// M·L²·T⁻³·A⁻¹
pub const VOLTAGE: Dimension = POWER.div(ELECTRIC_CURRENT);
const _: () = {
    assert!(VOLTAGE.eq_array([-3, 2, 1, -1, 0, 0, 0]));
};

/// Electrical resistance (ohm, Ω)
///
/// M·L²·T⁻³·A⁻²
pub const RESISTANCE: Dimension = VOLTAGE.div(ELECTRIC_CURRENT);
const _: () = {
    assert!(RESISTANCE.eq_array([-3, 2, 1, -2, 0, 0, 0]));
};

/// Electrical conductance (siemens, S)
///
/// M⁻¹·L⁻²·T³·A²
pub const CONDUCTANCE: Dimension = RESISTANCE.recip();
const _: () = {
    assert!(CONDUCTANCE.eq_array([3, -2, -1, 2, 0, 0, 0]));
};

/// Electric charge (coulomb, C)
///
/// T·A
pub const ELECTRIC_CHARGE: Dimension = TIME.mul(ELECTRIC_CURRENT);
const _: () = {
    assert!(ELECTRIC_CHARGE.eq_array([1, 0, 0, 1, 0, 0, 0]));
};

/// Electrical capacitance (farad, F)
///
/// M⁻¹·L⁻²·T⁴·A²
pub const CAPACITANCE: Dimension = ELECTRIC_CHARGE.div(VOLTAGE);
const _: () = {
    assert!(CAPACITANCE.eq_array([4, -2, -1, 2, 0, 0, 0]));
};

/// Magnetic flux (weber, Wb)
///
/// M·L²·T⁻²·A⁻¹
pub const MAGNETIC_FLUX: Dimension = VOLTAGE.mul(TIME);
const _: () = {
    assert!(MAGNETIC_FLUX.eq_array([-2, 2, 1, -1, 0, 0, 0]));
};

/// Electrical inductance (henry, H)
///
/// M·L²·T⁻²·A⁻²
pub const INDUCTANCE: Dimension = MAGNETIC_FLUX.div(ELECTRIC_CURRENT);
const _: () = {
    assert!(INDUCTANCE.eq_array([-2, 2, 1, -2, 0, 0, 0]));
};

/// Magnetic flux density (tesla, T)
///
/// M·T⁻²·A⁻¹
pub const MAGNETIC_FLUX_DENSITY: Dimension = MAGNETIC_FLUX.div(LENGTH.pow(2));
const _: () = {
    assert!(MAGNETIC_FLUX_DENSITY.eq_array([-2, 0, 1, -1, 0, 0, 0]));
};
