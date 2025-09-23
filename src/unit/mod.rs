#![allow(non_upper_case_globals)]

use crate::{
    dimension::*,
    quantity::Quantity,
    scalar::{F32Scalar, F64Scalar},
};
use core::marker::PhantomData;

#[macro_use]
mod macros;
mod ops;
mod prefix;

#[derive(Debug, Clone, Copy)]
pub struct Unit<D: Dimensions> {
    pub(crate) prefix: i8,
    _phantom: PhantomData<D>,
}

impl<D: Dimensions> Unit<D> {
    pub const fn with_prefix(prefix: i8) -> Self {
        Self {
            prefix,
            _phantom: PhantomData,
        }
    }

    pub const fn base() -> Self {
        Self::with_prefix(0)
    }
}

include!(concat!(env!("OUT_DIR"), "/units_generated.rs"));
