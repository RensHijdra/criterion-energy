use std::env;
use std::ops::{Add};

use criterion::measurement::{Measurement, ValueFormatter};

use crate::msr::energyformatter::EnergyFormatter;

pub struct Energy;

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
    type Intermediate = ();
    type Value = f64;

    fn start(&self) -> Self::Intermediate {
        // Reset coverage so we only measure the benchmark itself
        minicov::reset_coverage();
    }

    fn end(&self, _: Self::Intermediate) -> Self::Value {
        capture_minicov_coverage();
        0.0
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
