#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod dimension;
pub mod quantity;
pub mod scalar;
pub mod units;

mod sealed;
pub(crate) use sealed::Sealed;

pub mod prelude {
    pub use crate::units::{constants::*, types::*, *};
}
