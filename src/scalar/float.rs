use super::Scalar;

impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }

    fn scale_by_power_of_10(&self, exponent: i8) -> Self {
        let factor = libm::exp10(exponent as _);
        self * factor
    }
}

impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }

    fn scale_by_power_of_10(&self, exponent: i8) -> Self {
        let factor = libm::exp10(exponent as _) as f32;
        self * factor
    }
}
