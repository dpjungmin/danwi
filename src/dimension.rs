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

pub mod indices {
    pub const TIME: usize = 0;
    pub const LENGTH: usize = 1;
    pub const MASS: usize = 2;
    pub const ELECTRIC_CURRENT: usize = 3;
    pub const THERMODYNAMIC_TEMPERATURE: usize = 4;
    pub const AMOUNT_OF_SUBSTANCE: usize = 5;
    pub const LUMINOUS_INTENSITY: usize = 6;
}

#[derive(Debug, Clone, Copy)]
pub struct Dimension(pub DimensionArray);

impl Dimension {
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

    pub const fn from_array(array: DimensionArray) -> Self {
        Self(array)
    }

    pub const fn dimensionless() -> Self {
        Self([0; 7])
    }

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

    pub const fn mul(self, other: Self) -> Self {
        self.add(other)
    }

    pub const fn div(self, other: Self) -> Self {
        self.sub(other)
    }

    pub const fn recip(self) -> Self {
        self.pow(-1)
    }

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

    pub const fn as_array(self) -> DimensionArray {
        self.0
    }

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

pub mod array_ops {
    use super::{DimensionArray, indices::*};

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

    pub const fn mul(a: DimensionArray, b: DimensionArray) -> DimensionArray {
        add(a, b)
    }

    pub const fn div(a: DimensionArray, b: DimensionArray) -> DimensionArray {
        sub(a, b)
    }

    pub const fn recip(d: DimensionArray) -> DimensionArray {
        pow(d, -1)
    }
}

pub const DIMENSIONLESS: Dimension = Dimension::dimensionless();
const _: () = {
    assert!(DIMENSIONLESS.is_dimensionless());
};

// Base dimensions

pub const TIME: Dimension = Dimension::new(1, 0, 0, 0, 0, 0, 0);
pub const LENGTH: Dimension = Dimension::new(0, 1, 0, 0, 0, 0, 0);
pub const MASS: Dimension = Dimension::new(0, 0, 1, 0, 0, 0, 0);
pub const ELECTRIC_CURRENT: Dimension = Dimension::new(0, 0, 0, 1, 0, 0, 0);
pub const THERMODYNAMIC_TEMPERATURE: Dimension = Dimension::new(0, 0, 0, 0, 1, 0, 0);
pub const AMOUNT_OF_SUBSTANCE: Dimension = Dimension::new(0, 0, 0, 0, 0, 1, 0);
pub const LUMINOUS_INTENSITY: Dimension = Dimension::new(0, 0, 0, 0, 0, 0, 1);

// Derived dimensions

/// T⁻¹
pub const FREQUENCY: Dimension = TIME.recip();
const _: () = {
    assert!(FREQUENCY.eq_array([-1, 0, 0, 0, 0, 0, 0]));
};

/// L·T⁻¹
pub const VELOCITY: Dimension = LENGTH.div(TIME);
const _: () = {
    assert!(VELOCITY.eq_array([-1, 1, 0, 0, 0, 0, 0]));
};

/// L·T⁻²
pub const ACCELERATION: Dimension = VELOCITY.div(TIME);
const _: () = {
    assert!(ACCELERATION.eq_array([-2, 1, 0, 0, 0, 0, 0]));
};

/// M·L·T⁻²
pub const FORCE: Dimension = MASS.mul(ACCELERATION);
const _: () = {
    assert!(FORCE.eq_array([-2, 1, 1, 0, 0, 0, 0]));
};

/// M·L²·T⁻²
pub const ENERGY: Dimension = FORCE.mul(LENGTH);
const _: () = {
    assert!(ENERGY.eq_array([-2, 2, 1, 0, 0, 0, 0]));
};

/// M·L²·T⁻³
pub const POWER: Dimension = ENERGY.div(TIME);
const _: () = {
    assert!(POWER.eq_array([-3, 2, 1, 0, 0, 0, 0]));
};

/// M·L²·T⁻³·A⁻¹
pub const VOLTAGE: Dimension = POWER.div(ELECTRIC_CURRENT);
const _: () = {
    assert!(VOLTAGE.eq_array([-3, 2, 1, -1, 0, 0, 0]));
};

/// M·L²·T⁻³·A⁻²
pub const RESISTANCE: Dimension = VOLTAGE.div(ELECTRIC_CURRENT);
const _: () = {
    assert!(RESISTANCE.eq_array([-3, 2, 1, -2, 0, 0, 0]));
};
