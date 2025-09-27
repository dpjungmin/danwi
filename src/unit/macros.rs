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
                pub use crate::Quantity;

                $(
                    paste::paste! {
                        pub type [<$name:camel>] = Quantity<f32, $dimension>;
                    }
                )*
            }

            #[cfg(feature = "f64")]
            pub mod f64 {
                pub use super::*;
                pub use crate::Quantity;

                $(
                    paste::paste! {
                        pub type [<$name:camel>] = Quantity<f64, $dimension>;
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
                        fn [<Q $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<R $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<Y $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<Z $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<E $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<P $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<T $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<G $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<M $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<k $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<h $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<da $symbol>](self) -> Quantity<f32, $dimension>;
                        fn $symbol(self) -> Quantity<f32, $dimension>;
                        fn [<d $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<c $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<m $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<u $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<n $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<p $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<f $symbol>](self) -> Quantity<f32, $dimension>;
                        // keyword collision for atto second (as)
                        fn [<atto $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<z $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<y $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<r $symbol>](self) -> Quantity<f32, $dimension>;
                        fn [<q $symbol>](self) -> Quantity<f32, $dimension>;

                        fn [<quetta $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<ronna $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<yotta $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<zetta $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<exa $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<peta $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<tera $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<giga $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<mega $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<kilo $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<hecto $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<deca $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<$name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<deci $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<centi $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<milli $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<micro $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<nano $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<pico $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<femto $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<atto $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<zepto $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<yocto $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<ronto $name:lower>](self) -> Quantity<f32, $dimension>;
                        fn [<quecto $name:lower>](self) -> Quantity<f32, $dimension>;
                    )*
                }

                #[cfg(feature = "f64")]
                pub trait F64QuantityExt {
                    $(
                        fn [<Q $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<R $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<Y $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<Z $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<E $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<P $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<T $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<G $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<M $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<k $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<h $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<da $symbol>](self) -> Quantity<f64, $dimension>;
                        fn $symbol(self) -> Quantity<f64, $dimension>;
                        fn [<d $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<c $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<m $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<u $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<n $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<p $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<f $symbol>](self) -> Quantity<f64, $dimension>;
                        // keyword collision for atto second (as)
                        fn [<atto $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<z $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<y $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<r $symbol>](self) -> Quantity<f64, $dimension>;
                        fn [<q $symbol>](self) -> Quantity<f64, $dimension>;

                        fn [<quetta $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<ronna $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<yotta $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<zetta $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<exa $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<peta $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<tera $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<giga $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<mega $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<kilo $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<hecto $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<deca $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<$name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<deci $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<centi $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<milli $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<micro $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<nano $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<pico $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<femto $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<atto $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<zepto $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<yocto $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<ronto $name:lower>](self) -> Quantity<f64, $dimension>;
                        fn [<quecto $name:lower>](self) -> Quantity<f64, $dimension>;
                    )*
                }
            }

            paste::paste! {
                #[cfg(feature = "f32")]
                impl F32QuantityExt for f32 {
                    $(
                        fn [<Q $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<Q $symbol>]
                        }
                        fn [<R $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<R $symbol>]
                        }
                        fn [<Y $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<Y $symbol>]
                        }
                        fn [<Z $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<Z $symbol>]
                        }
                        fn [<E $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<E $symbol>]
                        }
                        fn [<P $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<P $symbol>]
                        }
                        fn [<T $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<T $symbol>]
                        }
                        fn [<G $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<G $symbol>]
                        }
                        fn [<M $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<M $symbol>]
                        }
                        fn [<k $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<k $symbol>]
                        }
                        fn [<h $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<h $symbol>]
                        }
                        fn [<da $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<da $symbol>]
                        }
                        fn $symbol(self) -> Quantity<f32, $dimension> {
                            self * $symbol
                        }
                        fn [<d $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<d $symbol>]
                        }
                        fn [<c $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<c $symbol>]
                        }
                        fn [<m $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<m $symbol>]
                        }
                        fn [<u $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<u $symbol>]
                        }
                        fn [<n $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<n $symbol>]
                        }
                        fn [<p $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<p $symbol>]
                        }
                        fn [<f $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<f $symbol>]
                        }
                        fn [<atto $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<atto $symbol>]
                        }
                        fn [<z $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<z $symbol>]
                        }
                        fn [<y $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<y $symbol>]
                        }
                        fn [<r $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<r $symbol>]
                        }
                        fn [<q $symbol>](self) -> Quantity<f32, $dimension> {
                            self * [<q $symbol>]
                        }

                        fn [<quetta $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<Q $symbol>]()
                        }
                        fn [<ronna $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<R $symbol>]()
                        }
                        fn [<yotta $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<Y $symbol>]()
                        }
                        fn [<zetta $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<Z $symbol>]()
                        }
                        fn [<exa $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<E $symbol>]()
                        }
                        fn [<peta $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<P $symbol>]()
                        }
                        fn [<tera $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<T $symbol>]()
                        }
                        fn [<giga $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<G $symbol>]()
                        }
                        fn [<mega $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<M $symbol>]()
                        }
                        fn [<kilo $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<k $symbol>]()
                        }
                        fn [<hecto $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<h $symbol>]()
                        }
                        fn [<deca $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<da $symbol>]()
                        }
                        fn [<$name:lower>](self) -> Quantity<f32, $dimension> {
                            self.$symbol()
                        }
                        fn [<deci $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<d $symbol>]()
                        }
                        fn [<centi $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<c $symbol>]()
                        }
                        fn [<milli $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<m $symbol>]()
                        }
                        fn [<micro $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<u $symbol>]()
                        }
                        fn [<nano $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<n $symbol>]()
                        }
                        fn [<pico $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<p $symbol>]()
                        }
                        fn [<femto $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<f $symbol>]()
                        }
                        fn [<atto $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<atto $symbol>]()
                        }
                        fn [<zepto $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<z $symbol>]()
                        }
                        fn [<yocto $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<y $symbol>]()
                        }
                        fn [<ronto $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<r $symbol>]()
                        }
                        fn [<quecto $name:lower>](self) -> Quantity<f32, $dimension> {
                            self.[<q $symbol>]()
                        }
                    )*
                }

                #[cfg(feature = "f64")]
                impl F64QuantityExt for f64 {
                    $(
                        fn [<Q $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<Q $symbol>]
                        }
                        fn [<R $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<R $symbol>]
                        }
                        fn [<Y $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<Y $symbol>]
                        }
                        fn [<Z $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<Z $symbol>]
                        }
                        fn [<E $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<E $symbol>]
                        }
                        fn [<P $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<P $symbol>]
                        }
                        fn [<T $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<T $symbol>]
                        }
                        fn [<G $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<G $symbol>]
                        }
                        fn [<M $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<M $symbol>]
                        }
                        fn [<k $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<k $symbol>]
                        }
                        fn [<h $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<h $symbol>]
                        }
                        fn [<da $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<da $symbol>]
                        }
                        fn $symbol(self) -> Quantity<f64, $dimension> {
                            self * $symbol
                        }
                        fn [<d $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<d $symbol>]
                        }
                        fn [<c $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<c $symbol>]
                        }
                        fn [<m $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<m $symbol>]
                        }
                        fn [<u $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<u $symbol>]
                        }
                        fn [<n $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<n $symbol>]
                        }
                        fn [<p $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<p $symbol>]
                        }
                        fn [<f $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<f $symbol>]
                        }
                        fn [<atto $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<atto $symbol>]
                        }
                        fn [<z $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<z $symbol>]
                        }
                        fn [<y $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<y $symbol>]
                        }
                        fn [<r $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<r $symbol>]
                        }
                        fn [<q $symbol>](self) -> Quantity<f64, $dimension> {
                            self * [<q $symbol>]
                        }

                        fn [<quetta $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<Q $symbol>]()
                        }
                        fn [<ronna $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<R $symbol>]()
                        }
                        fn [<yotta $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<Y $symbol>]()
                        }
                        fn [<zetta $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<Z $symbol>]()
                        }
                        fn [<exa $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<E $symbol>]()
                        }
                        fn [<peta $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<P $symbol>]()
                        }
                        fn [<tera $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<T $symbol>]()
                        }
                        fn [<giga $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<G $symbol>]()
                        }
                        fn [<mega $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<M $symbol>]()
                        }
                        fn [<kilo $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<k $symbol>]()
                        }
                        fn [<hecto $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<h $symbol>]()
                        }
                        fn [<deca $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<da $symbol>]()
                        }
                        fn [<$name:lower>](self) -> Quantity<f64, $dimension> {
                            self.$symbol()
                        }
                        fn [<deci $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<d $symbol>]()
                        }
                        fn [<centi $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<c $symbol>]()
                        }
                        fn [<milli $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<m $symbol>]()
                        }
                        fn [<micro $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<u $symbol>]()
                        }
                        fn [<nano $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<n $symbol>]()
                        }
                        fn [<pico $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<p $symbol>]()
                        }
                        fn [<femto $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<f $symbol>]()
                        }
                        fn [<atto $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<atto $symbol>]()
                        }
                        fn [<zepto $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<z $symbol>]()
                        }
                        fn [<yocto $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<y $symbol>]()
                        }
                        fn [<ronto $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<r $symbol>]()
                        }
                        fn [<quecto $name:lower>](self) -> Quantity<f64, $dimension> {
                            self.[<q $symbol>]()
                        }
                    )*
                }
            }
        }
    };
}
