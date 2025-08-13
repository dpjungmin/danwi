//! Sealed trait pattern for preventing external implementations.

/// This is used to "seal" public traits, preventing external implementations
/// while still allowing the traits to be used as bounds.
pub trait Sealed {}
