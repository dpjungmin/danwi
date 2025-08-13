//! Extension traits for natural, ergonomic API.

#![allow(non_snake_case)]

use crate::units::*;

/// Extension trait for creating quantities from numeric values.
pub trait QuantityExtensions {
    // Length - concise
    fn m(self) -> Length;
    fn km(self) -> Length;
    fn cm(self) -> Length;
    fn mm(self) -> Length;
    fn um(self) -> Length; // micrometers
    fn nm(self) -> Length; // nanometers
    fn ft(self) -> Length;
    fn inch(self) -> Length;
    fn mi(self) -> Length; // miles

    // Length - clear aliases
    fn meters(self) -> Length;
    fn kilometers(self) -> Length;
    fn centimeters(self) -> Length;
    fn millimeters(self) -> Length;
    fn micrometers(self) -> Length;
    fn nanometers(self) -> Length;
    fn feet(self) -> Length;
    fn inches(self) -> Length;
    fn miles(self) -> Length;

    // Mass - concise
    fn kg(self) -> Mass;
    fn g(self) -> Mass;
    fn mg(self) -> Mass;
    fn ug(self) -> Mass; // micrograms
    fn lb(self) -> Mass;
    fn oz(self) -> Mass;

    // Mass - clear aliases
    fn kilograms(self) -> Mass;
    fn grams(self) -> Mass;
    fn milligrams(self) -> Mass;
    fn micrograms(self) -> Mass;
    fn pounds(self) -> Mass;
    fn ounces(self) -> Mass;

    // Time - concise
    fn s(self) -> Time;
    fn ms(self) -> Time;
    fn us(self) -> Time; // microseconds
    fn ns(self) -> Time;
    fn min_time(self) -> Time; // renamed to avoid conflict with f64::min
    fn h(self) -> Time;
    fn hr(self) -> Time; // alternative to h

    // Time - clear aliases
    fn seconds(self) -> Time;
    fn milliseconds(self) -> Time;
    fn microseconds(self) -> Time;
    fn nanoseconds(self) -> Time;
    fn minutes(self) -> Time;
    fn hours(self) -> Time;

    // Electric Current - concise
    fn A(self) -> ElectricCurrent;
    fn mA(self) -> ElectricCurrent;
    fn uA(self) -> ElectricCurrent; // microamperes

    // Electric Current - clear aliases
    fn amperes(self) -> ElectricCurrent;
    fn milliamperes(self) -> ElectricCurrent;
    fn microamperes(self) -> ElectricCurrent;

    // Voltage - concise
    fn V(self) -> Voltage;
    fn mV(self) -> Voltage;
    fn uV(self) -> Voltage; // microvolts
    fn kV(self) -> Voltage;
    fn MV(self) -> Voltage; // megavolts

    // Voltage - clear aliases
    fn volts(self) -> Voltage;
    fn millivolts(self) -> Voltage;
    fn microvolts(self) -> Voltage;
    fn kilovolts(self) -> Voltage;
    fn megavolts(self) -> Voltage;

    // Resistance - concise
    fn Ohm(self) -> Resistance;
    fn kOhm(self) -> Resistance;
    fn MOhm(self) -> Resistance; // megaohms

    // Resistance - clear aliases
    fn ohms(self) -> Resistance;
    fn kilohms(self) -> Resistance;
    fn megaohms(self) -> Resistance;

    // Power - concise
    fn W(self) -> Power;
    fn mW(self) -> Power;
    fn kW(self) -> Power;
    fn MW(self) -> Power; // megawatts
    fn hp(self) -> Power; // horsepower

    // Power - clear aliases
    fn watts(self) -> Power;
    fn milliwatts(self) -> Power;
    fn kilowatts(self) -> Power;
    fn megawatts(self) -> Power;
    fn horsepower(self) -> Power;

    // Energy - concise
    fn J(self) -> Energy;
    fn kJ(self) -> Energy;
    fn MJ(self) -> Energy;
    fn cal(self) -> Energy;
    fn kcal(self) -> Energy;
    fn kWh(self) -> Energy; // kilowatt-hours

    // Energy - clear aliases
    fn joules(self) -> Energy;
    fn kilojoules(self) -> Energy;
    fn megajoules(self) -> Energy;
    fn calories(self) -> Energy;
    fn kilocalories(self) -> Energy;
    fn kilowatt_hours(self) -> Energy;

    // Force - concise
    fn N(self) -> Force;
    fn kN(self) -> Force;
    fn MN(self) -> Force;
    fn lbf(self) -> Force; // pound-force

    // Force - clear aliases
    fn newtons(self) -> Force;
    fn kilonewtons(self) -> Force;
    fn meganewtons(self) -> Force;
    fn pound_force(self) -> Force;

    // Pressure - concise
    fn Pa(self) -> Pressure;
    fn kPa(self) -> Pressure;
    fn MPa(self) -> Pressure;
    fn GPa(self) -> Pressure;
    fn bar(self) -> Pressure;
    fn psi(self) -> Pressure;

    // Pressure - clear aliases
    fn pascals(self) -> Pressure;
    fn kilopascals(self) -> Pressure;
    fn megapascals(self) -> Pressure;
    fn gigapascals(self) -> Pressure;
    fn bars(self) -> Pressure;
    fn pounds_per_square_inch(self) -> Pressure;

    // Frequency - concise
    fn Hz(self) -> Frequency;
    fn kHz(self) -> Frequency;
    fn MHz(self) -> Frequency;
    fn GHz(self) -> Frequency;

    // Frequency - clear aliases
    fn hertz(self) -> Frequency;
    fn kilohertz(self) -> Frequency;
    fn megahertz(self) -> Frequency;
    fn gigahertz(self) -> Frequency;

    // Capacitance - concise
    fn F(self) -> Capacitance;
    fn mF(self) -> Capacitance;
    fn uF(self) -> Capacitance; // microfarads
    fn nF(self) -> Capacitance;
    fn pF(self) -> Capacitance;

    // Capacitance - clear aliases
    fn farads(self) -> Capacitance;
    fn millifarads(self) -> Capacitance;
    fn microfarads(self) -> Capacitance;
    fn nanofarads(self) -> Capacitance;
    fn picofarads(self) -> Capacitance;

    // Velocity - clear names only (compound units)
    fn meters_per_second(self) -> Velocity;
    fn kilometers_per_hour(self) -> Velocity;
    fn miles_per_hour(self) -> Velocity;

    // Acceleration - clear names only (compound units)
    fn meters_per_second_squared(self) -> Acceleration;

    // Temperature - concise
    fn K(self) -> Temperature;
    fn degC(self) -> Temperature; // degrees Celsius
    fn degF(self) -> Temperature; // degrees Fahrenheit

    // Temperature - clear aliases
    fn kelvin(self) -> Temperature;
    fn celsius(self) -> Temperature;
    fn fahrenheit(self) -> Temperature;
}

// Implement for f64
impl QuantityExtensions for f64 {
    // Length implementations
    fn m(self) -> Length {
        Length::meters(self)
    }
    fn km(self) -> Length {
        Length::kilometers(self)
    }
    fn cm(self) -> Length {
        Length::centimeters(self)
    }
    fn mm(self) -> Length {
        Length::millimeters(self)
    }
    fn um(self) -> Length {
        Length::meters(self * 1e-6)
    }
    fn nm(self) -> Length {
        Length::meters(self * 1e-9)
    }
    fn ft(self) -> Length {
        Length::feet(self)
    }
    fn inch(self) -> Length {
        Length::inches(self)
    }
    fn mi(self) -> Length {
        Length::miles(self)
    }

    fn meters(self) -> Length {
        self.m()
    }
    fn kilometers(self) -> Length {
        self.km()
    }
    fn centimeters(self) -> Length {
        self.cm()
    }
    fn millimeters(self) -> Length {
        self.mm()
    }
    fn micrometers(self) -> Length {
        self.um()
    }
    fn nanometers(self) -> Length {
        self.nm()
    }
    fn feet(self) -> Length {
        self.ft()
    }
    fn inches(self) -> Length {
        self.inch()
    }
    fn miles(self) -> Length {
        self.mi()
    }

    // Mass implementations
    fn kg(self) -> Mass {
        Mass::kilograms(self)
    }
    fn g(self) -> Mass {
        Mass::grams(self)
    }
    fn mg(self) -> Mass {
        Mass::milligrams(self)
    }
    fn ug(self) -> Mass {
        Mass::kilograms(self * 1e-9)
    }
    fn lb(self) -> Mass {
        Mass::pounds(self)
    }
    fn oz(self) -> Mass {
        Mass::kilograms(self * 0.028349523125)
    }

    fn kilograms(self) -> Mass {
        self.kg()
    }
    fn grams(self) -> Mass {
        self.g()
    }
    fn milligrams(self) -> Mass {
        self.mg()
    }
    fn micrograms(self) -> Mass {
        self.ug()
    }
    fn pounds(self) -> Mass {
        self.lb()
    }
    fn ounces(self) -> Mass {
        self.oz()
    }

    // Time implementations
    fn s(self) -> Time {
        Time::seconds(self)
    }
    fn ms(self) -> Time {
        Time::milliseconds(self)
    }
    fn us(self) -> Time {
        Time::microseconds(self)
    }
    fn ns(self) -> Time {
        Time::seconds(self * 1e-9)
    }
    fn min_time(self) -> Time {
        Time::minutes(self)
    }
    fn h(self) -> Time {
        Time::hours(self)
    }
    fn hr(self) -> Time {
        self.h()
    }

    fn seconds(self) -> Time {
        self.s()
    }
    fn milliseconds(self) -> Time {
        self.ms()
    }
    fn microseconds(self) -> Time {
        self.us()
    }
    fn nanoseconds(self) -> Time {
        self.ns()
    }
    fn minutes(self) -> Time {
        self.min_time()
    }
    fn hours(self) -> Time {
        self.h()
    }

    // Electric Current implementations
    fn A(self) -> ElectricCurrent {
        ElectricCurrent::from_f64(self)
    }
    fn mA(self) -> ElectricCurrent {
        ElectricCurrent::from_f64(self * 0.001)
    }
    fn uA(self) -> ElectricCurrent {
        ElectricCurrent::from_f64(self * 1e-6)
    }

    fn amperes(self) -> ElectricCurrent {
        self.A()
    }
    fn milliamperes(self) -> ElectricCurrent {
        self.mA()
    }
    fn microamperes(self) -> ElectricCurrent {
        self.uA()
    }

    // Voltage implementations
    fn V(self) -> Voltage {
        Voltage::from_f64(self)
    }
    fn mV(self) -> Voltage {
        Voltage::from_f64(self * 0.001)
    }
    fn uV(self) -> Voltage {
        Voltage::from_f64(self * 1e-6)
    }
    fn kV(self) -> Voltage {
        Voltage::from_f64(self * 1000.0)
    }
    fn MV(self) -> Voltage {
        Voltage::from_f64(self * 1e6)
    }

    fn volts(self) -> Voltage {
        self.V()
    }
    fn millivolts(self) -> Voltage {
        self.mV()
    }
    fn microvolts(self) -> Voltage {
        self.uV()
    }
    fn kilovolts(self) -> Voltage {
        self.kV()
    }
    fn megavolts(self) -> Voltage {
        self.MV()
    }

    // Resistance implementations
    fn Ohm(self) -> Resistance {
        Resistance::from_f64(self)
    }
    fn kOhm(self) -> Resistance {
        Resistance::from_f64(self * 1000.0)
    }
    fn MOhm(self) -> Resistance {
        Resistance::from_f64(self * 1e6)
    }

    fn ohms(self) -> Resistance {
        self.Ohm()
    }
    fn kilohms(self) -> Resistance {
        self.kOhm()
    }
    fn megaohms(self) -> Resistance {
        self.MOhm()
    }

    // Power implementations
    fn W(self) -> Power {
        Power::watts(self)
    }
    fn mW(self) -> Power {
        Power::watts(self * 0.001)
    }
    fn kW(self) -> Power {
        Power::kilowatts(self)
    }
    fn MW(self) -> Power {
        Power::watts(self * 1e6)
    }
    fn hp(self) -> Power {
        Power::horsepower(self)
    }

    fn watts(self) -> Power {
        self.W()
    }
    fn milliwatts(self) -> Power {
        self.mW()
    }
    fn kilowatts(self) -> Power {
        self.kW()
    }
    fn megawatts(self) -> Power {
        self.MW()
    }
    fn horsepower(self) -> Power {
        self.hp()
    }

    // Energy implementations
    fn J(self) -> Energy {
        Energy::joules(self)
    }
    fn kJ(self) -> Energy {
        Energy::kilojoules(self)
    }
    fn MJ(self) -> Energy {
        Energy::joules(self * 1e6)
    }
    fn cal(self) -> Energy {
        Energy::calories(self)
    }
    fn kcal(self) -> Energy {
        Energy::kilocalories(self)
    }
    fn kWh(self) -> Energy {
        Energy::joules(self * 3.6e6)
    }

    fn joules(self) -> Energy {
        self.J()
    }
    fn kilojoules(self) -> Energy {
        self.kJ()
    }
    fn megajoules(self) -> Energy {
        self.MJ()
    }
    fn calories(self) -> Energy {
        self.cal()
    }
    fn kilocalories(self) -> Energy {
        self.kcal()
    }
    fn kilowatt_hours(self) -> Energy {
        self.kWh()
    }

    // Force implementations
    fn N(self) -> Force {
        Force::newtons(self)
    }
    fn kN(self) -> Force {
        Force::kilonewtons(self)
    }
    fn MN(self) -> Force {
        Force::newtons(self * 1e6)
    }
    fn lbf(self) -> Force {
        Force::newtons(self * 4.448222)
    }

    fn newtons(self) -> Force {
        self.N()
    }
    fn kilonewtons(self) -> Force {
        self.kN()
    }
    fn meganewtons(self) -> Force {
        self.MN()
    }
    fn pound_force(self) -> Force {
        self.lbf()
    }

    // Pressure implementations
    fn Pa(self) -> Pressure {
        Pressure::from_f64(self)
    }
    fn kPa(self) -> Pressure {
        Pressure::from_f64(self * 1000.0)
    }
    fn MPa(self) -> Pressure {
        Pressure::from_f64(self * 1e6)
    }
    fn GPa(self) -> Pressure {
        Pressure::from_f64(self * 1e9)
    }
    fn bar(self) -> Pressure {
        Pressure::from_f64(self * 1e5)
    }
    fn psi(self) -> Pressure {
        Pressure::from_f64(self * 6894.76)
    }

    fn pascals(self) -> Pressure {
        self.Pa()
    }
    fn kilopascals(self) -> Pressure {
        self.kPa()
    }
    fn megapascals(self) -> Pressure {
        self.MPa()
    }
    fn gigapascals(self) -> Pressure {
        self.GPa()
    }
    fn bars(self) -> Pressure {
        self.bar()
    }
    fn pounds_per_square_inch(self) -> Pressure {
        self.psi()
    }

    // Frequency implementations
    fn Hz(self) -> Frequency {
        Frequency::from_f64(self)
    }
    fn kHz(self) -> Frequency {
        Frequency::from_f64(self * 1000.0)
    }
    fn MHz(self) -> Frequency {
        Frequency::from_f64(self * 1e6)
    }
    fn GHz(self) -> Frequency {
        Frequency::from_f64(self * 1e9)
    }

    fn hertz(self) -> Frequency {
        self.Hz()
    }
    fn kilohertz(self) -> Frequency {
        self.kHz()
    }
    fn megahertz(self) -> Frequency {
        self.MHz()
    }
    fn gigahertz(self) -> Frequency {
        self.GHz()
    }

    // Capacitance implementations
    fn F(self) -> Capacitance {
        Capacitance::from_f64(self)
    }
    fn mF(self) -> Capacitance {
        Capacitance::from_f64(self * 0.001)
    }
    fn uF(self) -> Capacitance {
        Capacitance::from_f64(self * 1e-6)
    }
    fn nF(self) -> Capacitance {
        Capacitance::from_f64(self * 1e-9)
    }
    fn pF(self) -> Capacitance {
        Capacitance::from_f64(self * 1e-12)
    }

    fn farads(self) -> Capacitance {
        self.F()
    }
    fn millifarads(self) -> Capacitance {
        self.mF()
    }
    fn microfarads(self) -> Capacitance {
        self.uF()
    }
    fn nanofarads(self) -> Capacitance {
        self.nF()
    }
    fn picofarads(self) -> Capacitance {
        self.pF()
    }

    // Velocity implementations
    fn meters_per_second(self) -> Velocity {
        Velocity::meters_per_second(self)
    }
    fn kilometers_per_hour(self) -> Velocity {
        Velocity::kilometers_per_hour(self)
    }
    fn miles_per_hour(self) -> Velocity {
        Velocity::miles_per_hour(self)
    }

    // Acceleration implementations
    fn meters_per_second_squared(self) -> Acceleration {
        Acceleration::meters_per_second_squared(self)
    }

    // Temperature implementations
    fn K(self) -> Temperature {
        Temperature::from_f64(self)
    }
    fn degC(self) -> Temperature {
        Temperature::from_f64(self + 273.15)
    }
    fn degF(self) -> Temperature {
        Temperature::from_f64((self - 32.0) * 5.0 / 9.0 + 273.15)
    }

    fn kelvin(self) -> Temperature {
        self.K()
    }
    fn celsius(self) -> Temperature {
        self.degC()
    }
    fn fahrenheit(self) -> Temperature {
        self.degF()
    }
}

/// Extension trait for creating F32Storage quantities from f32 values.
pub trait QuantityExtensionsF32 {
    // Length - concise
    fn m(self) -> LengthF32;
    fn km(self) -> LengthF32;
    fn cm(self) -> LengthF32;
    fn mm(self) -> LengthF32;
    fn um(self) -> LengthF32; // micrometers
    fn nm(self) -> LengthF32; // nanometers
    fn ft(self) -> LengthF32;
    fn inch(self) -> LengthF32;
    fn mi(self) -> LengthF32; // miles

    // Length - clear aliases
    fn meters(self) -> LengthF32;
    fn kilometers(self) -> LengthF32;
    fn centimeters(self) -> LengthF32;
    fn millimeters(self) -> LengthF32;
    fn micrometers(self) -> LengthF32;
    fn nanometers(self) -> LengthF32;
    fn feet(self) -> LengthF32;
    fn inches(self) -> LengthF32;
    fn miles(self) -> LengthF32;

    // Mass - concise
    fn kg(self) -> MassF32;
    fn g(self) -> MassF32;
    fn mg(self) -> MassF32;
    fn ug(self) -> MassF32; // micrograms
    fn lb(self) -> MassF32;
    fn oz(self) -> MassF32;

    // Mass - clear aliases
    fn kilograms(self) -> MassF32;
    fn grams(self) -> MassF32;
    fn milligrams(self) -> MassF32;
    fn micrograms(self) -> MassF32;
    fn pounds(self) -> MassF32;
    fn ounces(self) -> MassF32;

    // Time - concise
    fn s(self) -> TimeF32;
    fn ms(self) -> TimeF32;
    fn us(self) -> TimeF32; // microseconds
    fn ns(self) -> TimeF32;
    fn min_time(self) -> TimeF32; // renamed to avoid conflict with f32::min
    fn h(self) -> TimeF32;
    fn hr(self) -> TimeF32; // alternative to h

    // Time - clear aliases
    fn seconds(self) -> TimeF32;
    fn milliseconds(self) -> TimeF32;
    fn microseconds(self) -> TimeF32;
    fn nanoseconds(self) -> TimeF32;
    fn minutes(self) -> TimeF32;
    fn hours(self) -> TimeF32;

    // Electric Current - concise
    fn A(self) -> ElectricCurrentF32;
    fn mA(self) -> ElectricCurrentF32;
    fn uA(self) -> ElectricCurrentF32; // microamperes

    // Electric Current - clear aliases
    fn amperes(self) -> ElectricCurrentF32;
    fn milliamperes(self) -> ElectricCurrentF32;
    fn microamperes(self) -> ElectricCurrentF32;

    // Voltage - concise
    fn V(self) -> VoltageF32;
    fn mV(self) -> VoltageF32;
    fn uV(self) -> VoltageF32; // microvolts
    fn kV(self) -> VoltageF32;
    fn MV(self) -> VoltageF32; // megavolts

    // Voltage - clear aliases
    fn volts(self) -> VoltageF32;
    fn millivolts(self) -> VoltageF32;
    fn microvolts(self) -> VoltageF32;
    fn kilovolts(self) -> VoltageF32;
    fn megavolts(self) -> VoltageF32;

    // Resistance - concise
    fn Ohm(self) -> ResistanceF32;
    fn kOhm(self) -> ResistanceF32;
    fn MOhm(self) -> ResistanceF32; // megaohms

    // Resistance - clear aliases
    fn ohms(self) -> ResistanceF32;
    fn kilohms(self) -> ResistanceF32;
    fn megaohms(self) -> ResistanceF32;

    // Power - concise
    fn W(self) -> PowerF32;
    fn mW(self) -> PowerF32;
    fn kW(self) -> PowerF32;
    fn MW(self) -> PowerF32; // megawatts
    fn hp(self) -> PowerF32; // horsepower

    // Power - clear aliases
    fn watts(self) -> PowerF32;
    fn milliwatts(self) -> PowerF32;
    fn kilowatts(self) -> PowerF32;
    fn megawatts(self) -> PowerF32;
    fn horsepower(self) -> PowerF32;

    // Energy - concise
    fn J(self) -> EnergyF32;
    fn kJ(self) -> EnergyF32;
    fn MJ(self) -> EnergyF32;
    fn cal(self) -> EnergyF32;
    fn kcal(self) -> EnergyF32;
    fn kWh(self) -> EnergyF32; // kilowatt-hours

    // Energy - clear aliases
    fn joules(self) -> EnergyF32;
    fn kilojoules(self) -> EnergyF32;
    fn megajoules(self) -> EnergyF32;
    fn calories(self) -> EnergyF32;
    fn kilocalories(self) -> EnergyF32;
    fn kilowatt_hours(self) -> EnergyF32;

    // Force - concise
    fn N(self) -> ForceF32;
    fn kN(self) -> ForceF32;
    fn MN(self) -> ForceF32;
    fn lbf(self) -> ForceF32; // pound-force

    // Force - clear aliases
    fn newtons(self) -> ForceF32;
    fn kilonewtons(self) -> ForceF32;
    fn meganewtons(self) -> ForceF32;
    fn pound_force(self) -> ForceF32;

    // Pressure - concise
    fn Pa(self) -> PressureF32;
    fn kPa(self) -> PressureF32;
    fn MPa(self) -> PressureF32;
    fn GPa(self) -> PressureF32;
    fn bar(self) -> PressureF32;
    fn psi(self) -> PressureF32;

    // Pressure - clear aliases
    fn pascals(self) -> PressureF32;
    fn kilopascals(self) -> PressureF32;
    fn megapascals(self) -> PressureF32;
    fn gigapascals(self) -> PressureF32;
    fn bars(self) -> PressureF32;
    fn pounds_per_square_inch(self) -> PressureF32;

    // Frequency - concise
    fn Hz(self) -> FrequencyF32;
    fn kHz(self) -> FrequencyF32;
    fn MHz(self) -> FrequencyF32;
    fn GHz(self) -> FrequencyF32;

    // Frequency - clear aliases
    fn hertz(self) -> FrequencyF32;
    fn kilohertz(self) -> FrequencyF32;
    fn megahertz(self) -> FrequencyF32;
    fn gigahertz(self) -> FrequencyF32;

    // Capacitance - concise
    fn F(self) -> CapacitanceF32;
    fn mF(self) -> CapacitanceF32;
    fn uF(self) -> CapacitanceF32; // microfarads
    fn nF(self) -> CapacitanceF32;
    fn pF(self) -> CapacitanceF32;

    // Capacitance - clear aliases
    fn farads(self) -> CapacitanceF32;
    fn millifarads(self) -> CapacitanceF32;
    fn microfarads(self) -> CapacitanceF32;
    fn nanofarads(self) -> CapacitanceF32;
    fn picofarads(self) -> CapacitanceF32;

    // Velocity - clear names only (compound units)
    fn meters_per_second(self) -> VelocityF32;
    fn kilometers_per_hour(self) -> VelocityF32;
    fn miles_per_hour(self) -> VelocityF32;

    // Acceleration - clear names only (compound units)
    fn meters_per_second_squared(self) -> AccelerationF32;

    // Temperature - concise
    fn K(self) -> TemperatureF32;
    fn degC(self) -> TemperatureF32; // degrees Celsius
    fn degF(self) -> TemperatureF32; // degrees Fahrenheit

    // Temperature - clear aliases
    fn kelvin(self) -> TemperatureF32;
    fn celsius(self) -> TemperatureF32;
    fn fahrenheit(self) -> TemperatureF32;
}

impl QuantityExtensionsF32 for f32 {
    // Length implementations
    fn m(self) -> LengthF32 {
        LengthF32::from_f32(self)
    }
    fn km(self) -> LengthF32 {
        LengthF32::from_f32(self * 1000.0)
    }
    fn cm(self) -> LengthF32 {
        LengthF32::from_f32(self * 0.01)
    }
    fn mm(self) -> LengthF32 {
        LengthF32::from_f32(self * 0.001)
    }
    fn um(self) -> LengthF32 {
        LengthF32::from_f32(self * 1e-6)
    }
    fn nm(self) -> LengthF32 {
        LengthF32::from_f32(self * 1e-9)
    }
    fn ft(self) -> LengthF32 {
        LengthF32::from_f32(self * 0.3048)
    }
    fn inch(self) -> LengthF32 {
        LengthF32::from_f32(self * 0.0254)
    }
    fn mi(self) -> LengthF32 {
        LengthF32::from_f32(self * 1609.344)
    }

    fn meters(self) -> LengthF32 {
        self.m()
    }
    fn kilometers(self) -> LengthF32 {
        self.km()
    }
    fn centimeters(self) -> LengthF32 {
        self.cm()
    }
    fn millimeters(self) -> LengthF32 {
        self.mm()
    }
    fn micrometers(self) -> LengthF32 {
        self.um()
    }
    fn nanometers(self) -> LengthF32 {
        self.nm()
    }
    fn feet(self) -> LengthF32 {
        self.ft()
    }
    fn inches(self) -> LengthF32 {
        self.inch()
    }
    fn miles(self) -> LengthF32 {
        self.mi()
    }

    // Mass implementations
    fn kg(self) -> MassF32 {
        MassF32::from_f32(self)
    }
    fn g(self) -> MassF32 {
        MassF32::from_f32(self * 0.001)
    }
    fn mg(self) -> MassF32 {
        MassF32::from_f32(self * 1e-6)
    }
    fn ug(self) -> MassF32 {
        MassF32::from_f32(self * 1e-9)
    }
    fn lb(self) -> MassF32 {
        MassF32::from_f32(self * 0.45359237)
    }
    fn oz(self) -> MassF32 {
        MassF32::from_f32(self * 0.028349523)
    }

    fn kilograms(self) -> MassF32 {
        self.kg()
    }
    fn grams(self) -> MassF32 {
        self.g()
    }
    fn milligrams(self) -> MassF32 {
        self.mg()
    }
    fn micrograms(self) -> MassF32 {
        self.ug()
    }
    fn pounds(self) -> MassF32 {
        self.lb()
    }
    fn ounces(self) -> MassF32 {
        self.oz()
    }

    // Time implementations
    fn s(self) -> TimeF32 {
        TimeF32::from_f32(self)
    }
    fn ms(self) -> TimeF32 {
        TimeF32::from_f32(self * 0.001)
    }
    fn us(self) -> TimeF32 {
        TimeF32::from_f32(self * 1e-6)
    }
    fn ns(self) -> TimeF32 {
        TimeF32::from_f32(self * 1e-9)
    }
    fn min_time(self) -> TimeF32 {
        TimeF32::from_f32(self * 60.0)
    }
    fn h(self) -> TimeF32 {
        TimeF32::from_f32(self * 3600.0)
    }
    fn hr(self) -> TimeF32 {
        self.h()
    }

    fn seconds(self) -> TimeF32 {
        self.s()
    }
    fn milliseconds(self) -> TimeF32 {
        self.ms()
    }
    fn microseconds(self) -> TimeF32 {
        self.us()
    }
    fn nanoseconds(self) -> TimeF32 {
        self.ns()
    }
    fn minutes(self) -> TimeF32 {
        self.min_time()
    }
    fn hours(self) -> TimeF32 {
        self.h()
    }

    // Electric Current implementations
    fn A(self) -> ElectricCurrentF32 {
        ElectricCurrentF32::from_f32(self)
    }
    fn mA(self) -> ElectricCurrentF32 {
        ElectricCurrentF32::from_f32(self * 0.001)
    }
    fn uA(self) -> ElectricCurrentF32 {
        ElectricCurrentF32::from_f32(self * 1e-6)
    }

    fn amperes(self) -> ElectricCurrentF32 {
        self.A()
    }
    fn milliamperes(self) -> ElectricCurrentF32 {
        self.mA()
    }
    fn microamperes(self) -> ElectricCurrentF32 {
        self.uA()
    }

    // Voltage implementations
    fn V(self) -> VoltageF32 {
        VoltageF32::from_f32(self)
    }
    fn mV(self) -> VoltageF32 {
        VoltageF32::from_f32(self * 0.001)
    }
    fn uV(self) -> VoltageF32 {
        VoltageF32::from_f32(self * 1e-6)
    }
    fn kV(self) -> VoltageF32 {
        VoltageF32::from_f32(self * 1000.0)
    }
    fn MV(self) -> VoltageF32 {
        VoltageF32::from_f32(self * 1e6)
    }

    fn volts(self) -> VoltageF32 {
        self.V()
    }
    fn millivolts(self) -> VoltageF32 {
        self.mV()
    }
    fn microvolts(self) -> VoltageF32 {
        self.uV()
    }
    fn kilovolts(self) -> VoltageF32 {
        self.kV()
    }
    fn megavolts(self) -> VoltageF32 {
        self.MV()
    }

    // Resistance implementations
    fn Ohm(self) -> ResistanceF32 {
        ResistanceF32::from_f32(self)
    }
    fn kOhm(self) -> ResistanceF32 {
        ResistanceF32::from_f32(self * 1000.0)
    }
    fn MOhm(self) -> ResistanceF32 {
        ResistanceF32::from_f32(self * 1e6)
    }

    fn ohms(self) -> ResistanceF32 {
        self.Ohm()
    }
    fn kilohms(self) -> ResistanceF32 {
        self.kOhm()
    }
    fn megaohms(self) -> ResistanceF32 {
        self.MOhm()
    }

    // Power implementations
    fn W(self) -> PowerF32 {
        PowerF32::from_f32(self)
    }
    fn mW(self) -> PowerF32 {
        PowerF32::from_f32(self * 0.001)
    }
    fn kW(self) -> PowerF32 {
        PowerF32::from_f32(self * 1000.0)
    }
    fn MW(self) -> PowerF32 {
        PowerF32::from_f32(self * 1e6)
    }
    fn hp(self) -> PowerF32 {
        PowerF32::from_f32(self * 745.7)
    }

    fn watts(self) -> PowerF32 {
        self.W()
    }
    fn milliwatts(self) -> PowerF32 {
        self.mW()
    }
    fn kilowatts(self) -> PowerF32 {
        self.kW()
    }
    fn megawatts(self) -> PowerF32 {
        self.MW()
    }
    fn horsepower(self) -> PowerF32 {
        self.hp()
    }

    // Energy implementations
    fn J(self) -> EnergyF32 {
        EnergyF32::from_f32(self)
    }
    fn kJ(self) -> EnergyF32 {
        EnergyF32::from_f32(self * 1000.0)
    }
    fn MJ(self) -> EnergyF32 {
        EnergyF32::from_f32(self * 1e6)
    }
    fn cal(self) -> EnergyF32 {
        EnergyF32::from_f32(self * 4.184)
    }
    fn kcal(self) -> EnergyF32 {
        EnergyF32::from_f32(self * 4184.0)
    }
    fn kWh(self) -> EnergyF32 {
        EnergyF32::from_f32(self * 3.6e6)
    }

    fn joules(self) -> EnergyF32 {
        self.J()
    }
    fn kilojoules(self) -> EnergyF32 {
        self.kJ()
    }
    fn megajoules(self) -> EnergyF32 {
        self.MJ()
    }
    fn calories(self) -> EnergyF32 {
        self.cal()
    }
    fn kilocalories(self) -> EnergyF32 {
        self.kcal()
    }
    fn kilowatt_hours(self) -> EnergyF32 {
        self.kWh()
    }

    // Force implementations
    fn N(self) -> ForceF32 {
        ForceF32::from_f32(self)
    }
    fn kN(self) -> ForceF32 {
        ForceF32::from_f32(self * 1000.0)
    }
    fn MN(self) -> ForceF32 {
        ForceF32::from_f32(self * 1e6)
    }
    fn lbf(self) -> ForceF32 {
        ForceF32::from_f32(self * 4.448222)
    }

    fn newtons(self) -> ForceF32 {
        self.N()
    }
    fn kilonewtons(self) -> ForceF32 {
        self.kN()
    }
    fn meganewtons(self) -> ForceF32 {
        self.MN()
    }
    fn pound_force(self) -> ForceF32 {
        self.lbf()
    }

    // Pressure implementations
    fn Pa(self) -> PressureF32 {
        PressureF32::from_f32(self)
    }
    fn kPa(self) -> PressureF32 {
        PressureF32::from_f32(self * 1000.0)
    }
    fn MPa(self) -> PressureF32 {
        PressureF32::from_f32(self * 1e6)
    }
    fn GPa(self) -> PressureF32 {
        PressureF32::from_f32(self * 1e9)
    }
    fn bar(self) -> PressureF32 {
        PressureF32::from_f32(self * 1e5)
    }
    fn psi(self) -> PressureF32 {
        PressureF32::from_f32(self * 6894.76)
    }

    fn pascals(self) -> PressureF32 {
        self.Pa()
    }
    fn kilopascals(self) -> PressureF32 {
        self.kPa()
    }
    fn megapascals(self) -> PressureF32 {
        self.MPa()
    }
    fn gigapascals(self) -> PressureF32 {
        self.GPa()
    }
    fn bars(self) -> PressureF32 {
        self.bar()
    }
    fn pounds_per_square_inch(self) -> PressureF32 {
        self.psi()
    }

    // Frequency implementations
    fn Hz(self) -> FrequencyF32 {
        FrequencyF32::from_f32(self)
    }
    fn kHz(self) -> FrequencyF32 {
        FrequencyF32::from_f32(self * 1000.0)
    }
    fn MHz(self) -> FrequencyF32 {
        FrequencyF32::from_f32(self * 1e6)
    }
    fn GHz(self) -> FrequencyF32 {
        FrequencyF32::from_f32(self * 1e9)
    }

    fn hertz(self) -> FrequencyF32 {
        self.Hz()
    }
    fn kilohertz(self) -> FrequencyF32 {
        self.kHz()
    }
    fn megahertz(self) -> FrequencyF32 {
        self.MHz()
    }
    fn gigahertz(self) -> FrequencyF32 {
        self.GHz()
    }

    // Capacitance implementations
    fn F(self) -> CapacitanceF32 {
        CapacitanceF32::from_f32(self)
    }
    fn mF(self) -> CapacitanceF32 {
        CapacitanceF32::from_f32(self * 0.001)
    }
    fn uF(self) -> CapacitanceF32 {
        CapacitanceF32::from_f32(self * 1e-6)
    }
    fn nF(self) -> CapacitanceF32 {
        CapacitanceF32::from_f32(self * 1e-9)
    }
    fn pF(self) -> CapacitanceF32 {
        CapacitanceF32::from_f32(self * 1e-12)
    }

    fn farads(self) -> CapacitanceF32 {
        self.F()
    }
    fn millifarads(self) -> CapacitanceF32 {
        self.mF()
    }
    fn microfarads(self) -> CapacitanceF32 {
        self.uF()
    }
    fn nanofarads(self) -> CapacitanceF32 {
        self.nF()
    }
    fn picofarads(self) -> CapacitanceF32 {
        self.pF()
    }

    // Velocity implementations
    fn meters_per_second(self) -> VelocityF32 {
        VelocityF32::from_f32(self)
    }
    fn kilometers_per_hour(self) -> VelocityF32 {
        VelocityF32::from_f32(self / 3.6)
    }
    fn miles_per_hour(self) -> VelocityF32 {
        VelocityF32::from_f32(self * 0.44704)
    }

    // Acceleration implementations
    fn meters_per_second_squared(self) -> AccelerationF32 {
        AccelerationF32::from_f32(self)
    }

    // Temperature implementations
    fn K(self) -> TemperatureF32 {
        TemperatureF32::from_f32(self)
    }
    fn degC(self) -> TemperatureF32 {
        TemperatureF32::from_f32(self + 273.15)
    }
    fn degF(self) -> TemperatureF32 {
        TemperatureF32::from_f32((self - 32.0) * 5.0 / 9.0 + 273.15)
    }

    fn kelvin(self) -> TemperatureF32 {
        self.K()
    }
    fn celsius(self) -> TemperatureF32 {
        self.degC()
    }
    fn fahrenheit(self) -> TemperatureF32 {
        self.degF()
    }
}
