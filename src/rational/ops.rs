//! Arithmetic operations for rational numbers.

use super::{Rational, gcd_u128};
use core::ops::{Add, Div, Mul, Neg, Sub};

impl Add for Rational {
    type Output = Self;

    /// Adds two rational numbers.
    ///
    /// The result is automatically reduced to lowest terms.
    ///
    /// # Panics
    ///
    /// Panics if the operation causes an overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// assert_eq!(Rational::new(1, 2) + Rational::new(1, 3), Rational::new(5, 6));
    /// assert_eq!(Rational::new(1, 4) + Rational::new(3, 4), Rational::new(1, 1));
    /// assert_eq!(Rational::new(-1, 2) + Rational::new(1, 3), Rational::new(-1, 6));
    /// assert_eq!(Rational::zero() + Rational::new(2, 3), Rational::new(2, 3));
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::rational::Rational;
    /// Rational::new_int(i128::MAX) + Rational::new_int(1);
    /// ```
    fn add(self, other: Self) -> Self {
        self.checked_add(&other).unwrap()
    }
}

impl Sub for Rational {
    type Output = Self;

    /// Subtracts one rational number from another.
    ///
    /// The result is automatically reduced to lowest terms.
    ///
    /// # Panics
    ///
    /// Panics if the operation causes an overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// assert_eq!(Rational::new(3, 4) - Rational::new(1, 2), Rational::new(1, 4));
    /// assert_eq!(Rational::new(1, 3) - Rational::new(1, 2), Rational::new(-1, 6));
    /// assert_eq!(Rational::new(5, 7) - Rational::new(5, 7), Rational::zero());
    /// assert_eq!(Rational::new(2, 3) - Rational::zero(), Rational::new(2, 3));
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::rational::Rational;
    /// Rational::new_int(i128::MIN) - Rational::new_int(1);
    /// ```
    fn sub(self, other: Self) -> Self {
        self.checked_sub(&other).unwrap()
    }
}

impl Mul for Rational {
    type Output = Self;

    /// Multiplies two rational numbers.
    ///
    /// The result is automatically reduced to lowest terms.
    ///
    /// # Panics
    ///
    /// Panics if the operation causes an overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// assert_eq!(Rational::new(2, 3) * Rational::new(3, 4), Rational::new(1, 2));
    /// assert_eq!(Rational::new(6, 8) * Rational::new(4, 3), Rational::new(1, 1));
    /// assert_eq!(Rational::new(-2, 5) * Rational::new(5, 2), Rational::new(-1, 1));
    /// assert_eq!(Rational::new(5, 7) * Rational::zero(), Rational::zero());
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::rational::Rational;
    /// Rational::new_int(i128::MAX) * Rational::new_int(2);
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::rational::Rational;
    /// Rational::new_int(i128::MIN) * Rational::new_int(2);
    /// ```
    fn mul(self, other: Self) -> Self {
        self.checked_mul(&other).unwrap()
    }
}

impl Div for Rational {
    type Output = Self;

    /// Divides one rational number by another.
    ///
    /// This is equivalent to multiplying by the reciprocal.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - The divisor is zero
    /// - The operation causes an overflow
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// assert_eq!(Rational::new(2, 3) / Rational::new(4, 5), Rational::new(5, 6));
    /// assert_eq!(Rational::new(6, 7) / Rational::new(2, 7), Rational::new(3, 1));
    /// assert_eq!(Rational::new(-1, 2) / Rational::new(1, 3), Rational::new(-3, 2));
    /// assert_eq!(Rational::new(5, 8) / Rational::new_int(1), Rational::new(5, 8));
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::rational::Rational;
    /// Rational::new(1, 2) / Rational::zero();
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::rational::Rational;
    /// Rational::new_int(i128::MAX) / Rational::new(1, u128::MAX);
    /// ```
    fn div(self, other: Self) -> Self {
        self.checked_div(&other).unwrap()
    }
}

impl Neg for Rational {
    type Output = Self;

    /// Negates a rational number.
    ///
    /// # Panics
    ///
    /// Panics if the numerator is `i128::MIN` (overflow).
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// assert_eq!(-Rational::new(2, 3), Rational::new(-2, 3));
    /// assert_eq!(-Rational::new(-5, 7), Rational::new(5, 7));
    /// assert_eq!(-Rational::zero(), Rational::zero());
    /// assert_eq!(-(-Rational::new(3, 4)), Rational::new(3, 4));
    /// ```
    ///
    /// ```should_panic
    /// # use danwi::rational::Rational;
    /// -Rational::new_int(i128::MIN);
    /// ```
    fn neg(self) -> Self {
        self.checked_neg().unwrap()
    }
}

impl Rational {
    /// Attempts to add two rationals, returning `None` on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// assert_eq!(Rational::new(1, 2).checked_add(&Rational::new(1, 3)), Some(Rational::new(5, 6)));
    /// assert_eq!(Rational::new(1, 4).checked_add(&Rational::new(3, 4)), Some(Rational::new(1, 1)));
    /// assert_eq!(Rational::new(-1, 2).checked_add(&Rational::new(1, 3)), Some(Rational::new(-1, 6)));
    /// assert_eq!(Rational::zero().checked_add(&Rational::new(2, 3)), Some(Rational::new(2, 3)));
    ///
    /// assert_eq!(Rational::new_int(i128::MAX).checked_add(&Rational::new_int(1)), None);
    /// ```
    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        // a/b + c/d = (a*d + b*c) / (b*d)

        // To minimize overflow risk, we find GCD of denominators first
        let gcd_denom = gcd_u128(self.denominator, other.denominator);

        // Scale factors
        let scale_self = other.denominator / gcd_denom;
        let scale_other = self.denominator / gcd_denom;

        // Compute numerator with overflow checking
        let num1 = self.numerator.checked_mul(scale_self as i128)?;
        let num2 = other.numerator.checked_mul(scale_other as i128)?;
        let numerator = num1.checked_add(num2)?;

        // Compute denominator
        let denominator = self.denominator.checked_mul(scale_self)?;

        Self::try_new(numerator, denominator)
    }

    /// Attempts to subtract two rationals, returning `None` on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// assert_eq!(Rational::new(3, 4).checked_sub(&Rational::new(1, 2)), Some(Rational::new(1, 4)));
    /// assert_eq!(Rational::new(1, 3).checked_sub(&Rational::new(1, 2)), Some(Rational::new(-1, 6)));
    /// assert_eq!(Rational::new(5, 7).checked_sub(&Rational::new(5, 7)), Some(Rational::zero()));
    /// assert_eq!(Rational::new(2, 3).checked_sub(&Rational::zero()), Some(Rational::new(2, 3)));
    ///
    /// assert_eq!(Rational::new_int(i128::MIN).checked_sub(&Rational::new_int(1)), None);
    /// ```
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        let neg_other = Self {
            numerator: other.numerator.checked_neg()?,
            denominator: other.denominator,
        };

        self.checked_add(&neg_other)
    }

    /// Attempts to multiply two rationals, returning `None` on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// assert_eq!(Rational::new(2, 3).checked_mul(&Rational::new(3, 4)), Some(Rational::new(1, 2)));
    /// assert_eq!(Rational::new(6, 8).checked_mul(&Rational::new(4, 3)), Some(Rational::new(1, 1)));
    /// assert_eq!(Rational::new(-2, 5).checked_mul(&Rational::new(5, 2)), Some(Rational::new(-1, 1)));
    /// assert_eq!(Rational::new(5, 7).checked_mul(&Rational::zero()), Some(Rational::zero()));
    ///
    /// assert_eq!(Rational::new_int(i128::MAX).checked_mul(&Rational::new_int(2)), None);
    /// assert_eq!(Rational::new_int(i128::MIN).checked_mul(&Rational::new_int(2)), None);
    /// ```
    pub fn checked_mul(&self, other: &Self) -> Option<Self> {
        // To minimize overflow, factor out GCDs before multiplication
        let gcd_ad = gcd_u128(self.numerator.unsigned_abs(), other.denominator);
        let gcd_cb = gcd_u128(other.numerator.unsigned_abs(), self.denominator);

        let num1 = self.numerator / (gcd_ad as i128);
        let num2 = other.numerator / (gcd_cb as i128);
        let den1 = self.denominator / gcd_cb;
        let den2 = other.denominator / gcd_ad;

        let numerator = num1.checked_mul(num2)?;
        let denominator = den1.checked_mul(den2)?;

        Some(Self {
            numerator,
            denominator,
        })
    }

    /// Attempts to divide two rationals, returning `None` on overflow or
    /// division by zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// assert_eq!(Rational::new(2, 3).checked_div(&Rational::new(4, 5)), Some(Rational::new(5, 6)));
    /// assert_eq!(Rational::new(6, 7).checked_div(&Rational::new(2, 7)), Some(Rational::new(3, 1)));
    /// assert_eq!(Rational::new(-1, 2).checked_div(&Rational::new(1, 3)), Some(Rational::new(-3, 2)));
    /// assert_eq!(Rational::new(5, 8).checked_div(&Rational::new_int(1)), Some(Rational::new(5, 8)));
    ///
    /// assert_eq!(Rational::new(1, 2).checked_div(&Rational::zero()), None);
    /// assert_eq!(Rational::new_int(i128::MIN).checked_div(&Rational::new(1, u128::MAX)), None);
    /// ```
    pub fn checked_div(&self, other: &Self) -> Option<Self> {
        // Reject division by zero
        if other.numerator == 0 {
            return None;
        }

        let recip = other.checked_recip()?;
        self.checked_mul(&recip)
    }

    /// Attempts to negate the rational, returning `None` on overflow.
    ///
    /// This can only overflow when the numerator is `i128::MIN`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use danwi::rational::Rational;
    /// assert_eq!(Rational::new(2, 3).checked_neg(), Some(Rational::new(-2, 3)));
    /// assert_eq!(Rational::new(-5, 7).checked_neg(), Some(Rational::new(5, 7)));
    /// assert_eq!(Rational::zero().checked_neg(), Some(Rational::zero()));
    ///
    /// assert_eq!(Rational::new_int(i128::MIN).checked_neg(), None);
    /// ```
    pub fn checked_neg(&self) -> Option<Self> {
        Some(Self {
            numerator: self.numerator.checked_neg()?,
            denominator: self.denominator,
        })
    }
}
