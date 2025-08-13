//! Floating-point storage implementations for quantities.
//!
//! This module provides storage types based on IEEE 754 floating-point numbers,
//! offering different trade-offs between precision, range, and performance.

use super::{RationalStorage, Storage};
use crate::sealed::Sealed;
use core::convert::TryFrom;

/// Internal macro to generate floating-point storage implementations.
macro_rules! impl_float_storage {
    (
        $(#[$struct_meta:meta])*
        $name:ident,
        $type:ty
    ) => {
        $(#[$struct_meta])*
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct $name {
            value: $type,
        }

        impl $name {
            /// Creates a new storage with the given value.
            ///
            /// This can be used in const contexts.
            pub const fn new(value: $type) -> Self {
                Self { value }
            }

            /// Creates storage from the native type directly.
            ///
            /// This doesn't perform any conversion, avoiding potential rounding
            /// when the value is already in the correct type.
            pub fn from_native(value: $type) -> Self {
                Self { value }
            }

            pub fn is_finite(&self) -> bool {
                self.value.is_finite()
            }

            pub fn is_nan(&self) -> bool {
                self.value.is_nan()
            }

            pub fn is_infinite(&self) -> bool {
                self.value.is_infinite()
            }
        }

        impl Storage for $name {
            type Value = $type;

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

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self { value }
            }
        }

        impl From<$name> for $type {
            fn from(storage: $name) -> $type {
                storage.value
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self { value: 0.0 }
            }
        }
    };
}

impl_float_storage!(F32Storage, f32);
impl_float_storage!(F64Storage, f64);

impl Sealed for F32Storage {}
impl Sealed for F64Storage {}

impl From<F32Storage> for f64 {
    fn from(storage: F32Storage) -> f64 {
        storage.value as _
    }
}

impl From<f64> for F32Storage {
    fn from(value: f64) -> Self {
        Self { value: value as _ }
    }
}

// Lossy
impl From<F64Storage> for F32Storage {
    fn from(value: F64Storage) -> Self {
        F32Storage::from(value.value as f32)
    }
}

impl From<F32Storage> for F64Storage {
    fn from(value: F32Storage) -> Self {
        F64Storage::from(value.value as f64)
    }
}

impl TryFrom<F32Storage> for RationalStorage {
    type Error = ();

    fn try_from(value: F32Storage) -> Result<Self, Self::Error> {
        RationalStorage::try_from(value.value)
    }
}

impl TryFrom<F64Storage> for RationalStorage {
    type Error = ();

    fn try_from(value: F64Storage) -> Result<Self, Self::Error> {
        RationalStorage::try_from(value.value)
    }
}

// Lossy
impl From<RationalStorage> for F64Storage {
    fn from(value: RationalStorage) -> Self {
        F64Storage::from(value.raw_value().to_f64())
    }
}

// Lossy
impl From<RationalStorage> for F32Storage {
    fn from(value: RationalStorage) -> Self {
        F32Storage::from(value.raw_value().to_f32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Macro to generate common tests for float storage types
    macro_rules! test_float_storage {
        ($storage:ty, $type:ty, $name:ident) => {
            mod $name {
                use super::*;

                #[test]
                fn performs_all_arithmetic_operations_correctly() {
                    let a = <$storage>::from(5.0);
                    let b = <$storage>::from(3.0);

                    assert_eq!(a.raw_value(), 5.0 as $type);
                    assert_eq!(b.raw_value(), 3.0 as $type);

                    assert_eq!(a.add(&b).raw_value(), 8.0 as $type);
                    assert_eq!(a.sub(&b).raw_value(), 2.0 as $type);
                    assert_eq!(a.mul(&b).raw_value(), 15.0 as $type);
                    assert_eq!(a.div(&b).raw_value(), (5.0 as $type) / (3.0 as $type));
                    assert_eq!(a.neg().raw_value(), -5.0 as $type);
                }

                #[test]
                fn handles_infinity_and_finite_checks_correctly() {
                    let inf = <$storage>::from(f64::INFINITY);
                    assert!(inf.is_infinite());
                    assert!(!inf.is_finite());

                    let normal = <$storage>::from_native(1.0);
                    assert!(normal.is_finite());
                    assert!(!normal.is_infinite());
                }

                #[test]
                fn allows_const_construction_at_compile_time() {
                    const ZERO: $storage = <$storage>::new(0.0);
                    assert_eq!(ZERO.raw_value(), 0.0);
                }

                #[test]
                fn default_initializes_to_zero() {
                    assert_eq!(<$storage>::default().raw_value(), 0.0);
                }
            }
        };
    }

    test_float_storage!(F32Storage, f32, f32_storage_tests);
    test_float_storage!(F64Storage, f64, f64_storage_tests);
}
