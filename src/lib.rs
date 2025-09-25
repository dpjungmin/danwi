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
        F32QuantityExt as QuantityExt, dimension,
        quantity::Quantity,
        scalar::F32Scalar,
        unit::{Unit, constants, types::f32 as types},
    };

    /// Standard gravitational acceleration (9.80665 m/s²).
    pub const G_0: Quantity<F32Scalar, dimension::Acceleration> = Quantity::from_f32_const(9.80665);
}

#[cfg(feature = "f64")]
pub mod f64 {
    pub use crate::{
        F64QuantityExt as QuantityExt, dimension,
        quantity::Quantity,
        scalar::F64Scalar,
        unit::{Unit, constants, types::f64 as types},
    };

    /// Standard gravitational acceleration (9.80665 m/s²).
    pub const G_0: Quantity<F64Scalar, dimension::Acceleration> = Quantity::from_f64_const(9.80665);
}

#[cfg(feature = "f64")]
pub mod prelude {
    pub use crate::{
        F64QuantityExt as QuantityExt,
        quantity::Quantity,
        scalar::F64Scalar as Scalar,
        unit::{Unit, constants::*, types::f64::*},
    };
}
