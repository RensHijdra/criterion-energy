use criterion::measurement::{Measurement, ValueFormatter};

use crate::msr::energyformatter::EnergyFormatter;

pub struct Energy;


impl Measurement for Energy {
    type Intermediate = ();
    type Value = f64;

    fn start(&self) -> Self::Intermediate {
        partial_callgrind::start();
    }

    fn end(&self, _: Self::Intermediate) -> Self::Value {
        partial_callgrind::stop();
        0.0
    }

    fn add(&self, _: &Self::Value, _: &Self::Value) -> Self::Value {
        0.0
    }

    fn zero(&self) -> Self::Value {
        0f64
    }

    fn to_f64(&self, _: &Self::Value) -> f64 {
        0.0
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &EnergyFormatter
    }
}
