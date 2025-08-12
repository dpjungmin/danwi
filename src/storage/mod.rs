//! Storage trait for different numeric representations of quantities.
//!
//! This module provides a flexible storage abstraction that allows quantities
//! to use different underlying numeric types while maintaining the same
//! dimensional type safety. This design enables users to choose the most
//! appropriate numeric representation for their specific use case.

use std::fmt::Debug;

pub mod float;
pub mod rational;

pub use float::{F32Storage, F64Storage};
pub use rational::RationalStorage;

/// Trait for types that can store quantity values.
///
/// Storage types determine how the numerical value of a quantity is represented
/// in memory and how arithmetic operations are performed. This abstraction
/// allows the same dimensional analysis to work with different
/// numeric representations.
pub trait Storage: Clone + Debug + PartialEq + Sized {
    /// The underlying numeric type (f32, f64, Rational, etc.).
    ///
    /// This type represents the actual value stored in memory.
    type Value: Clone + Debug;

    /// Get the raw stored value without any unit conversion.
    ///
    /// This provides direct access to the internal representation, which is
    /// useful for storage-specific operations.
    ///
    /// # Returns
    ///
    /// The raw value as stored internally, without any conversion.
    fn raw_value(&self) -> Self::Value;

    /// Add two stored values.
    ///
    /// Both values must represent quantities with the same dimensions.
    /// The implementation should handle the storage type's specific
    /// addition semantics (e.g., floating-point rounding, rational reduction).
    ///
    /// # Arguments
    ///
    /// * `other` - The value to add to self
    ///
    /// # Returns
    ///
    /// A new storage instance containing the sum.
    fn add(&self, other: &Self) -> Self;

    /// Subtract two stored values.
    ///
    /// Both values must represent quantities with the same dimensions.
    ///
    /// # Arguments
    ///
    /// * `other` - The value to subtract from self
    ///
    /// # Returns
    ///
    /// A new storage instance containing the difference.
    fn sub(&self, other: &Self) -> Self;

    /// Multiply two stored values (for quantity multiplication).
    ///
    /// This is used when multiplying quantities with different dimensions,
    /// resulting in a new quantity with combined dimensions.
    ///
    /// # Arguments
    ///
    /// * `other` - The value to multiply with self
    ///
    /// # Returns
    ///
    /// A new storage instance containing the product.
    fn mul(&self, other: &Self) -> Self;

    /// Divide two stored values (for quantity division).
    ///
    /// This is used when dividing quantities, resulting in a new quantity
    /// with the quotient of dimensions.
    ///
    /// # Arguments
    ///
    /// * `other` - The divisor (must not be zero)
    ///
    /// # Returns
    ///
    /// A new storage instance containing the quotient.
    fn div(&self, other: &Self) -> Self;

    /// Negate the stored value.
    ///
    /// Changes the sign of the quantity without affecting its magnitude or
    /// dimensions.
    ///
    /// # Returns
    ///
    /// A new storage instance with the negated value.
    fn neg(&self) -> Self;
}
