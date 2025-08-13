//! Conversion traits and methods for rational numbers.

use super::Rational;
use core::convert::TryFrom;

impl Rational {
    /// Attempts to create a rational approximation with a maximum denominator
    /// limit using the
    /// [continued fractions algorithm](https://en.wikipedia.org/wiki/Continued_fraction).
    ///
    /// Returns `None` if:
    /// - `value` is not finite.
    /// - `value` cannot be represented as i128/u128 (overflow)
    ///
    /// This is a non-panicking alternative to [`Self::from_f64_limited`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::Rational;
    /// let pi = Rational::try_from_f64_limited(core::f64::consts::PI, 100).unwrap();
    /// assert!(pi.denominator() <= 100);
    /// assert_eq!(pi, Rational::new(22, 7));
    /// assert!((pi.to_f64() - core::f64::consts::PI).abs() < 1e-2);
    ///
    /// assert_eq!(Rational::try_from_f64_limited(0.5, 1000), Some(Rational::new(1, 2)));
    /// assert_eq!(Rational::try_from_f64_limited(-0.5, 1000), Some(Rational::new(-1, 2)));
    /// assert_eq!(Rational::try_from_f64_limited(0.25, 1000), Some(Rational::new(1, 4)));
    /// assert_eq!(Rational::try_from_f64_limited(1.0 / 3.0, 1000), Some(Rational::new(1, 3)));
    /// assert_eq!(Rational::try_from_f64_limited(0.0, 1000), Some(Rational::new(0, 1)));
    ///
    /// assert_eq!(Rational::try_from_f64_limited(f64::NAN, 1000), None);
    /// assert_eq!(Rational::try_from_f64_limited(f64::INFINITY, 1000), None);
    /// assert_eq!(Rational::try_from_f64_limited(f64::NEG_INFINITY, 1000), None);
    /// assert_eq!(Rational::try_from_f64_limited(f64::MAX, 1000), None);
    /// ```
    pub fn try_from_f64_limited(value: f64, max_denominator: u128) -> Option<Self> {
        // Reject non-finite value
        if !value.is_finite() {
            return None;
        }

        // Special handling for zero
        if value == 0.0 {
            return Some(Self::zero());
        }

        let (numerator, denominator) =
            continued_fraction_approximation(value.abs(), max_denominator)?;

        // Reject if the value is too large to represent (overflow)
        if numerator > i128::MAX as u128 {
            return None;
        }

        let sign = if value < 0.0 { -1_i128 } else { 1_i128 };
        Some(Self::new(sign * (numerator as i128), denominator))
    }

    /// Attempts to create a rational approximation from a floating-point value.
    ///
    /// This is equivalent to `try_from_f64_limited(value, 1e12 as _)`.
    ///
    /// This is a non-panicking alternative to [`Self::from_f64`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::Rational;
    /// let pi = Rational::try_from_f64(core::f64::consts::PI).unwrap();
    /// assert!(pi.denominator() <= 1e12 as _);
    /// assert!((pi.to_f64() - core::f64::consts::PI).abs() < 1e-12);
    ///
    /// assert_eq!(Rational::try_from_f64(0.5), Some(Rational::new(1, 2)));
    /// assert_eq!(Rational::try_from_f64(-0.5), Some(Rational::new(-1, 2)));
    /// assert_eq!(Rational::try_from_f64(0.25), Some(Rational::new(1, 4)));
    /// assert_eq!(Rational::try_from_f64(1.0 / 3.0), Some(Rational::new(1, 3)));
    /// assert_eq!(Rational::try_from_f64(0.0), Some(Rational::new(0, 1)));
    ///
    /// assert_eq!(Rational::try_from_f64(f64::NAN), None);
    /// assert_eq!(Rational::try_from_f64(f64::INFINITY), None);
    /// assert_eq!(Rational::try_from_f64(f64::NEG_INFINITY), None);
    /// assert_eq!(Rational::try_from_f64(f64::MAX), None);
    /// ```
    pub fn try_from_f64(value: f64) -> Option<Self> {
        Self::try_from_f64_limited(value, 1e12 as _)
    }

    /// Creates a rational approximation with a maximum denominator limit using
    /// the
    /// [continued fractions algorithm](https://en.wikipedia.org/wiki/Continued_fraction).
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `value` is not finite.
    /// - `value` cannot be represented as i128/u128 (overflow)
    ///
    /// For a non-panicking version, use [`Self::try_from_f64_limited`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::Rational;
    /// let pi_simple = Rational::from_f64_limited(core::f64::consts::PI, 100);
    /// assert!(pi_simple.denominator() <= 100);
    /// assert!((pi_simple.to_f64() - core::f64::consts::PI).abs() < 1e-2);
    ///
    /// assert_eq!(Rational::from_f64_limited(0.5, 1000), Rational::new(1, 2));
    /// assert_eq!(Rational::from_f64_limited(-0.5, 1000), Rational::new(-1, 2));
    /// assert_eq!(Rational::from_f64_limited(0.25, 1000), Rational::new(1, 4));
    /// assert_eq!(Rational::from_f64_limited(1.0 / 3.0, 1000), Rational::new(1, 3));
    /// assert_eq!(Rational::from_f64_limited(0.0, 1000), Rational::new(0, 1));
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::Rational;
    /// Rational::from_f64_limited(f64::NAN, 1000);
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::Rational;
    /// Rational::from_f64_limited(f64::INFINITY, 1000);
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::Rational;
    /// Rational::from_f64_limited(f64::NEG_INFINITY, 1000);
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::Rational;
    /// Rational::from_f64_limited(f64::MAX, 1000);
    /// ```
    pub fn from_f64_limited(value: f64, max_denominator: u128) -> Self {
        Self::try_from_f64_limited(value, max_denominator).unwrap()
    }

    /// Creates a rational approximation from a floating-point value.
    ///
    /// This is equivalent to `from_f64_limited(value, 1e12 as _)`.
    ///
    /// For a version with controllable maximum denominator, use
    /// [`Self::from_f64_limited`].
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `value` is not finite.
    /// - `value` cannot be represented as i128/u128 (overflow)
    ///
    /// For a non-panicking version, use [`Self::try_from_f64`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::Rational;
    /// let one_third = Rational::from_f64(1.0 / 3.0);
    /// assert_eq!(one_third.numerator(), 1);
    /// assert_eq!(one_third.denominator(), 3);
    ///
    /// assert_eq!(Rational::from_f64(0.5), Rational::new(1, 2));
    /// assert_eq!(Rational::from_f64(-0.5), Rational::new(-1, 2));
    /// assert_eq!(Rational::from_f64(0.25), Rational::new(1, 4));
    /// assert_eq!(Rational::from_f64(1.0 / 3.0), Rational::new(1, 3));
    /// assert_eq!(Rational::from_f64(0.0), Rational::new(0, 1));
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::Rational;
    /// Rational::from_f64(f64::NAN);
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::Rational;
    /// Rational::from_f64(f64::INFINITY);
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::Rational;
    /// Rational::from_f64(f64::NEG_INFINITY);
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::Rational;
    /// Rational::from_f64(f64::MAX);
    /// ```
    pub fn from_f64(value: f64) -> Self {
        Self::from_f64_limited(value, 1e12 as _)
    }

    /// Converts the rational to an f64 approximation.
    ///
    /// This conversion may lose precision, especially for large numerators
    /// or denominators that exceed f64's 53-bit mantissa precision.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::Rational;
    /// let one_third = Rational::new(1, 3);
    /// assert!((one_third.to_f64() - 0.333333333333333).abs() < 1e-15);
    ///
    /// let a = Rational::new(9_007_199_254_740_993_i128, 1); // 2^53 + 1
    /// let b = Rational::from_f64(a.to_f64());
    /// // The round-trip loses precision due to f64's limitations
    /// assert_ne!(a, b);
    /// assert_eq!(b, Rational::new(9_007_199_254_740_992_i128, 1));
    /// ```
    pub fn to_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    /// Converts the rational to an f32 approximation.
    ///
    /// This conversion may lose significantly more precision than
    /// [`Self::to_f64`], as f32 has only 24 bits of mantissa precision
    /// compared to f64's 53 bits.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::Rational;
    /// let big = Rational::new(16_777_217, 1); // 2^24 + 1
    /// // f32 can't represent this exactly (only 24 bits mantissa)
    /// assert_eq!(big.to_f32(), 16_777_216.0_f32);
    /// // f64 can represent it exactly (53 bits mantissa)
    /// assert_eq!(big.to_f64(), 16_777_217.0_f64);
    /// ```
    pub fn to_f32(&self) -> f32 {
        self.to_f64() as _
    }
}

impl TryFrom<f32> for Rational {
    type Error = ();

    /// Attempts to convert an f32 to a Rational.
    ///
    /// Returns an error if the value is not finite.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::Rational;
    /// # use core::convert::TryFrom;
    /// assert_eq!(Rational::try_from(0.5_f32), Ok(Rational::new(1, 2)));
    /// assert_eq!(Rational::try_from(0.25_f32), Ok(Rational::new(1, 4)));
    ///
    /// assert!(Rational::try_from(f32::NAN).is_err());
    /// assert!(Rational::try_from(f32::INFINITY).is_err());
    /// assert!(Rational::try_from(f32::NEG_INFINITY).is_err());
    /// assert!(Rational::try_from(f32::MAX).is_err());
    /// ```
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from_f64(value as f64).ok_or(())
    }
}

impl TryFrom<f64> for Rational {
    type Error = ();

    /// Attempts to convert an f64 to a Rational.
    ///
    /// Returns an error if the value is not finite.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::Rational;
    /// # use core::convert::TryFrom;
    /// assert_eq!(Rational::try_from(0.5), Ok(Rational::new(1, 2)));
    /// assert_eq!(Rational::try_from(0.25), Ok(Rational::new(1, 4)));
    /// assert_eq!(Rational::try_from(1.0 / 3.0), Ok(Rational::new(1, 3)));
    ///
    /// assert!(Rational::try_from(f64::NAN).is_err());
    /// assert!(Rational::try_from(f64::INFINITY).is_err());
    /// assert!(Rational::try_from(f64::NEG_INFINITY).is_err());
    /// assert!(Rational::try_from(f64::MAX).is_err());
    /// ```
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::try_from_f64(value).ok_or(())
    }
}

impl From<i32> for Rational {
    /// Converts an i32 to a Rational.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::Rational;
    /// let five: Rational = 5_i32.into();
    /// assert_eq!(five, Rational::new(5, 1));
    ///
    /// assert_eq!(Rational::from(-3_i32), Rational::new(-3, 1));
    /// assert_eq!(Rational::from(0_i32), Rational::new(0, 1));
    /// assert_eq!(Rational::from(i32::MIN), Rational::new(i32::MIN as _, 1));
    /// assert_eq!(Rational::from(i32::MAX), Rational::new(i32::MAX as _, 1));
    /// ```
    fn from(value: i32) -> Self {
        Self::new_int(value as _)
    }
}

impl From<i64> for Rational {
    /// Converts an i64 to a Rational.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::Rational;
    /// let five: Rational = 5_i64.into();
    /// assert_eq!(five, Rational::new(5, 1));
    ///
    /// assert_eq!(Rational::from(-3_i64), Rational::new(-3, 1));
    /// assert_eq!(Rational::from(0_i64), Rational::new(0, 1));
    /// assert_eq!(Rational::from(i64::MIN), Rational::new(i64::MIN as _, 1));
    /// assert_eq!(Rational::from(i64::MAX), Rational::new(i64::MAX as _, 1));
    /// ```
    fn from(value: i64) -> Self {
        Self::new_int(value as _)
    }
}

/// Computes the continued fraction approximation of a positive f64 value.
///
/// Returns `Some((numerator, denominator))` as u128 values representing the
/// best rational approximation with denominator ≤ max_denominator, or `None`
/// if the input is invalid.
///
/// # Returns
///
/// - `None` if:
///   - `value` is negative or non-finite
///   - `max_denominator` is 0
///   - `value` cannot be represented as u128 (overflow)
/// - `Some((0, 1))` if `value` is zero.
/// - `Some((numerator, denominator))` otherwise
pub(super) fn continued_fraction_approximation(
    value: f64,
    max_denominator: u128,
) -> Option<(u128, u128)> {
    // Reject negative value
    if value < 0.0 {
        return None;
    }

    // Reject non-finite value
    if !value.is_finite() {
        return None;
    }

    // Reject zero max denominator
    if max_denominator == 0 {
        return None;
    }

    const U128_MAX_F64: f64 = u128::MAX as f64;

    // Reject if the value is too large to represent (overflow)
    if value > U128_MAX_F64 {
        return None;
    }

    // Special handling for zero
    if value == 0.0 {
        return Some((0, 1));
    }

    // Initialize convergents
    let mut x = value;
    let mut h_prev = 1u128;
    let mut k_prev = 0u128;
    // Extract integer part (we know x is positive and < u128::MAX)
    let mut h = x as u128; // truncation gives floor for positive numbers
    let mut k = 1u128;

    // Iterate at most 100 times to prevent infinite loops (complex irrationals
    // converge within ~40 ierations)
    for _ in 0..100 {
        // Is the current approximation close enough?
        let current_value = h as f64 / k as f64;
        let relative_error = ((current_value - value) / value).abs();

        if relative_error < f64::EPSILON {
            break;
        }

        // Extract fractional part (x is positive, truncation gives floor)
        x = x - (x as u128 as f64);

        // Stop if fractional part is smaller than 1e-15 to avoid overflow in
        // reciprocal
        if x < 1e-15 {
            break;
        }

        // Compute reciprocal
        x = 1.0 / x;

        // Check if reciprocal is finite and not too large
        if !x.is_finite() || x > U128_MAX_F64 {
            break;
        }

        // Next coefficient (x is positive after reciprocal)
        let ai = x as u128;

        // Calculate next convergent using recurrence relation:
        // - h_n = a_n * h_{n-1} + h_{n-2}
        // - k_n = a_n * k_{n-1} + k_{n-2}

        // Check for overflow in numerator calculation
        let h_new = match ai.checked_mul(h).and_then(|x| x.checked_add(h_prev)) {
            Some(v) => v,
            None => break, // Overflow, use current convergent
        };

        // Check for overflow and max_denominator limit
        let k_new = match ai.checked_mul(k).and_then(|x| x.checked_add(k_prev)) {
            Some(v) if v <= max_denominator => v,
            _ => break, // Overflow or exceeds limit, use current convergent
        };

        // Update convergents
        h_prev = h;
        k_prev = k;
        h = h_new;
        k = k_new;
    }

    Some((h, k))
}
