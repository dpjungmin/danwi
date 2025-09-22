#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod dimension;
pub mod quantity;
pub mod scalar;
pub mod unit;

pub use quantity::Quantity;
pub use scalar::{F32Scalar, F64Scalar};
pub use unit::{
    Unit,
    ext::{F32QuantityExt, F64QuantityExt},
};

#[cfg(feature = "f32")]
pub mod f32 {
    pub use crate::{
        F32QuantityExt as QuantityExt,
        quantity::Quantity,
        scalar::F32Scalar as Scalar,
        unit::{Unit, constants},
    };
}

#[cfg(feature = "f64")]
pub mod f64 {
    pub use crate::{
        F64QuantityExt as QuantityExt,
        quantity::Quantity,
        scalar::F64Scalar as Scalar,
        unit::{Unit, constants},
    };
}

pub mod prelude {
    pub use crate::{
        F64QuantityExt,
        quantity::Quantity,
        scalar::F64Scalar,
        unit::{Unit, constants::*},
    };
}
