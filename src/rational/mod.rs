//! Rational numbers for exact arithmetic.
//!
//! This module provides the [`Rational`] type, which represents exact
//! fractions using a fixed-sized numerator and denominator. Unlike
//! floating-point numbers, rational numbers provide exact arithmetic without
//! rounding errors, making them ideal for applications requiring perfect
//! precision.
//!
//! # Overview
//!
//! A [`Rational`] represents a fraction as `numerator / denominator` where:
//! - The numerator is a `i128` (signed, carries the sign of the fraction).
//! - The denominator is a `u128` (unsigned, always positive).
//! - The fraction is always stored in lowest terms (reduced form).

use core::fmt;

mod convert;
mod ops;

#[cfg(test)]
mod tests;

/// A rational number represented as a fraction.
///
/// # Representation
///
/// - Sign is always in the numerator.
/// - Denominator is always positive.
/// - Fraction is always in lowest terms.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rational {
    pub(super) numerator: i128,
    pub(super) denominator: u128,
}

impl Rational {
    /// Creates a new rational number from `numerator` and `denominator`,
    /// returning `None` if the input is invalid.
    ///
    /// The fraction is automatically reduced to lowest terms using the
    /// Euclidean algorithm for GCD calculation.
    ///
    /// Returns `None` if:
    /// - The `denominator` is zero
    /// - Reduction causes an overflow (should never be possible, but checking
    ///   to protect against any bugs)
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// // Basic fractions
    /// let one_half = Rational::try_new(1, 2).unwrap();
    /// let one_third = Rational::try_new(1, 3).unwrap();
    ///
    /// // Automatic reduction
    /// let one_half = Rational::try_new(5, 10).unwrap();
    /// assert_eq!(one_half.numerator(), 1);
    /// assert_eq!(one_half.denominator(), 2);
    ///
    /// // Zero
    /// let zero = Rational::try_new(0, 5).unwrap();
    /// assert_eq!(zero.numerator(), 0);
    /// assert_eq!(zero.denominator(), 1);
    ///
    /// // Zero denominator is now allowed
    /// let err = Rational::try_new(1, 0);
    /// assert_eq!(err, None);
    /// ```
    pub fn try_new(numerator: i128, denominator: u128) -> Option<Self> {
        if denominator == 0 {
            return None;
        }

        let gcd = gcd_u128(numerator.unsigned_abs(), denominator);

        // Reduce fraction to lowest terms
        //
        // Note: These checked_div calls are defensive programming.
        // Mathematically, they can never fail because:
        // - GCD is always >= 1 (so no division by zero)
        // - GCD is always <= min(|numerator|, denominator) (so no overflow)
        // - overflow only occurs with i128::MIN / -1
        let numerator = numerator.checked_div(gcd as _)?;
        let denominator = denominator.checked_div(gcd)?;

        Some(Self {
            numerator,
            denominator,
        })
    }

    /// Creates a new rational number from `numerator` and `denominator`.
    ///
    /// The fraction is automatically reduced to lowest terms using the
    /// Euclidean algorithm for GCD calculation.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - The `denominator` is zero.
    /// - Reduction causes an overflow (should never be possible, but we check
    ///   to protect against any bugs).
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// // Basic fractions
    /// let one_half = Rational::new(1, 2);
    /// let one_third = Rational::new(1, 3);
    ///
    /// // Automatic reduction
    /// assert_eq!(Rational::new(2, 4), Rational::new(1, 2));
    /// assert_eq!(Rational::new(6, 8), Rational::new(3, 4));
    ///
    /// // Negative numbers (sign always in numerator)
    /// let neg = Rational::new(-3, 5);
    /// assert_eq!(neg.numerator(), -3);
    /// assert_eq!(neg.denominator(), 5);
    ///
    /// // Zero
    /// let zero = Rational::new(0, 5);
    /// assert_eq!(zero, Rational::zero());
    /// assert_eq!(zero.denominator(), 1);
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::rational::Rational;
    /// Rational::new(1, 0);  // Zero denominator panics
    /// ```
    pub fn new(numerator: i128, denominator: u128) -> Self {
        Self::try_new(numerator, denominator).unwrap()
    }

    /// Attempts to compute the reciprocal, returning `None` on failure.
    ///
    /// Returns `None` if:
    /// - The numerator is zero (division by zero)
    /// - The operation would cause an overflow
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// let one_half = Rational::new(1, 2);
    /// assert_eq!(one_half.checked_recip(), Some(Rational::new(2, 1)));
    ///
    /// // Numerator is zero
    /// assert_eq!(Rational::zero().checked_recip(), None);
    ///
    /// // Overflow (denominator > i128::MAX)
    /// assert_eq!(Rational::new(1, u128::MAX).checked_recip(), None);
    /// ```
    pub fn checked_recip(self) -> Option<Self> {
        if self.numerator == 0 {
            return None;
        }

        // Convert denominator to i128
        let numerator = if self.numerator.is_negative() {
            i128::try_from(self.denominator).ok()?.checked_neg()?
        } else {
            i128::try_from(self.denominator).ok()?
        };

        Some(Self {
            numerator,
            denominator: self.numerator.unsigned_abs(),
        })
    }

    /// Returns the reciprocal (multiplicative inverse) of this rational.
    ///
    /// For a rational a/b, returns b/a. The sign is preserved in the numerator.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - The `numerator` is zero (division by zero)
    /// - The operation causes an overflow (when denominator > i128::MAX)
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// assert_eq!(Rational::new(2, 3).recip(), Rational::new(3, 2));
    /// assert_eq!(Rational::new(-2, 5).recip(), Rational::new(-5, 2));
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::rational::Rational;
    /// Rational::zero().recip();
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::rational::Rational;
    /// Rational::new(1, u128::MAX).recip();
    /// ```
    pub fn recip(self) -> Self {
        match self.checked_recip() {
            Some(result) => result,
            None => {
                if self.numerator == 0 {
                    panic!("cannot take reciprocal of zero");
                } else {
                    panic!("reciprocal overflow: denominator too large");
                }
            }
        }
    }

    /// Returns the zero rational (0/1).
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// let zero = Rational::zero();
    /// assert_eq!(zero.numerator(), 0);
    /// assert_eq!(zero.denominator(), 1);
    /// ```
    pub const fn zero() -> Self {
        Self::new_int(0)
    }

    /// Creates a rational number representing an integer.
    ///
    /// This is equivalent to `new(numerator, 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// let five = Rational::new_int(5);
    /// assert_eq!(five.numerator(), 5);
    /// assert_eq!(five.denominator(), 1);
    /// ```
    pub const fn new_int(numerator: i128) -> Self {
        Self {
            numerator,
            denominator: 1,
        }
    }

    /// Returns the numerator of the fraction.
    ///
    /// The numerator carries the sign of the rational number.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// // Numerator carries the sign
    /// assert_eq!(Rational::new(3, 4).numerator(), 3);
    ///
    /// // Fractions are always reduced (6/8 = 3/4)
    /// assert_eq!(Rational::new(6, 8).numerator(), 3);
    ///
    /// // Zero has numerator 0
    /// assert_eq!(Rational::zero().numerator(), 0);
    /// ```
    pub fn numerator(&self) -> i128 {
        self.numerator
    }

    /// Returns the denominator of the fraction.
    ///
    /// The denominator is always positive and the fraction is always in lowest
    /// terms.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// // Denominators are always positive (sign is in numerator)
    /// assert_eq!(Rational::new(-2, 5).denominator(), 5);
    ///
    /// // Fractions are always reduced (6/8 = 3/4)
    /// assert_eq!(Rational::new(6, 8).denominator(), 4);
    ///
    /// // Zero has denominator 1
    /// assert_eq!(Rational::zero().denominator(), 1);
    /// ```
    pub fn denominator(&self) -> u128 {
        self.denominator
    }
}

/// Formats a rational number for display.
///
/// # Examples
///
/// ```
/// # use danwi::rational::Rational;
/// // Integers display without denominator
/// assert_eq!(format!("{}", Rational::new_int(5)), "5");
/// assert_eq!(format!("{}", Rational::new_int(-3)), "-3");
///
/// // Fractions display as numerator/denominator
/// assert_eq!(format!("{}", Rational::new(3, 4)), "3/4");
/// assert_eq!(format!("{}", Rational::new(-2, 3)), "-2/3");
///
/// // Fractions are always reduced
/// assert_eq!(format!("{}", Rational::new(6, 8)), "3/4");
/// ```
impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == 1 {
            return write!(f, "{}", self.numerator);
        }

        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

/// Calculates the greatest common divisor (GCD) using the Euclidean algorithm.
///
/// This function is used internally for:
/// - Reducing fractions to lowest terms
/// - Minimizing overflow risk in arithmetic operations
pub(super) fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}
