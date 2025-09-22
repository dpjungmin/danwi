#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod dimension;
pub mod quantity;
pub mod scalar;
pub mod unit;

pub use unit::ext::{F32QuantityExt, F64QuantityExt};

pub mod prelude {
    pub use crate::{F64QuantityExt, quantity::Quantity, scalar::F64Scalar, unit::*};
}
