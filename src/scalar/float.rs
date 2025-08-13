use super::Scalar;

macro_rules! impl_float_scalar {
    ($name:ident, $type:ty) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct $name(pub $type);

        impl $crate::sealed::Sealed for $name {}

        impl $name {
            pub const fn new(value: $type) -> Self {
                Self(value)
            }
        }

        impl Scalar for $name {
            type Value = $type;

            fn get(&self) -> Self::Value {
                self.0
            }

            fn scale_by_power_of_10(&self, exponent: i8) -> Self {
                let factor = libm::exp10(exponent as _) as $type;
                Self(self.0 * factor)
            }

            fn add(&self, other: &Self) -> Self {
                Self(self.0 + other.0)
            }

            fn sub(&self, other: &Self) -> Self {
                Self(self.0 - other.0)
            }

            fn mul(&self, other: &Self) -> Self {
                Self(self.0 * other.0)
            }

            fn div(&self, other: &Self) -> Self {
                Self(self.0 / other.0)
            }

            fn neg(&self) -> Self {
                Self(-self.0)
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $type {
            fn from(value: $name) -> $type {
                value.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self(0.0)
            }
        }
    };
}

impl_float_scalar!(F32Scalar, f32);
impl_float_scalar!(F64Scalar, f64);
