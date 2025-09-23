use danwi::f64::{
    QuantityExt,
    constants::{A, V, mA, mV},
    types::Volt,
};

fn scale_adc_counts_to_e(count: u16) -> Volt {
    let v_ref = 3.3.V();
    v_ref * (count as f64 / 4096.0)
}

fn main() {
    let e = scale_adc_counts_to_e(2048);
    println!("{:.3} V", e); // 1.65 V
    println!("{:.3} mV", e.to(mV)); // 1650 mV

    let i = 50.0.mV() / 0.1.Ohm();
    println!("{:.3} mA", i.to(mA)); // 500 mA

    let p = (12.0 * V) * (0.5 * A);
    println!("{:.3} W", p); // 6 W

    let v_in = 5.0.V();
    let r_top = 2.2.kOhm();
    let r_bottom = 3.3.kOhm();
    let v_out = v_in * (r_bottom / (r_top + r_bottom));
    println!("{:.3} V", v_out); // 3 V
}
