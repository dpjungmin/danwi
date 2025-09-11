use approx::{assert_abs_diff_eq, assert_relative_eq};
use danwi::prelude::*;

fn main() {
    ohms_law();
    power_calculations();
    rc_circuit();
    capacitor_calculations();
    unit_conversions();
    frequency_and_time();
}

fn ohms_law() {
    let current = 20.0.mA();
    let resistance = 470.0.Ohm();
    let voltage = current * resistance;
    assert_relative_eq!(voltage, 9.4.V(), epsilon = 1e-10);

    let voltage = 3.3.V();
    let resistance = 10.0.kOhm();
    let current = voltage / resistance;
    assert_relative_eq!(current, 0.33.mA(), epsilon = 1e-10);

    let voltage = 12.0.V();
    let current = 2.5.A();
    let resistance = voltage / current;
    assert_relative_eq!(resistance, 4.8.Ohm(), epsilon = 1e-10);
}

fn power_calculations() {
    let voltage = 5.0.V();
    let current = 500.0.mA();
    let power = voltage * current;
    assert_relative_eq!(power, 2.5.W(), epsilon = 1e-10);

    let energy = 3600.0.J();
    let time = 1.0.s();
    let power = energy / time;
    assert_relative_eq!(power, 3.6.kW(), epsilon = 1e-10);

    let power = 100.0.W();
    let time = 3600.0.s();
    let energy = power * time;
    assert_relative_eq!(energy, 360.0.kJ(), epsilon = 1e-10);
}

fn rc_circuit() {
    let resistance = 1.0.kOhm();
    let capacitance = 1.0.mF();
    let time_constant = resistance * capacitance;
    assert_abs_diff_eq!(time_constant, 1.0.s(), epsilon = 1e-10);

    let capacitance = 10.0.nF();
    let resistance = 100.0.kOhm();
    let time_constant = capacitance * resistance;
    assert_abs_diff_eq!(time_constant, 1.0.ms(), epsilon = 1e-10);
}

fn capacitor_calculations() {
    let capacitance = 100.0.uF();
    let voltage = 5.0.V();
    let charge = capacitance * voltage;
    assert_relative_eq!(charge, 500.0.uC(), epsilon = 1e-10);

    let charge = 1.0.C();
    let voltage = 10.0.V();
    let capacitance = charge / voltage;
    assert_relative_eq!(capacitance, 100.0.mF(), epsilon = 1e-10);
}

fn unit_conversions() {
    let voltage = 3.0.V();
    assert_eq!(voltage.to::<MilliVolt>(), 3000.0 * mV);
    assert_eq!(voltage.to::<MicroVolt>(), 3000000.0 * uV);

    let current = 5.0.mA();
    assert_eq!(current.to::<Ampere>(), 0.005 * A);
    let current_base = current.to::<Ampere>();
    assert_eq!(current_base.to::<MicroAmpere>(), 5000.0 * uA);

    let resistance = 2.0.kOhm();
    assert_eq!(resistance.to::<Ohms>(), 2000.0 * Ohm);
    let resistance_base = resistance.to::<Ohms>();
    assert_eq!(resistance_base.to::<MegaOhms>(), 0.002 * MOhm);

    let capacitance = 1.0.uF();
    assert_eq!(capacitance.to::<Farad>(), 0.000001 * F);
    let capacitance_base = capacitance.to::<Farad>();
    assert_eq!(capacitance_base.to::<NanoFarad>(), 1000.0 * nF);
    assert_eq!(capacitance_base.to::<PicoFarad>(), 1000000.0 * pF);
}

fn frequency_and_time() {
    let frequency = 1000.0.Hz();
    assert_eq!(frequency.to::<KiloHertz>(), 1.0 * kHz);
    assert_eq!(frequency.to::<MegaHertz>(), 0.001 * MHz);

    let another_freq = 1.0.kHz();
    assert_eq!(another_freq.to::<Hertz>(), 1000.0 * Hz);

    let time = 0.001.s();
    assert_eq!(time.to::<MilliSecond>(), 1.0 * ms);
    assert_eq!(time.to::<MicroSecond>(), 1000.0 * us);
}
