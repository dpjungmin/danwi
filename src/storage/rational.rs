//! Rational number storage for exact arithmetic.
//!
//! This module provides exact arithmetic through rational numbers (fractions),
//! eliminating rounding errors inherent in floating-point arithmetic.

use super::Storage;
use crate::rational::Rational;
use core::fmt;
use std::convert::TryFrom;

/// Storage using rational numbers for exact arithmetic.
///
/// This storage type preserves exact values without floating-point rounding
/// errors, making it ideal for applications requiring precise calculations.
/// However, it is slower than floating-point storage and can overflow with
/// very large numerators or denominators.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RationalStorage {
    value: Rational,
}

impl RationalStorage {
    /// Creates a RationalStorage from a rational value.
    pub fn from_rational(value: Rational) -> Self {
        Self { value }
    }

    /// Creates a RationalStorage from numerator and denominator.
    ///
    /// The fraction will be automatically reduced to lowest terms.
    pub fn from_ratio(numerator: i128, denominator: u128) -> Self {
        Self {
            value: Rational::new(numerator, denominator),
        }
    }

    /// Creates a RationalStorage from an integer value.
    pub fn from_int(value: i128) -> Self {
        Self {
            value: Rational::new_int(value),
        }
    }

    /// Returns the numerator of the stored rational.
    pub fn numerator(&self) -> i128 {
        self.value.numerator()
    }

    /// Returns the denominator of the stored rational.
    pub fn denominator(&self) -> u128 {
        self.value.denominator()
    }
}

impl Storage for RationalStorage {
    type Value = Rational;

    fn raw_value(&self) -> Self::Value {
        self.value
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }

    fn div(&self, other: &Self) -> Self {
        Self {
            value: self.value / other.value,
        }
    }

    fn neg(&self) -> Self {
        Self { value: -self.value }
    }
}

impl Default for RationalStorage {
    fn default() -> Self {
        Self {
            value: Rational::zero(),
        }
    }
}

impl fmt::Display for RationalStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<f32> for RationalStorage {
    type Error = ();

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(value as f64)
    }
}

impl TryFrom<f64> for RationalStorage {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Self {
            value: Rational::try_from_f64(value).ok_or(())?,
        })
    }
}

impl From<Rational> for RationalStorage {
    fn from(value: Rational) -> Self {
        Self { value }
    }
}

impl From<i32> for RationalStorage {
    fn from(value: i32) -> Self {
        Self {
            value: Rational::from(value),
        }
    }
}

impl From<i64> for RationalStorage {
    fn from(value: i64) -> Self {
        Self {
            value: Rational::from(value),
        }
    }
}

impl From<i128> for RationalStorage {
    fn from(value: i128) -> Self {
        Self {
            value: Rational::new_int(value),
        }
    }
}

// Lossy
impl From<RationalStorage> for f64 {
    fn from(storage: RationalStorage) -> Self {
        storage.value.to_f64()
    }
}

// Lossy
impl From<RationalStorage> for f32 {
    fn from(storage: RationalStorage) -> Self {
        storage.value.to_f32()
    }
}

impl From<RationalStorage> for Rational {
    fn from(storage: RationalStorage) -> Self {
        storage.value
    }
}
