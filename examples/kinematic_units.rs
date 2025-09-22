use danwi::f32::{QuantityExt, constants::kmps2};

fn main() {
    let velocity = 10.0.mps(); // 10 m/s
    let acceleration = 2.0.mps2(); // 2 m/s²
    let time = 5.0.s(); // 5 s

    assert_eq!(velocity + (acceleration * time), 20.0.mps());

    let distance = velocity * time + 0.5 * acceleration * (time * time);
    assert_eq!(distance, 75.0.m());

    let accel_in_km = acceleration.to(kmps2);
    assert_eq!(accel_in_km, 0.002.kmps2());
}
