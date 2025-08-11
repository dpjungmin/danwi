//! Dimensional analysis for the SI system.
//!
//! This module provides compile-time dimensional analysis through the
//! [`Dimensions`] struct, which tracks the exponents of the seven SI base
//! quantities.

/// Represents the dimensional exponents for the seven SI base quantities.
///
/// This struct stores the dimensional exponents used in dimensional analysis.
/// Each field represents the power to which that base quantity is raised.
///
/// References:
/// - <https://www.bipm.org/en/measurement-units/si-base-units>
/// - <https://en.wikipedia.org/wiki/International_System_of_Units>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    /// Exponent for time dimension \[T\].
    pub time: i8,
    /// Exponent for length dimension \[L\].
    pub length: i8,
    /// Exponent for mass dimension \[M\].
    pub mass: i8,
    /// Exponent for electric current dimension \[I\].
    pub electric_current: i8,
    /// Exponent for thermodynamic temperature dimension \[Θ\].
    pub thermodynamic_temperature: i8,
    /// Exponent for amount of substance dimension \[N\].
    pub amount_of_substance: i8,
    /// Exponent for luminous intensity dimension \[J\].
    pub luminous_intensity: i8,
}

impl Dimensions {
    /// Creates a dimensionless quantity.
    ///
    /// Returns a `Dimensions` with all exponents set to zero.
    pub const fn dimensionless() -> Self {
        Self {
            time: 0,
            length: 0,
            mass: 0,
            electric_current: 0,
            thermodynamic_temperature: 0,
            amount_of_substance: 0,
            luminous_intensity: 0,
        }
    }

    /// Multiplies two dimensions.
    ///
    /// Returns a new `Dimensions` with exponents that are the sum of the
    /// corresponding exponents from `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::dimensions::Dimensions;
    /// let mut a = Dimensions::dimensionless();
    /// let mut b = Dimensions::dimensionless();
    ///
    /// a.time = 1;
    /// b.time = 1;
    /// b.length = 1;
    ///
    /// let c = a.mul(b);
    ///
    /// assert_eq!(c.time, 2);
    /// assert_eq!(c.length, 1);
    /// assert_eq!(c.mass, 0);
    /// assert_eq!(c.electric_current, 0);
    /// assert_eq!(c.thermodynamic_temperature, 0);
    /// assert_eq!(c.amount_of_substance, 0);
    /// assert_eq!(c.luminous_intensity, 0);
    /// ```
    pub const fn mul(self, other: Self) -> Self {
        Self {
            time: self.time + other.time,
            length: self.length + other.length,
            mass: self.mass + other.mass,
            electric_current: self.electric_current + other.electric_current,
            thermodynamic_temperature: self.thermodynamic_temperature
                + other.thermodynamic_temperature,
            amount_of_substance: self.amount_of_substance + other.amount_of_substance,
            luminous_intensity: self.luminous_intensity + other.luminous_intensity,
        }
    }

    /// Divides two dimensions.
    ///
    /// Returns a new `Dimensions` with exponents that are the subtraction of
    /// the corresponding exponents from `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::dimensions::Dimensions;
    /// let mut a = Dimensions::dimensionless();
    /// let mut b = Dimensions::dimensionless();
    ///
    /// a.time = 1;
    /// b.time = 1;
    /// b.length = 1;
    ///
    /// let c = a.div(b);
    ///
    /// assert_eq!(c.time, 0);
    /// assert_eq!(c.length, -1);
    /// assert_eq!(c.mass, 0);
    /// assert_eq!(c.electric_current, 0);
    /// assert_eq!(c.thermodynamic_temperature, 0);
    /// assert_eq!(c.amount_of_substance, 0);
    /// assert_eq!(c.luminous_intensity, 0);
    /// ```
    pub const fn div(self, other: Self) -> Self {
        Self {
            time: self.time - other.time,
            length: self.length - other.length,
            mass: self.mass - other.mass,
            electric_current: self.electric_current - other.electric_current,
            thermodynamic_temperature: self.thermodynamic_temperature
                - other.thermodynamic_temperature,
            amount_of_substance: self.amount_of_substance - other.amount_of_substance,
            luminous_intensity: self.luminous_intensity - other.luminous_intensity,
        }
    }

    /// Raises dimensions to a power.
    ///
    /// Returns a new `Dimensions` with all exponents multiplied by `n`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::dimensions::Dimensions;
    /// let mut a = Dimensions::dimensionless();
    ///
    /// a.time = 1;
    /// a.length = 2;
    ///
    /// let b = a.pow(2);
    ///
    /// assert_eq!(b.time, 2);
    /// assert_eq!(b.length, 4);
    /// assert_eq!(b.mass, 0);
    /// assert_eq!(b.electric_current, 0);
    /// assert_eq!(b.thermodynamic_temperature, 0);
    /// assert_eq!(b.amount_of_substance, 0);
    /// assert_eq!(b.luminous_intensity, 0);
    /// ```
    pub const fn pow(self, n: i8) -> Self {
        Self {
            time: self.time * n,
            length: self.length * n,
            mass: self.mass * n,
            electric_current: self.electric_current * n,
            thermodynamic_temperature: self.thermodynamic_temperature * n,
            amount_of_substance: self.amount_of_substance * n,
            luminous_intensity: self.luminous_intensity * n,
        }
    }

    /// Returns the reciprocal of dimensions.
    ///
    /// Equivalent to `self.pow(-1)`. All exponents are negated.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::dimensions::Dimensions;
    /// let mut a = Dimensions::dimensionless();
    ///
    /// a.time = 1;
    /// a.length = 2;
    ///
    /// let b = a.recip();
    ///
    /// assert_eq!(b.time, -1);
    /// assert_eq!(b.length, -2);
    /// assert_eq!(b.mass, 0);
    /// assert_eq!(b.electric_current, 0);
    /// assert_eq!(b.thermodynamic_temperature, 0);
    /// assert_eq!(b.amount_of_substance, 0);
    /// assert_eq!(b.luminous_intensity, 0);
    /// ```
    pub const fn recip(self) -> Self {
        self.pow(-1)
    }

    /// Checks if this represents a dimensionless quantity.
    ///
    /// Returns `true` if all exponents are zero.
    pub const fn is_dimensionless(&self) -> bool {
        self.time == 0
            && self.length == 0
            && self.mass == 0
            && self.electric_current == 0
            && self.thermodynamic_temperature == 0
            && self.amount_of_substance == 0
            && self.luminous_intensity == 0
    }
}

/// SI base dimension constants.
///
/// This module provides dimension constants for the seven SI base quantities.
/// Each constant has exactly one exponent set to 1, with all others set to 0.
pub mod base {
    use super::Dimensions;

    /// Time dimension \[T\].
    pub const TIME: Dimensions = Dimensions {
        time: 1,
        length: 0,
        mass: 0,
        electric_current: 0,
        thermodynamic_temperature: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };
    const _: () = {
        assert!(TIME.time == 1);
        assert!(TIME.length == 0);
        assert!(TIME.mass == 0);
        assert!(TIME.electric_current == 0);
        assert!(TIME.thermodynamic_temperature == 0);
        assert!(TIME.amount_of_substance == 0);
        assert!(TIME.luminous_intensity == 0);
    };

    /// Length dimension \[L\].
    pub const LENGTH: Dimensions = Dimensions {
        time: 0,
        length: 1,
        mass: 0,
        electric_current: 0,
        thermodynamic_temperature: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };
    const _: () = {
        assert!(LENGTH.time == 0);
        assert!(LENGTH.length == 1);
        assert!(LENGTH.mass == 0);
        assert!(LENGTH.electric_current == 0);
        assert!(LENGTH.thermodynamic_temperature == 0);
        assert!(LENGTH.amount_of_substance == 0);
        assert!(LENGTH.luminous_intensity == 0);
    };

    /// Mass dimension \[M\].
    pub const MASS: Dimensions = Dimensions {
        time: 0,
        length: 0,
        mass: 1,
        electric_current: 0,
        thermodynamic_temperature: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };
    const _: () = {
        assert!(MASS.time == 0);
        assert!(MASS.length == 0);
        assert!(MASS.mass == 1);
        assert!(MASS.electric_current == 0);
        assert!(MASS.thermodynamic_temperature == 0);
        assert!(MASS.amount_of_substance == 0);
        assert!(MASS.luminous_intensity == 0);
    };

    /// Electric current dimension \[I\].
    pub const ELECTRIC_CURRENT: Dimensions = Dimensions {
        time: 0,
        length: 0,
        mass: 0,
        electric_current: 1,
        thermodynamic_temperature: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };
    const _: () = {
        assert!(ELECTRIC_CURRENT.time == 0);
        assert!(ELECTRIC_CURRENT.length == 0);
        assert!(ELECTRIC_CURRENT.mass == 0);
        assert!(ELECTRIC_CURRENT.electric_current == 1);
        assert!(ELECTRIC_CURRENT.thermodynamic_temperature == 0);
        assert!(ELECTRIC_CURRENT.amount_of_substance == 0);
        assert!(ELECTRIC_CURRENT.luminous_intensity == 0);
    };

    /// Thermodynamic temperature dimension \[Θ\].
    pub const THERMODYNAMIC_TEMPERATURE: Dimensions = Dimensions {
        time: 0,
        length: 0,
        mass: 0,
        electric_current: 0,
        thermodynamic_temperature: 1,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };
    const _: () = {
        assert!(THERMODYNAMIC_TEMPERATURE.time == 0);
        assert!(THERMODYNAMIC_TEMPERATURE.length == 0);
        assert!(THERMODYNAMIC_TEMPERATURE.mass == 0);
        assert!(THERMODYNAMIC_TEMPERATURE.electric_current == 0);
        assert!(THERMODYNAMIC_TEMPERATURE.thermodynamic_temperature == 1);
        assert!(THERMODYNAMIC_TEMPERATURE.amount_of_substance == 0);
        assert!(THERMODYNAMIC_TEMPERATURE.luminous_intensity == 0);
    };

    /// Amount of substance dimension \[N\].
    pub const AMOUNT_OF_SUBSTANCE: Dimensions = Dimensions {
        time: 0,
        length: 0,
        mass: 0,
        electric_current: 0,
        thermodynamic_temperature: 0,
        amount_of_substance: 1,
        luminous_intensity: 0,
    };
    const _: () = {
        assert!(AMOUNT_OF_SUBSTANCE.time == 0);
        assert!(AMOUNT_OF_SUBSTANCE.length == 0);
        assert!(AMOUNT_OF_SUBSTANCE.mass == 0);
        assert!(AMOUNT_OF_SUBSTANCE.electric_current == 0);
        assert!(AMOUNT_OF_SUBSTANCE.thermodynamic_temperature == 0);
        assert!(AMOUNT_OF_SUBSTANCE.amount_of_substance == 1);
        assert!(AMOUNT_OF_SUBSTANCE.luminous_intensity == 0);
    };

    /// Luminous intensity dimension \[J\].
    pub const LUMINOUS_INTENSITY: Dimensions = Dimensions {
        time: 0,
        length: 0,
        mass: 0,
        electric_current: 0,
        thermodynamic_temperature: 0,
        amount_of_substance: 0,
        luminous_intensity: 1,
    };
    const _: () = {
        assert!(LUMINOUS_INTENSITY.time == 0);
        assert!(LUMINOUS_INTENSITY.length == 0);
        assert!(LUMINOUS_INTENSITY.mass == 0);
        assert!(LUMINOUS_INTENSITY.electric_current == 0);
        assert!(LUMINOUS_INTENSITY.thermodynamic_temperature == 0);
        assert!(LUMINOUS_INTENSITY.amount_of_substance == 0);
        assert!(LUMINOUS_INTENSITY.luminous_intensity == 1);
    };
}

/// Common derived dimension constants.
///
/// This module provides dimension constants for commonly used derived
/// quantities.
pub mod derived {
    use super::Dimensions;
    use super::base::*;

    /// Dimensionless quantity.
    pub const DIMENSIONLESS: Dimensions = Dimensions::dimensionless();
    const _: () = {
        assert!(DIMENSIONLESS.time == 0);
        assert!(DIMENSIONLESS.length == 0);
        assert!(DIMENSIONLESS.mass == 0);
        assert!(DIMENSIONLESS.electric_current == 0);
        assert!(DIMENSIONLESS.thermodynamic_temperature == 0);
        assert!(DIMENSIONLESS.amount_of_substance == 0);
        assert!(DIMENSIONLESS.luminous_intensity == 0);
    };

    /// Frequency dimension \[T⁻¹\].
    pub const FREQUENCY: Dimensions = TIME.recip();
    const _: () = {
        assert!(FREQUENCY.time == -1);
        assert!(FREQUENCY.length == 0);
        assert!(FREQUENCY.mass == 0);
        assert!(FREQUENCY.electric_current == 0);
        assert!(FREQUENCY.thermodynamic_temperature == 0);
        assert!(FREQUENCY.amount_of_substance == 0);
        assert!(FREQUENCY.luminous_intensity == 0);
    };

    /// Area dimension \[L²\].
    pub const AREA: Dimensions = LENGTH.pow(2);
    const _: () = {
        assert!(AREA.time == 0);
        assert!(AREA.length == 2);
        assert!(AREA.mass == 0);
        assert!(AREA.electric_current == 0);
        assert!(AREA.thermodynamic_temperature == 0);
        assert!(AREA.amount_of_substance == 0);
        assert!(AREA.luminous_intensity == 0);
    };

    /// Volume dimension \[L³\].
    pub const VOLUME: Dimensions = LENGTH.pow(3);
    const _: () = {
        assert!(VOLUME.time == 0);
        assert!(VOLUME.length == 3);
        assert!(VOLUME.mass == 0);
        assert!(VOLUME.electric_current == 0);
        assert!(VOLUME.thermodynamic_temperature == 0);
        assert!(VOLUME.amount_of_substance == 0);
        assert!(VOLUME.luminous_intensity == 0);
    };

    /// Velocity dimension \[LT⁻¹\].
    pub const VELOCITY: Dimensions = LENGTH.div(TIME);
    const _: () = {
        assert!(VELOCITY.time == -1);
        assert!(VELOCITY.length == 1);
        assert!(VELOCITY.mass == 0);
        assert!(VELOCITY.electric_current == 0);
        assert!(VELOCITY.thermodynamic_temperature == 0);
        assert!(VELOCITY.amount_of_substance == 0);
        assert!(VELOCITY.luminous_intensity == 0);
    };

    /// Acceleration dimension \[LT⁻²\].
    pub const ACCELERATION: Dimensions = VELOCITY.div(TIME);
    const _: () = {
        assert!(ACCELERATION.time == -2);
        assert!(ACCELERATION.length == 1);
        assert!(ACCELERATION.mass == 0);
        assert!(ACCELERATION.electric_current == 0);
        assert!(ACCELERATION.thermodynamic_temperature == 0);
        assert!(ACCELERATION.amount_of_substance == 0);
        assert!(ACCELERATION.luminous_intensity == 0);
    };

    /// Force dimension \[MLT⁻²\].
    pub const FORCE: Dimensions = MASS.mul(ACCELERATION);
    const _: () = {
        assert!(FORCE.time == -2);
        assert!(FORCE.length == 1);
        assert!(FORCE.mass == 1);
        assert!(FORCE.electric_current == 0);
        assert!(FORCE.thermodynamic_temperature == 0);
        assert!(FORCE.amount_of_substance == 0);
        assert!(FORCE.luminous_intensity == 0);
    };

    /// Energy dimension \[ML²T⁻²\].
    pub const ENERGY: Dimensions = FORCE.mul(LENGTH);
    const _: () = {
        assert!(ENERGY.time == -2);
        assert!(ENERGY.length == 2);
        assert!(ENERGY.mass == 1);
        assert!(ENERGY.electric_current == 0);
        assert!(ENERGY.thermodynamic_temperature == 0);
        assert!(ENERGY.amount_of_substance == 0);
        assert!(ENERGY.luminous_intensity == 0);
    };

    /// Power dimension \[ML²T⁻³\].
    pub const POWER: Dimensions = ENERGY.div(TIME);
    const _: () = {
        assert!(POWER.time == -3);
        assert!(POWER.length == 2);
        assert!(POWER.mass == 1);
        assert!(POWER.electric_current == 0);
        assert!(POWER.thermodynamic_temperature == 0);
        assert!(POWER.amount_of_substance == 0);
        assert!(POWER.luminous_intensity == 0);
    };

    /// Pressure dimension \[ML⁻¹T⁻²\].
    pub const PRESSURE: Dimensions = FORCE.div(AREA);
    const _: () = {
        assert!(PRESSURE.time == -2);
        assert!(PRESSURE.length == -1);
        assert!(PRESSURE.mass == 1);
        assert!(PRESSURE.electric_current == 0);
        assert!(PRESSURE.thermodynamic_temperature == 0);
        assert!(PRESSURE.amount_of_substance == 0);
        assert!(PRESSURE.luminous_intensity == 0);
    };

    /// Electric charge dimension \[IT\].
    pub const CHARGE: Dimensions = ELECTRIC_CURRENT.mul(TIME);
    const _: () = {
        assert!(CHARGE.time == 1);
        assert!(CHARGE.length == 0);
        assert!(CHARGE.mass == 0);
        assert!(CHARGE.electric_current == 1);
        assert!(CHARGE.thermodynamic_temperature == 0);
        assert!(CHARGE.amount_of_substance == 0);
        assert!(CHARGE.luminous_intensity == 0);
    };

    /// Electric potential dimension \[ML²T⁻³I⁻¹\].
    pub const VOLTAGE: Dimensions = POWER.div(ELECTRIC_CURRENT);
    const _: () = {
        assert!(VOLTAGE.time == -3);
        assert!(VOLTAGE.length == 2);
        assert!(VOLTAGE.mass == 1);
        assert!(VOLTAGE.electric_current == -1);
        assert!(VOLTAGE.thermodynamic_temperature == 0);
        assert!(VOLTAGE.amount_of_substance == 0);
        assert!(VOLTAGE.luminous_intensity == 0);
    };

    /// Electric resistance dimension \[ML²T⁻³I⁻²\].
    pub const RESISTANCE: Dimensions = VOLTAGE.div(ELECTRIC_CURRENT);
    const _: () = {
        assert!(RESISTANCE.time == -3);
        assert!(RESISTANCE.length == 2);
        assert!(RESISTANCE.mass == 1);
        assert!(RESISTANCE.electric_current == -2);
        assert!(RESISTANCE.thermodynamic_temperature == 0);
        assert!(RESISTANCE.amount_of_substance == 0);
        assert!(RESISTANCE.luminous_intensity == 0);
    };
}

#[cfg(test)]
mod tests {
    use super::base::*;
    use super::derived::*;
    use super::*;

    #[test]
    fn multiplication_is_applied_to_all_quantities() {
        let a = Dimensions {
            time: 1,
            length: 2,
            mass: 3,
            electric_current: 4,
            thermodynamic_temperature: 5,
            amount_of_substance: 6,
            luminous_intensity: 7,
        };
        let b = Dimensions {
            time: 1,
            length: 2,
            mass: 3,
            electric_current: 4,
            thermodynamic_temperature: 5,
            amount_of_substance: 6,
            luminous_intensity: 7,
        };
        let c = a.mul(b);
        assert_eq!(c.time, 2);
        assert_eq!(c.length, 4);
        assert_eq!(c.mass, 6);
        assert_eq!(c.electric_current, 8);
        assert_eq!(c.thermodynamic_temperature, 10);
        assert_eq!(c.amount_of_substance, 12);
        assert_eq!(c.luminous_intensity, 14);
    }

    #[test]
    fn division_is_applied_to_all_quantities() {
        let a = Dimensions {
            time: 1,
            length: 2,
            mass: 3,
            electric_current: 4,
            thermodynamic_temperature: 5,
            amount_of_substance: 6,
            luminous_intensity: 7,
        };
        let b = Dimensions {
            time: 0,
            length: 1,
            mass: 2,
            electric_current: 3,
            thermodynamic_temperature: 4,
            amount_of_substance: 5,
            luminous_intensity: 6,
        };
        let c = a.div(b);
        assert_eq!(c.time, 1);
        assert_eq!(c.length, 1);
        assert_eq!(c.mass, 1);
        assert_eq!(c.electric_current, 1);
        assert_eq!(c.thermodynamic_temperature, 1);
        assert_eq!(c.amount_of_substance, 1);
        assert_eq!(c.luminous_intensity, 1);
    }

    #[test]
    fn multiplication_is_associative() {
        // (A * B) * C = A * (B * C)
        let a = (FORCE.mul(VELOCITY)).mul(TIME);
        let b = FORCE.mul(VELOCITY.mul(TIME));
        assert_eq!(a, b);
    }

    #[test]
    fn multiplication_is_commutative() {
        // A * B = B * A
        assert_eq!(LENGTH.mul(TIME), TIME.mul(LENGTH));
        assert_eq!(MASS.mul(VELOCITY), VELOCITY.mul(MASS));
        assert_eq!(FORCE.mul(LENGTH), LENGTH.mul(FORCE));
    }

    #[test]
    fn dimensionless_is_multiplicative_identity() {
        // A * 1 = A
        assert_eq!(LENGTH.mul(DIMENSIONLESS), LENGTH);
        assert_eq!(FORCE.mul(DIMENSIONLESS), FORCE);
        assert_eq!(ENERGY.mul(DIMENSIONLESS), ENERGY);
        assert_eq!(VOLTAGE.mul(DIMENSIONLESS), VOLTAGE);
    }

    #[test]
    fn reciprocal_is_multiplicative_inverse() {
        // A * A^-1 = 1
        assert!(LENGTH.mul(LENGTH.recip()).is_dimensionless());
        assert!(FORCE.mul(FORCE.recip()).is_dimensionless());
        assert!(ENERGY.mul(ENERGY.recip()).is_dimensionless());
        assert!(VOLTAGE.mul(VOLTAGE.recip()).is_dimensionless());
        assert!(ACCELERATION.mul(ACCELERATION.recip()).is_dimensionless());
    }

    #[test]
    fn division_by_self_yields_dimensionless() {
        // A / A = 1
        assert!(LENGTH.div(LENGTH).is_dimensionless());
        assert!(ENERGY.div(ENERGY).is_dimensionless());
        assert!(FORCE.div(FORCE).is_dimensionless());
        assert!(PRESSURE.div(PRESSURE).is_dimensionless());
        assert!(CHARGE.div(CHARGE).is_dimensionless());
    }

    #[test]
    fn power_of_power_multiplies_exponents() {
        // (A^n)^m = A^(n * m)
        assert_eq!(LENGTH.pow(2).pow(3), LENGTH.pow(6));
        assert_eq!(TIME.pow(3).pow(2), TIME.pow(6));
        assert_eq!(MASS.pow(2).pow(-2), MASS.pow(-4));
    }

    #[test]
    fn power_zero_yields_dimensionless() {
        // A^0 = 1
        assert!(LENGTH.pow(0).is_dimensionless());
        assert!(FORCE.pow(0).is_dimensionless());
        assert!(ENERGY.pow(0).is_dimensionless());
        assert!(VOLTAGE.pow(0).is_dimensionless());
    }

    #[test]
    fn power_one_preserves_dimension() {
        // A^1 = A
        assert_eq!(LENGTH.pow(1), LENGTH);
        assert_eq!(FORCE.pow(1), FORCE);
        assert_eq!(ENERGY.pow(1), ENERGY);
        assert_eq!(RESISTANCE.pow(1), RESISTANCE);
    }

    #[test]
    fn negative_power_equals_reciprocal_of_positive_power() {
        // A^(-n) = (A^n)^(-1)
        assert_eq!(LENGTH.pow(-2), LENGTH.pow(2).recip());
        assert_eq!(TIME.pow(-3), TIME.pow(3).recip());
        assert_eq!(MASS.pow(-1), MASS.recip());

        // Also verify A^(-1) = A.recip()
        assert_eq!(LENGTH.pow(-1), LENGTH.recip());
        assert_eq!(VELOCITY.pow(-1), VELOCITY.recip());
    }

    #[test]
    fn dimensionless_remains_dimensionless_under_all_operations() {
        // 1 * 1 = 1
        assert!(DIMENSIONLESS.mul(DIMENSIONLESS).is_dimensionless());

        // 1 / 1 = 1
        assert!(DIMENSIONLESS.div(DIMENSIONLESS).is_dimensionless());

        // 1^n = 1
        assert!(DIMENSIONLESS.pow(0).is_dimensionless());
        assert!(DIMENSIONLESS.pow(1).is_dimensionless());
        assert!(DIMENSIONLESS.pow(5).is_dimensionless());
        assert!(DIMENSIONLESS.pow(-1).is_dimensionless());
        assert!(DIMENSIONLESS.pow(-3).is_dimensionless());

        // 1.recip() = 1^(-1) = 1
        assert!(DIMENSIONLESS.recip().is_dimensionless());
    }

    #[test]
    fn power_distributes_over_multiplication() {
        // (A * B)^n = A^n * B^n
        let a = (LENGTH.mul(TIME)).pow(2);
        let b = LENGTH.pow(2).mul(TIME.pow(2));
        assert_eq!(a, b);
    }

    #[test]
    fn power_distributes_over_division() {
        // (A / B)^n = A^n / B^n
        let a = (LENGTH.div(TIME)).pow(2);
        let b = LENGTH.pow(2).div(TIME.pow(2));
        assert_eq!(a, b);
    }
}
