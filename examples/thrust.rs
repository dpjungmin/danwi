use danwi::f64::{QuantityExt, constants::MN};

fn main() {
    let mdot = 680.0.kg() / 1.0.s(); // mass flow rate (ṁ)
    let isp = 350.0.s(); // specific impulse
    let g0 = 9.80665.mps2(); // standard gravity
    let v_e = isp * g0; // exhaust velocity
    let thrust = mdot * v_e; // F = ṁ * v_e

    println!("Thrust: {:.0} MN", thrust.to(MN));

    let burn_time = 360.0.s();
    let prop = mdot * burn_time; // kg

    println!("Propellant for 6 m burn: {} tons", prop.value() / 1000.0);
}
