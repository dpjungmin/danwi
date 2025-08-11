//! Tests for the rational module.

mod arithmetic;
mod continued_fraction_algorithm;

use crate::rational::Rational;

#[test]
fn try_new_returns_none_if_denominator_is_zero() {
    assert_eq!(Rational::try_new(i128::MIN, 0), None);
    assert_eq!(Rational::try_new(i128::MAX, 0), None);

    assert_eq!(Rational::try_new(-1, u128::MIN), None);
    assert_eq!(Rational::try_new(0, u128::MIN), None);
    assert_eq!(Rational::try_new(1, u128::MIN), None);

    assert_eq!(Rational::try_new(-1, 0), None);
    assert_eq!(Rational::try_new(0, 0), None);
    assert_eq!(Rational::try_new(1, 0), None);
}

#[test]
fn try_new_automatically_reduces_to_lowest_terms() {
    let r = Rational::try_new(2, 4).unwrap();
    assert_eq!(r.numerator(), 1);
    assert_eq!(r.denominator(), 2);

    let r = Rational::try_new(3, 9).unwrap();
    assert_eq!(r.numerator(), 1);
    assert_eq!(r.denominator(), 3);

    let r = Rational::try_new(10, 35).unwrap();
    assert_eq!(r.numerator(), 2);
    assert_eq!(r.denominator(), 7);

    let r = Rational::try_new(0, 5).unwrap();
    assert_eq!(r.numerator(), 0);
    assert_eq!(r.denominator(), 1);
}

#[test]
fn try_new_handles_valid_inputs() {
    assert_eq!(
        Rational::try_new(1, 2),
        Some(Rational {
            numerator: 1,
            denominator: 2
        })
    );
    assert_eq!(
        Rational::try_new(2, 4),
        Some(Rational {
            numerator: 1,
            denominator: 2
        })
    );
    assert_eq!(
        Rational::try_new(-2, 3),
        Some(Rational {
            numerator: -2,
            denominator: 3
        })
    );
    assert_eq!(
        Rational::try_new(0, 5),
        Some(Rational {
            numerator: 0,
            denominator: 1
        })
    );
}

#[test]
#[should_panic]
fn new_panics_if_denominator_is_zero() {
    Rational::new(1, 0);
}

#[test]
fn new_automatically_reduces_to_lowest_terms() {
    let r = Rational::new(2, 4);
    assert_eq!(r.numerator(), 1);
    assert_eq!(r.denominator(), 2);

    let r = Rational::new(3, 9);
    assert_eq!(r.numerator(), 1);
    assert_eq!(r.denominator(), 3);

    let r = Rational::new(10, 35);
    assert_eq!(r.numerator(), 2);
    assert_eq!(r.denominator(), 7);

    let r = Rational::new(0, 5);
    assert_eq!(r.numerator(), 0);
    assert_eq!(r.denominator(), 1);
}

#[test]
fn new_handles_valid_inputs() {
    assert_eq!(
        Rational::new(1, 2),
        Rational {
            numerator: 1,
            denominator: 2
        }
    );
    assert_eq!(
        Rational::new(2, 4),
        Rational {
            numerator: 1,
            denominator: 2
        }
    );
    assert_eq!(
        Rational::new(-2, 3),
        Rational {
            numerator: -2,
            denominator: 3
        }
    );
    assert_eq!(
        Rational::new(0, 5),
        Rational {
            numerator: 0,
            denominator: 1
        }
    );
}

#[test]
fn checked_recip_returns_none_if_numerator_is_zero() {
    assert_eq!(Rational::zero().checked_recip(), None);
    assert_eq!(Rational::new(0, 1).checked_recip(), None);
    assert_eq!(Rational::new(0, 5).checked_recip(), None);
}

#[test]
fn checked_recip_returns_none_if_denominator_is_greater_than_i128_max() {
    let overflow = Rational::new(1, u128::MAX);
    assert_eq!(overflow.checked_recip(), None);
}

#[test]
fn checked_recip_handles_valid_inputs() {
    assert_eq!(
        Rational::new(1, 2).checked_recip(),
        Some(Rational::new(2, 1))
    );
    assert_eq!(
        Rational::new(2, 7).checked_recip(),
        Some(Rational::new(7, 2))
    );
    assert_eq!(
        Rational::new(-3, 4).checked_recip(),
        Some(Rational::new(-4, 3))
    );
}

#[test]
#[should_panic(expected = "cannot take reciprocal of zero")]
fn recip_panics_if_numerator_is_zero() {
    Rational::zero().recip();
}

#[test]
#[should_panic(expected = "reciprocal overflow: denominator too large")]
fn recip_panics_if_denominator_is_greater_than_i128_max() {
    Rational::new(1, u128::MAX).recip();
}

#[test]
fn recip_handles_valid_inputs() {
    assert_eq!(Rational::new(1, 2).recip(), Rational::new(2, 1));
    assert_eq!(Rational::new(2, 7).recip(), Rational::new(7, 2));
    assert_eq!(Rational::new(-3, 4).recip(), Rational::new(-4, 3));
}

mod gcd_u128_tests {
    use crate::rational::gcd_u128;

    #[test]
    fn computes_common_divisors() {
        assert_eq!(gcd_u128(48, 18), 6);
        assert_eq!(gcd_u128(100, 50), 50);
        assert_eq!(gcd_u128(21, 14), 7);
        assert_eq!(gcd_u128(12, 8), 4);
        assert_eq!(gcd_u128(16, 12), 4);
        assert_eq!(gcd_u128(32, 20), 4);
        assert_eq!(gcd_u128(64, 48), 16);
    }

    #[test]
    fn computes_coprimes() {
        assert_eq!(gcd_u128(17, 19), 1);
        assert_eq!(gcd_u128(13, 27), 1);
        assert_eq!(gcd_u128(25, 49), 1);
    }

    #[test]
    fn gcd_of_zeros_is_zero() {
        assert_eq!(gcd_u128(0, 0), 0);
    }

    #[test]
    fn gcd_with_zero_is_other() {
        assert_eq!(gcd_u128(0, 5), 5);
        assert_eq!(gcd_u128(5, 0), 5);
    }

    #[test]
    fn gcd_with_one_is_one() {
        assert_eq!(gcd_u128(1, 1), 1);
        assert_eq!(gcd_u128(1, 100), 1);
        assert_eq!(gcd_u128(100, 1), 1);
    }

    #[test]
    fn gcd_with_same_number_is_the_number() {
        assert_eq!(gcd_u128(42, 42), 42);
        assert_eq!(gcd_u128(1000, 1000), 1000);
    }

    #[test]
    fn gcd_with_divisor_is_the_divisor() {
        assert_eq!(gcd_u128(12, 36), 12);
        assert_eq!(gcd_u128(36, 12), 12);
        assert_eq!(gcd_u128(7, 49), 7);

        // Power of 2
        assert_eq!(gcd_u128(16, 32), 16);
        assert_eq!(gcd_u128(64, 128), 64);
        assert_eq!(gcd_u128(1024, 512), 512);
    }

    #[test]
    fn is_commutative() {
        // gcd(a, b) = gcd(b, a)
        assert_eq!(gcd_u128(48, 18), gcd_u128(18, 48));
        assert_eq!(gcd_u128(100, 75), gcd_u128(75, 100));
        assert_eq!(gcd_u128(0, 42), gcd_u128(42, 0));
    }

    #[test]
    fn is_associative() {
        // gcd(gcd(a, b), c) = gcd(a, gcd(b, c))
        let a = 60;
        let b = 48;
        let c = 36;
        assert_eq!(gcd_u128(gcd_u128(a, b), c), gcd_u128(a, gcd_u128(b, c)));
    }

    #[test]
    fn is_identity() {
        // gcd(a, 1) = 1 for any a > 0
        assert_eq!(gcd_u128(42, 1), 1);
        assert_eq!(gcd_u128(1, 42), 1);
        assert_eq!(gcd_u128(u128::MAX, 1), 1);
    }

    #[test]
    fn correctly_finds_common_prime_factors() {
        let a = 2 * 2 * 3 * 5 * 7;
        let b = 2 * 3 * 3 * 7 * 11;
        let expected = 2 * 3 * 7;
        assert_eq!(gcd_u128(a, b), expected);

        let a = 2_u128.pow(10) * 3_u128.pow(5) * 5_u128.pow(3);
        let b = 2_u128.pow(8) * 3_u128.pow(7) * 7_u128.pow(2);
        let expected = 2_u128.pow(8) * 3_u128.pow(5);
        assert_eq!(gcd_u128(a, b), expected);
    }
}
