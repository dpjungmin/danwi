#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod dimension;
mod prefixes;
pub mod quantity;
pub mod scalar;
pub mod units;

mod sealed;
pub(crate) use sealed::Sealed;

pub use units::ext::{F32QuantityExt, F64QuantityExt};

pub mod prelude {
    pub use crate::units::{constants::*, ext::F64QuantityExt, types::*, *};
}
