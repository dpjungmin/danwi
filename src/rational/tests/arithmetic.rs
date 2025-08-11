//! Tests for arithmetic operations on rational numbers.

use crate::rational::Rational;

#[test]
fn rational_follows_the_laws_of_arithmetic() {
    let a = Rational::new(2, 3);
    let b = Rational::new(3, 4);
    let c = Rational::new(5, 6);

    // Commutative property of addition
    assert_eq!(a + b, b + a);

    // Commutative property of multiplication
    assert_eq!(a * b, b * a);

    // Associative property of addition
    assert_eq!((a + b) + c, a + (b + c));

    // Associative property of multiplication
    assert_eq!((a * b) * c, a * (b * c));

    // Distributive property
    assert_eq!(a * (b + c), (a * b) + (a * c));
}
