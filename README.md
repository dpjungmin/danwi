# danwi

A dimensional analysis library for Rust with SI units, compile-time type
checking, and `no_std` support.

## Usage

```rust
use danwi::prelude::*;

// create a quantity by multiplying units
let v = (5.0_f64 * mA) * (2.0_f64 * kOhm);
assert_eq!(v, 10.0 * V);
assert_eq!(v.value(), 10.0);
assert_eq!(v, F64Volt::from(10.0)); // create a quantity using type alias

// convert between prefixes
let mv = v.to(mV);
assert_eq!(mv.value(), 10000.0);
assert_eq!(mv, 10000.0 * mV);
assert_eq!(mv, 10.0 * V);
assert_eq!(mv, 0.01 * kV);

let i = v / (2.0 * kOhm);
assert_eq!(i, 5.0 * mA);

let t = (1.0_f64 * s) + (1e3 * ms) + (1e6 * us) + (1e9 * ns);
assert_eq!(t, 4.0 * s);
assert_eq!(t / (2.0 * s), 2.0);
assert_eq!(t / (2e3 * ms), 2.0);

let period = 1.0 * s;
let freq = 1.0 / period;
assert_eq!(freq, 1.0 * Hz);

let a = F64Metre::from(100.0);
let b = 50.0 * cm;
let c = 0.001 * km;
let len = a + b + c;
println!("Length: {} m", len.value()); // Length: 101.5 m
println!("Length: {} m", len); // Length: 101.5 m
println!("Length: {} cm", len.to(cm)); // Length: 10150 cm
println!("Length: {} km", len.to(km)); // Length: 0.1015 km
```
