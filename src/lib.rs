#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod dimension;
pub mod prefix;
pub mod quantity;
pub mod scalar;
pub mod unit;

pub mod prelude {
    pub use crate::{quantity::Quantity, scalar::*, unit::*};
}
