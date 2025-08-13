# danwi

> **Work in progress**

```rust
use danwi::prelude::*;

let v = (5.0_f64 * mA) * (2.0 * kOhm);
assert_eq!(v, 10.0 * V);
assert_eq!(v, F64Volt::from(10.0));

let mv = v.to::<MilliVolt>();
assert_eq!(mv, 10000.0 * mV);
assert_eq!(mv, 10.0 * V);
assert_eq!(mv, 0.01 * kV);

let a = v / (2.0 * kOhm);
assert_eq!(a, 5.0 * mA);
```

## TODO

- add extension traits for directly converting scalar types to quantity types
- try packed dimension for generic consts (show a proof of concept impl)
