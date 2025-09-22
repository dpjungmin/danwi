macro_rules! define_units {
    ($($name:ident ($symbol:ident): $dimension:ty),* $(,)?) => {
        pub mod constants {
            use super::*;

            $(
                paste::paste! {
                    // constants
                    pub const [<Q $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::QUETTA);
                    pub const [<R $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::RONNA);
                    pub const [<Y $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::YOTTA);
                    pub const [<Z $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::ZETTA);
                    pub const [<E $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::EXA);
                    pub const [<P $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::PETA);
                    pub const [<T $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::TERA);
                    pub const [<G $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::GIGA);
                    pub const [<M $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::MEGA);
                    pub const [<k $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::KILO);
                    pub const [<h $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::HECTO);
                    pub const [<da $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::DECA);
                    pub const $symbol: Unit<$dimension> = Unit::base();
                    pub const [<d $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::DECI);
                    pub const [<c $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::CENTI);
                    pub const [<m $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::MILLI);
                    pub const [<u $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::MICRO);
                    pub const [<n $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::NANO);
                    pub const [<p $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::PICO);
                    pub const [<f $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::FEMTO);
                    // keyword collision for atto second (as)
                    pub const [<atto $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::ATTO);
                    pub const [<z $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::ZEPTO);
                    pub const [<y $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::YOCTO);
                    pub const [<r $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::RONTO);
                    pub const [<q $symbol>]: Unit<$dimension> = Unit::with_prefix(prefix::QUECTO);
                }
            )*
        }

        pub mod types {
            pub use super::*;

            #[cfg(feature = "f32")]
            pub mod f32 {
                pub use super::*;
                pub use crate::{F32Scalar, Quantity};

                $(
                    paste::paste! {
                        pub type [<$name:camel>] = Quantity<F32Scalar, $dimension>;
                    }
                )*
            }

            #[cfg(feature = "f64")]
            pub mod f64 {
                pub use super::*;
                pub use crate::{F64Scalar, Quantity};

                $(
                    paste::paste! {
                        pub type [<$name:camel>] = Quantity<F64Scalar, $dimension>;
                    }
                )*
            }
        }

        pub mod ext {
            #![allow(non_snake_case)]

            use super::{*, constants::*};

            paste::paste! {
                #[cfg(feature = "f32")]
                pub trait F32QuantityExt {
                    $(
                        fn [<Q $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<R $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<Y $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<Z $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<E $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<P $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<T $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<G $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<M $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<k $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<h $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<da $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn $symbol(self) -> Quantity<F32Scalar, $dimension>;
                        fn [<d $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<c $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<m $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<u $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<n $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<p $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<f $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        // keyword collision for atto second (as)
                        fn [<atto $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<z $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<y $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<r $symbol>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<q $symbol>](self) -> Quantity<F32Scalar, $dimension>;

                        fn [<quetta $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<ronna $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<yotta $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<zetta $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<exa $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<peta $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<tera $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<giga $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<mega $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<kilo $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<hecto $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<deca $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<$name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<deci $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<centi $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<milli $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<micro $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<nano $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<pico $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<femto $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<atto $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<zepto $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<yocto $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<ronto $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                        fn [<quecto $name:lower>](self) -> Quantity<F32Scalar, $dimension>;
                    )*
                }

                #[cfg(feature = "f64")]
                pub trait F64QuantityExt {
                    $(
                        fn [<Q $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<R $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<Y $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<Z $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<E $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<P $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<T $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<G $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<M $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<k $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<h $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<da $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn $symbol(self) -> Quantity<F64Scalar, $dimension>;
                        fn [<d $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<c $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<m $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<u $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<n $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<p $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<f $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        // keyword collision for atto second (as)
                        fn [<atto $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<z $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<y $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<r $symbol>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<q $symbol>](self) -> Quantity<F64Scalar, $dimension>;

                        fn [<quetta $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<ronna $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<yotta $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<zetta $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<exa $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<peta $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<tera $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<giga $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<mega $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<kilo $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<hecto $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<deca $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<$name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<deci $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<centi $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<milli $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<micro $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<nano $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<pico $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<femto $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<atto $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<zepto $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<yocto $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<ronto $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                        fn [<quecto $name:lower>](self) -> Quantity<F64Scalar, $dimension>;
                    )*
                }
            }

            paste::paste! {
                #[cfg(feature = "f32")]
                impl F32QuantityExt for f32 {
                    $(
                        fn [<Q $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<Q $symbol>]
                        }
                        fn [<R $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<R $symbol>]
                        }
                        fn [<Y $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<Y $symbol>]
                        }
                        fn [<Z $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<Z $symbol>]
                        }
                        fn [<E $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<E $symbol>]
                        }
                        fn [<P $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<P $symbol>]
                        }
                        fn [<T $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<T $symbol>]
                        }
                        fn [<G $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<G $symbol>]
                        }
                        fn [<M $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<M $symbol>]
                        }
                        fn [<k $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<k $symbol>]
                        }
                        fn [<h $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<h $symbol>]
                        }
                        fn [<da $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<da $symbol>]
                        }
                        fn $symbol(self) -> Quantity<F32Scalar, $dimension> {
                            self * $symbol
                        }
                        fn [<d $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<d $symbol>]
                        }
                        fn [<c $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<c $symbol>]
                        }
                        fn [<m $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<m $symbol>]
                        }
                        fn [<u $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<u $symbol>]
                        }
                        fn [<n $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<n $symbol>]
                        }
                        fn [<p $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<p $symbol>]
                        }
                        fn [<f $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<f $symbol>]
                        }
                        fn [<atto $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<atto $symbol>]
                        }
                        fn [<z $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<z $symbol>]
                        }
                        fn [<y $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<y $symbol>]
                        }
                        fn [<r $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<r $symbol>]
                        }
                        fn [<q $symbol>](self) -> Quantity<F32Scalar, $dimension> {
                            self * [<q $symbol>]
                        }

                        fn [<quetta $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<Q $symbol>]()
                        }
                        fn [<ronna $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<R $symbol>]()
                        }
                        fn [<yotta $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<Y $symbol>]()
                        }
                        fn [<zetta $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<Z $symbol>]()
                        }
                        fn [<exa $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<E $symbol>]()
                        }
                        fn [<peta $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<P $symbol>]()
                        }
                        fn [<tera $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<T $symbol>]()
                        }
                        fn [<giga $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<G $symbol>]()
                        }
                        fn [<mega $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<M $symbol>]()
                        }
                        fn [<kilo $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<k $symbol>]()
                        }
                        fn [<hecto $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<h $symbol>]()
                        }
                        fn [<deca $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<da $symbol>]()
                        }
                        fn [<$name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.$symbol()
                        }
                        fn [<deci $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<d $symbol>]()
                        }
                        fn [<centi $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<c $symbol>]()
                        }
                        fn [<milli $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<m $symbol>]()
                        }
                        fn [<micro $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<u $symbol>]()
                        }
                        fn [<nano $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<n $symbol>]()
                        }
                        fn [<pico $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<p $symbol>]()
                        }
                        fn [<femto $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<f $symbol>]()
                        }
                        fn [<atto $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<atto $symbol>]()
                        }
                        fn [<zepto $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<z $symbol>]()
                        }
                        fn [<yocto $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<y $symbol>]()
                        }
                        fn [<ronto $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<r $symbol>]()
                        }
                        fn [<quecto $name:lower>](self) -> Quantity<F32Scalar, $dimension> {
                            self.[<q $symbol>]()
                        }
                    )*
                }

                #[cfg(feature = "f64")]
                impl F64QuantityExt for f64 {
                    $(
                        fn [<Q $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<Q $symbol>]
                        }
                        fn [<R $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<R $symbol>]
                        }
                        fn [<Y $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<Y $symbol>]
                        }
                        fn [<Z $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<Z $symbol>]
                        }
                        fn [<E $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<E $symbol>]
                        }
                        fn [<P $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<P $symbol>]
                        }
                        fn [<T $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<T $symbol>]
                        }
                        fn [<G $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<G $symbol>]
                        }
                        fn [<M $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<M $symbol>]
                        }
                        fn [<k $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<k $symbol>]
                        }
                        fn [<h $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<h $symbol>]
                        }
                        fn [<da $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<da $symbol>]
                        }
                        fn $symbol(self) -> Quantity<F64Scalar, $dimension> {
                            self * $symbol
                        }
                        fn [<d $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<d $symbol>]
                        }
                        fn [<c $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<c $symbol>]
                        }
                        fn [<m $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<m $symbol>]
                        }
                        fn [<u $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<u $symbol>]
                        }
                        fn [<n $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<n $symbol>]
                        }
                        fn [<p $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<p $symbol>]
                        }
                        fn [<f $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<f $symbol>]
                        }
                        fn [<atto $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<atto $symbol>]
                        }
                        fn [<z $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<z $symbol>]
                        }
                        fn [<y $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<y $symbol>]
                        }
                        fn [<r $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<r $symbol>]
                        }
                        fn [<q $symbol>](self) -> Quantity<F64Scalar, $dimension> {
                            self * [<q $symbol>]
                        }

                        fn [<quetta $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<Q $symbol>]()
                        }
                        fn [<ronna $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<R $symbol>]()
                        }
                        fn [<yotta $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<Y $symbol>]()
                        }
                        fn [<zetta $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<Z $symbol>]()
                        }
                        fn [<exa $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<E $symbol>]()
                        }
                        fn [<peta $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<P $symbol>]()
                        }
                        fn [<tera $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<T $symbol>]()
                        }
                        fn [<giga $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<G $symbol>]()
                        }
                        fn [<mega $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<M $symbol>]()
                        }
                        fn [<kilo $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<k $symbol>]()
                        }
                        fn [<hecto $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<h $symbol>]()
                        }
                        fn [<deca $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<da $symbol>]()
                        }
                        fn [<$name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.$symbol()
                        }
                        fn [<deci $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<d $symbol>]()
                        }
                        fn [<centi $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<c $symbol>]()
                        }
                        fn [<milli $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<m $symbol>]()
                        }
                        fn [<micro $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<u $symbol>]()
                        }
                        fn [<nano $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<n $symbol>]()
                        }
                        fn [<pico $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<p $symbol>]()
                        }
                        fn [<femto $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<f $symbol>]()
                        }
                        fn [<atto $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<atto $symbol>]()
                        }
                        fn [<zepto $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<z $symbol>]()
                        }
                        fn [<yocto $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<y $symbol>]()
                        }
                        fn [<ronto $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<r $symbol>]()
                        }
                        fn [<quecto $name:lower>](self) -> Quantity<F64Scalar, $dimension> {
                            self.[<q $symbol>]()
                        }
                    )*
                }
            }
        }
    };
}
