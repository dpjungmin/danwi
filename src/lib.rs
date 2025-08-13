#![no_std]

pub(crate) mod quantity;
pub(crate) mod rational;
pub(crate) mod sealed;
pub(crate) mod storage;

pub mod constants;
pub mod extensions;
pub mod units;

pub use crate::rational::Rational;

pub mod prelude {
    pub use crate::{
        constants::*,
        extensions::QuantityExtensions,
        quantity::{LinearContext, OscillatoryContext, Quantity, RotationalContext, TorqueContext},
        units::*,
    };
}
