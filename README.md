# danwi

> **Work in progress**

```rust
use danwi::prelude::*;

assert_eq!(5.0.m(), 5.0 * m);
assert_eq!(10.0.kg(), 10.0 * kg);
assert_eq!(2.0.s(), 2.0 * s);
assert_eq!(Length::kilometers(1.0), 1.0 * km);
assert_eq!(Voltage::millivolts(100.0), 0.1 * V);
assert_eq!(100.0 * mV, 0.1 * V);
assert_eq!(1.0.km() + 500.0.m(), 1500.0.m());
assert_eq!(1.0 * km + 500.0 * m, 1500.0 * m);
assert_eq!(1000.0.ms(), 1.0.s());
assert_eq!(1.0.hours(), 60.0.minutes());
assert_eq!(10.0 * kg * 9.8 * m_per_s2, 98.0 * N);
```
