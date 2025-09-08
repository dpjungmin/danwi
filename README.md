> **Work in progress**

# danwi

A dimensional analysis library for Rust with SI units, compile-time type
checking, and `no_std` support.

## Usage

```rust
use danwi::prelude::*;
use danwi::quantity::Quantity;
use danwi::scalar::F64Scalar;

// create a quantity by multiplying units
let v = (5.0_f64 * mA) * (2.0 * kOhm);
assert_eq!(v, 10.0 * V);
assert_eq!(v, 10.0.V()); // create a quantity using the extension trait
assert_eq!(v, 5.0.mA() * 2.0.kOhm());
assert_eq!(v, Quantity::<F64Scalar, Volt>::from(10.0)); // create a quantity directly
assert_eq!(v, F64Volt::from(10.0)); // create a quantity using type alias

// convert between prefixes
let mv = v.to::<MilliVolt>();
assert_eq!(mv, 10000.0 * mV);
assert_eq!(mv, 10.0 * V);
assert_eq!(mv, 0.01 * kV);

let i = v / (2.0 * kOhm);
assert_eq!(i, 5.0 * mA);
```

## TODO

- add more SI derived units (i.e., power, energy, force, etc.)
- implement `Display` trait for pretty-printing quantities with units
- add common mathematical operations (`abs`, `powi`, `sqrt`)
- add feature flags for conditional compilation
  - unit categories (electrical, mechanical, etc.)
  - prefix sets (common vs. all)

## Future

Use packed dimensions as generic consts to perform computation with const
generic parameters (once `generic_const_exprs` feature stabilizes).

```rust,ignore
// now: manual implementation for every unit combination
impl Multiply<Ohms> for Ampere {
    type Output = Volt;
}

// future: automatic dimension calculation
impl<const D1: u128, const D2: u128> Mul<Unit<D2>> for Unit<D1> {
    type Output = Unit<{D1 + D2}>;  // requires generic_const_exprs
}
```

- RFC: https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md
- Tracking issue: https://github.com/rust-lang/rust/issues/76560
