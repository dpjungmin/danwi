#[macro_use]
mod macros;

use crate::{
    Sealed,
    dimension::{self, Dimension},
    prefixes,
    quantity::Quantity,
    scalar::{F32Scalar, F64Scalar},
};
use core::{marker::PhantomData, ops::Mul};
use paste::paste;

pub trait Unit: 'static + Copy {
    const DIMENSION: Dimension;
    const PREFIX: i8;
}

pub trait SameDimension<U1: Unit, U2: Unit> {}
pub struct DimensionEq<U1: Unit, U2: Unit>(PhantomData<(U1, U2)>);
impl<U: Unit> SameDimension<U, U> for DimensionEq<U, U> {}
impl<U1: Unit, U2: Unit> Sealed for DimensionEq<U1, U2> {}

pub trait BaseUnit: Unit {
    type Base: Unit;
}

pub trait Multiply<Rhs: Unit>: Unit {
    type Output: Unit;
}

pub trait Divide<Rhs: Unit>: Unit {
    type Output: Unit;
}

define_units! {
    base {
        Ampere (A): dimension::ELECTRIC_CURRENT,
        Volt (V): dimension::VOLTAGE,
        Ohms (Ohm): dimension::RESISTANCE,
    }

    mul {
        Volt = Ampere * Ohms,
    }

    div {
        Ampere = Volt / Ohms,
        Ohms = Volt / Ampere,
    }
}
