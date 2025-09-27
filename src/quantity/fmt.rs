use super::Quantity;
use crate::{dimension::Dimensions, scalar::Scalar};
use core::fmt;

impl<S: Scalar, D: Dimensions> fmt::Display for Quantity<S, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
