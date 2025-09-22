use super::Quantity;
use crate::{dimension::Dimensions, scalar::Scalar};
use core::fmt;

impl<S, D> fmt::Display for Quantity<S, D>
where
    S: Scalar,
    S::Value: fmt::Display,
    D: Dimensions,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
