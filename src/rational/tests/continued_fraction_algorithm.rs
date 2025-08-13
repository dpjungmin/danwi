//! Tests for the continued fraction approximation algorithm.

use crate::rational::convert::continued_fraction_approximation as cfa;

#[test]
fn rejects_negative_value() {
    assert!(cfa(-0.1, 1000).is_none());
}

#[test]
fn rejects_non_finite_value() {
    assert!(cfa(f64::NAN, 1000).is_none());
    assert!(cfa(f64::INFINITY, 1000).is_none());
    assert!(cfa(f64::NEG_INFINITY, 1000).is_none());
}

#[test]
fn rejects_zero_max_denominator() {
    assert!(cfa(0.5, 0).is_none());
}

#[test]
fn rejects_overflow() {
    assert!(cfa(f64::MAX, 1000).is_none());
}

#[test]
fn zero() {
    assert_eq!(cfa(0 as _, 1000), Some((0, 1)));
}

#[test]
fn simple_fractions_should_be_found_exactly() {
    assert_eq!(cfa(0.5, 1000), Some((1, 2)));
    assert_eq!(cfa(0.25, 1000), Some((1, 4)));
    assert_eq!(cfa(0.75, 1000), Some((3, 4)));
    assert_eq!(cfa(1.0 / 3.0, 1000), Some((1, 3)));
    assert_eq!(cfa(2.0 / 3.0, 1000), Some((2, 3)));
}

#[test]
fn integers_should_be_found_exactly() {
    assert_eq!(cfa(0.0, 1000), Some((0, 1)));
    assert_eq!(cfa(1.0, 1000), Some((1, 1)));
    assert_eq!(cfa(42.0, 1000), Some((42, 1)));
    assert_eq!(cfa(1000.0, 1000), Some((1000, 1)));
}

#[test]
fn max_denominator_is_respected() {
    // With max_denominator = 10, 1/3 stays as 1/3
    assert_eq!(cfa(1.0 / 3.0, 10), Some((1, 3)));

    // With max_denominator = 2, 1/3 should become 1/2 (closest approximation)
    let (_n, d) = cfa(1.0 / 3.0, 2).unwrap();
    assert!(d <= 2);

    // Pi with small denominator
    let (n, d) = cfa(core::f64::consts::PI, 10).unwrap();
    assert!(d <= 10);
    assert_eq!((n, d), (22, 7));
}

#[test]
fn misc_calcs() {
    assert_eq!(cfa(f64::MIN_POSITIVE, 1e6 as _), Some((0, 1)));
    assert_eq!(cfa(1.0 + f64::EPSILON, 1e6 as _), Some((1, 1)));

    assert_eq!(cfa(0.625, 1000), Some((5, 8)));
    assert_eq!(cfa(0.125, 1000), Some((1, 8)));
    assert_eq!(cfa(0.375, 1000), Some((3, 8)));

    // Powers of 2 as fractions
    assert_eq!(cfa(0.5, 1000), Some((1, 2)));
    assert_eq!(cfa(0.25, 1000), Some((1, 4)));
    assert_eq!(cfa(0.125, 1000), Some((1, 8)));
    assert_eq!(cfa(0.0625, 1000), Some((1, 16)));
    assert_eq!(cfa(0.03125, 1000), Some((1, 32)));
    assert_eq!(cfa(0.015625, 1000), Some((1, 64)));
    assert_eq!(cfa(0.0078125, 1000), Some((1, 128)));
    assert_eq!(cfa(0.00390625, 1000), Some((1, 256)));
}
