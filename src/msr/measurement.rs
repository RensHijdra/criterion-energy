
use std::ops::{Add, Mul};

use criterion::measurement::{Measurement, ValueFormatter};

use crate::msr::energyformatter::EnergyFormatter;
use crate::msr::util::{read_power_unit, read_raw_energy};

pub struct Energy;

const CPU: usize = 3;

#[cfg(feature = "coverage")]
fn capture_minicov_coverage() {
    let mut coverage = vec![];
    unsafe {
        // Note that this function is not thread-safe! Use a lock if needed.
        minicov::capture_coverage(&mut coverage).unwrap();
    }
    let string = env::var("MINICOV_PROFILE_FILE").unwrap_or("output.profraw".to_string());
    std::fs::write(string, coverage).unwrap();
}


impl Measurement for Energy {
    type Intermediate = u64;
    type Value = f64;

    #[cfg(feature = "coverage")]
    fn start(&self) -> Self::Intermediate {
        // Reset coverage so we only measure the benchmark itself
        minicov::reset_coverage();
        0
    }

    #[cfg(not(feature = "coverage"))]
    fn start(&self) -> Self::Intermediate {
        read_raw_energy(CPU)
    }

    #[cfg(feature = "coverage")]
    fn end(&self, _: Self::Intermediate) -> Self::Value {
        capture_minicov_coverage();
        0.0
    }

    #[cfg(not(feature = "coverage"))]
    fn end(&self, intermediate: Self::Intermediate) -> Self::Value {
        // If the u64 wraps (once) during the measurement, wrapping around 0 gives the correct measurement
        // Wrapping is expected to occur
        let raw_value = read_raw_energy(CPU).wrapping_sub(intermediate);

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
