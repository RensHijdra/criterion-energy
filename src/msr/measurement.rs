use std::ops::{Add, Mul};

use criterion::measurement::{Measurement, ValueFormatter};

use crate::msr::energyformatter::EnergyFormatter;
use crate::msr::util::{read_power_unit, read_raw_energy};


// TODO make this variable e.g. get_env
const CPU: usize = 3;

pub struct Energy;

impl Measurement for Energy {
    type Intermediate = u64;
    type Value = f64;

    fn start(&self) -> Self::Intermediate {
        read_raw_energy(CPU)
    }

    fn end(&self, intermediate: Self::Intermediate) -> Self::Value {
        // Data is contained in a u64 register, but actually wraps at u32::MAX.
        // If the u32 wraps (once) during the measurement, wrapping around 0 gives the correct measurement
        // Wrapping is expected to occur every so often
        let raw_value = (read_raw_energy(CPU) as u32).wrapping_sub(intermediate as u32);

        let unit = read_power_unit(CPU); // joules per unit raw value
        (raw_value as f64).mul(unit)  // joules
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        v1.add(v2)
    }

    fn zero(&self) -> Self::Value {
        0f64
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        value.clone()
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &EnergyFormatter
    }
}
