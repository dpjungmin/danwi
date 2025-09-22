use danwi::prelude::*;

fn main() {
    let adc_count = 2048_u16;
    let v_ref = 3.3.V();
    let v = v_ref * (adc_count as f64 / 4096.0);
    println!("{:.3} V", v); // 1.65 V
    println!("{:.3} mV", v.to(mV)); // 1650 mV

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
