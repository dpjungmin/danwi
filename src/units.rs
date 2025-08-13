use crate::{
    Sealed,
    dimension::{self, Dimension},
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

mod prefixes {
    pub(crate) const QUETTA: i8 = 30;
    pub(crate) const RONNA: i8 = 27;
    pub(crate) const YOTTA: i8 = 24;
    pub(crate) const ZETTA: i8 = 21;
    pub(crate) const EXA: i8 = 18;
    pub(crate) const PETA: i8 = 15;
    pub(crate) const TERA: i8 = 12;
    pub(crate) const GIGA: i8 = 9;
    pub(crate) const MEGA: i8 = 6;
    pub(crate) const KILO: i8 = 3;
    pub(crate) const HECTO: i8 = 2;
    pub(crate) const DECA: i8 = 1;
    pub(crate) const BASE: i8 = 0;
    pub(crate) const DECI: i8 = -1;
    pub(crate) const CENTI: i8 = -2;
    pub(crate) const MILLI: i8 = -3;
    pub(crate) const MICRO: i8 = -6;
    pub(crate) const NANO: i8 = -9;
    pub(crate) const PICO: i8 = -12;
    pub(crate) const FEMTO: i8 = -15;
    pub(crate) const ATTO: i8 = -18;
    pub(crate) const ZEPTO: i8 = -21;
    pub(crate) const YOCTO: i8 = -24;
    pub(crate) const RONTO: i8 = -27;
    pub(crate) const QUECTO: i8 = -30;
}

macro_rules! impl_unit {
    ($name:ident, $base:ident, $dimension:expr, $prefix:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;

        impl Unit for $name {
            const DIMENSION: Dimension = $dimension;
            const PREFIX: i8 = $prefix;
        }

        impl BaseUnit for $name {
            type Base = $base;
        }

        impl Mul<$name> for f32 {
            type Output = Quantity<F32Scalar, $name>;

            fn mul(self, _: $name) -> Self::Output {
                Quantity::new(F32Scalar::new(self))
            }
        }

        impl Mul<$name> for f64 {
            type Output = Quantity<F64Scalar, $name>;

            fn mul(self, _: $name) -> Self::Output {
                Quantity::new(F64Scalar::new(self))
            }
        }
    };
}

macro_rules! impl_units {
    {$($name:ident ($symbol:ident): $dimension:expr),* $(,)?} => {
        $(
            paste! {
                impl_unit!([<Quetta $name>], $name, $dimension, prefixes::QUETTA);
                impl_unit!([<Ronna $name>], $name, $dimension, prefixes::RONNA);
                impl_unit!([<Yotta $name>], $name, $dimension, prefixes::YOTTA);
                impl_unit!([<Zetta $name>], $name, $dimension, prefixes::ZETTA);
                impl_unit!([<Exa $name>], $name, $dimension, prefixes::EXA);
                impl_unit!([<Peta $name>], $name, $dimension, prefixes::PETA);
                impl_unit!([<Tera $name>], $name, $dimension, prefixes::TERA);
                impl_unit!([<Giga $name>], $name, $dimension, prefixes::GIGA);
                impl_unit!([<Mega $name>], $name, $dimension, prefixes::MEGA);
                impl_unit!([<Kilo $name>], $name, $dimension, prefixes::KILO);
                impl_unit!([<Hecto $name>], $name, $dimension, prefixes::HECTO);
                impl_unit!([<Deca $name>], $name, $dimension, prefixes::DECA);

                impl_unit!($name, $name, $dimension, prefixes::BASE);

                impl_unit!([<Deci $name>], $name, $dimension, prefixes::DECI);
                impl_unit!([<Centi $name>], $name, $dimension, prefixes::CENTI);
                impl_unit!([<Milli $name>], $name, $dimension, prefixes::MILLI);
                impl_unit!([<Micro $name>], $name, $dimension, prefixes::MICRO);
                impl_unit!([<Nano $name>], $name, $dimension, prefixes::NANO);
                impl_unit!([<Pico $name>], $name, $dimension, prefixes::PICO);
                impl_unit!([<Femto $name>], $name, $dimension, prefixes::FEMTO);
                impl_unit!([<Atto $name>], $name, $dimension, prefixes::ATTO);
                impl_unit!([<Zepto $name>], $name, $dimension, prefixes::ZEPTO);
                impl_unit!([<Yocto $name>], $name, $dimension, prefixes::YOCTO);
                impl_unit!([<Ronto $name>], $name, $dimension, prefixes::RONTO);
                impl_unit!([<Quecto $name>], $name, $dimension, prefixes::QUECTO);

                impl SameDimension<[<Quetta $name>], $name> for DimensionEq<[<Quetta $name>], $name> {}
                impl SameDimension<$name, [<Quetta $name>]> for DimensionEq<$name, [<Quetta $name>]> {}
                impl SameDimension<[<Ronna $name>], $name> for DimensionEq<[<Ronna $name>], $name> {}
                impl SameDimension<$name, [<Ronna $name>]> for DimensionEq<$name, [<Ronna $name>]> {}
                impl SameDimension<[<Yotta $name>], $name> for DimensionEq<[<Yotta $name>], $name> {}
                impl SameDimension<$name, [<Yotta $name>]> for DimensionEq<$name, [<Yotta $name>]> {}
                impl SameDimension<[<Zetta $name>], $name> for DimensionEq<[<Zetta $name>], $name> {}
                impl SameDimension<$name, [<Zetta $name>]> for DimensionEq<$name, [<Zetta $name>]> {}
                impl SameDimension<[<Exa $name>], $name> for DimensionEq<[<Exa $name>], $name> {}
                impl SameDimension<$name, [<Exa $name>]> for DimensionEq<$name, [<Exa $name>]> {}
                impl SameDimension<[<Peta $name>], $name> for DimensionEq<[<Peta $name>], $name> {}
                impl SameDimension<$name, [<Peta $name>]> for DimensionEq<$name, [<Peta $name>]> {}
                impl SameDimension<[<Tera $name>], $name> for DimensionEq<[<Tera $name>], $name> {}
                impl SameDimension<$name, [<Tera $name>]> for DimensionEq<$name, [<Tera $name>]> {}
                impl SameDimension<[<Giga $name>], $name> for DimensionEq<[<Giga $name>], $name> {}
                impl SameDimension<$name, [<Giga $name>]> for DimensionEq<$name, [<Giga $name>]> {}
                impl SameDimension<[<Mega $name>], $name> for DimensionEq<[<Mega $name>], $name> {}
                impl SameDimension<$name, [<Mega $name>]> for DimensionEq<$name, [<Mega $name>]> {}
                impl SameDimension<[<Kilo $name>], $name> for DimensionEq<[<Kilo $name>], $name> {}
                impl SameDimension<$name, [<Kilo $name>]> for DimensionEq<$name, [<Kilo $name>]> {}
                impl SameDimension<[<Hecto $name>], $name> for DimensionEq<[<Hecto $name>], $name> {}
                impl SameDimension<$name, [<Hecto $name>]> for DimensionEq<$name, [<Hecto $name>]> {}
                impl SameDimension<[<Deca $name>], $name> for DimensionEq<[<Deca $name>], $name> {}
                impl SameDimension<$name, [<Deca $name>]> for DimensionEq<$name, [<Deca $name>]> {}

                impl SameDimension<[<Deci $name>], $name> for DimensionEq<[<Deci $name>], $name> {}
                impl SameDimension<$name, [<Deci $name>]> for DimensionEq<$name, [<Deci $name>]> {}
                impl SameDimension<[<Centi $name>], $name> for DimensionEq<[<Centi $name>], $name> {}
                impl SameDimension<$name, [<Centi $name>]> for DimensionEq<$name, [<Centi $name>]> {}
                impl SameDimension<[<Milli $name>], $name> for DimensionEq<[<Milli $name>], $name> {}
                impl SameDimension<$name, [<Milli $name>]> for DimensionEq<$name, [<Milli $name>]> {}
                impl SameDimension<[<Micro $name>], $name> for DimensionEq<[<Micro $name>], $name> {}
                impl SameDimension<$name, [<Micro $name>]> for DimensionEq<$name, [<Micro $name>]> {}
                impl SameDimension<[<Nano $name>], $name> for DimensionEq<[<Nano $name>], $name> {}
                impl SameDimension<$name, [<Nano $name>]> for DimensionEq<$name, [<Nano $name>]> {}
                impl SameDimension<[<Pico $name>], $name> for DimensionEq<[<Pico $name>], $name> {}
                impl SameDimension<$name, [<Pico $name>]> for DimensionEq<$name, [<Pico $name>]> {}
                impl SameDimension<[<Femto $name>], $name> for DimensionEq<[<Femto $name>], $name> {}
                impl SameDimension<$name, [<Femto $name>]> for DimensionEq<$name, [<Femto $name>]> {}
                impl SameDimension<[<Atto $name>], $name> for DimensionEq<[<Atto $name>], $name> {}
                impl SameDimension<$name, [<Atto $name>]> for DimensionEq<$name, [<Atto $name>]> {}
                impl SameDimension<[<Zepto $name>], $name> for DimensionEq<[<Zepto $name>], $name> {}
                impl SameDimension<$name, [<Zepto $name>]> for DimensionEq<$name, [<Zepto $name>]> {}
                impl SameDimension<[<Yocto $name>], $name> for DimensionEq<[<Yocto $name>], $name> {}
                impl SameDimension<$name, [<Yocto $name>]> for DimensionEq<$name, [<Yocto $name>]> {}
                impl SameDimension<[<Ronto $name>], $name> for DimensionEq<[<Ronto $name>], $name> {}
                impl SameDimension<$name, [<Ronto $name>]> for DimensionEq<$name, [<Ronto $name>]> {}
                impl SameDimension<[<Quecto $name>], $name> for DimensionEq<[<Quecto $name>], $name> {}
                impl SameDimension<$name, [<Quecto $name>]> for DimensionEq<$name, [<Quecto $name>]> {}
            }
        )*

        pub mod constants {
            #![allow(non_upper_case_globals)]

            use super::*;

            $(
                paste! {
                    impl_units!(@constants $name, $symbol);
                }
            )*
        }

        pub mod types {
            use super::*;

            $(
                paste! {
                    impl_units!(@types $name);
                }
            )*
        }
    };

    (@constants $name:ident, $symbol:ident) => {
        paste! {
            pub const [<Q $symbol>]: [<Quetta $name>] = [<Quetta $name>];
            pub const [<R $symbol>]: [<Ronna $name>] = [<Ronna $name>];
            pub const [<Y $symbol>]: [<Yotta $name>] = [<Yotta $name>];
            pub const [<Z $symbol>]: [<Zetta $name>] = [<Zetta $name>];
            pub const [<E $symbol>]: [<Exa $name>] = [<Exa $name>];
            pub const [<P $symbol>]: [<Peta $name>] = [<Peta $name>];
            pub const [<T $symbol>]: [<Tera $name>] = [<Tera $name>];
            pub const [<G $symbol>]: [<Giga $name>] = [<Giga $name>];
            pub const [<M $symbol>]: [<Mega $name>] = [<Mega $name>];
            pub const [<k $symbol>]: [<Kilo $name>] = [<Kilo $name>];
            pub const [<h $symbol>]: [<Hecto $name>] = [<Hecto $name>];
            pub const [<da $symbol>]: [<Deca $name>] = [<Deca $name>];

            pub const $symbol: $name = $name;

            pub const [<d $symbol>]: [<Deci $name>] = [<Deci $name>];
            pub const [<c $symbol>]: [<Centi $name>] = [<Centi $name>];
            pub const [<m $symbol>]: [<Milli $name>] = [<Milli $name>];
            pub const [<u $symbol>]: [<Micro $name>] = [<Micro $name>];
            pub const [<n $symbol>]: [<Nano $name>] = [<Nano $name>];
            pub const [<p $symbol>]: [<Pico $name>] = [<Pico $name>];
            pub const [<f $symbol>]: [<Femto $name>] = [<Femto $name>];
            pub const [<a $symbol>]: [<Atto $name>] = [<Atto $name>];
            pub const [<z $symbol>]: [<Zepto $name>] = [<Zepto $name>];
            pub const [<y $symbol>]: [<Yocto $name>] = [<Yocto $name>];
            pub const [<r $symbol>]: [<Ronto $name>] = [<Ronto $name>];
            pub const [<q $symbol>]: [<Quecto $name>] = [<Quecto $name>];
        }
    };

    (@types $name:ident) => {
        paste! {
            // F64
            pub type [<F64Giga $name>] = Quantity<F64Scalar, super::[<Giga $name>]>;
            pub type [<F64Mega $name>] = Quantity<F64Scalar, super::[<Mega $name>]>;
            pub type [<F64Kilo $name>] = Quantity<F64Scalar, super::[<Kilo $name>]>;
            pub type [<F64 $name>] = Quantity<F64Scalar, super::$name>;
            pub type [<F64Milli $name>] = Quantity<F64Scalar, super::[<Milli $name>]>;
            pub type [<F64Micro $name>] = Quantity<F64Scalar, super::[<Micro $name>]>;
            pub type [<F64Nano $name>] = Quantity<F64Scalar, super::[<Nano $name>]>;

            // F32
            pub type [<F32Giga $name>] = Quantity<F32Scalar, super::[<Giga $name>]>;
            pub type [<F32Mega $name>] = Quantity<F32Scalar, super::[<Mega $name>]>;
            pub type [<F32Kilo $name>] = Quantity<F32Scalar, super::[<Kilo $name>]>;
            pub type [<F32 $name>] = Quantity<F32Scalar, super::$name>;
            pub type [<F32Milli $name>] = Quantity<F32Scalar, super::[<Milli $name>]>;
            pub type [<F32Micro $name>] = Quantity<F32Scalar, super::[<Micro $name>]>;
            pub type [<F32Nano $name>] = Quantity<F32Scalar, super::[<Nano $name>]>;
        }
    };
}

macro_rules! impl_multiply {
    ($($result:ty = $u1:ident * $u2:ident,)* $(;)?) => {
        $(
            impl Multiply<$u2> for $u1 { type Output = $result; }
            // commutative
            impl Multiply<$u1> for $u2 { type Output = $result; }
        )*
    };
}

macro_rules! impl_divide {
    ($($result:ty = $u1:ident / $u2:ident,)* $(;)?) => {
        $(
            impl Divide<$u2> for $u1 { type Output = $result; }
        )*
    };
}

impl_units! {
    Ampere (A): dimension::ELECTRIC_CURRENT,
    Volt (V): dimension::VOLTAGE,
    Ohms (Ohm): dimension::RESISTANCE,
}

impl_multiply! {
    Volt = Ampere * Ohms,
}

impl_divide! {
    Ampere = Volt / Ohms,
    Ohms = Volt / Ampere,
}
