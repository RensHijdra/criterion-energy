use std::ops::Add;
use criterion::measurement::{Measurement, ValueFormatter};
use crate::msr::energyformatter::EnergyFormatter;
use crate::msr::profiler::read_single_core_msr_file;

pub struct Energy;

impl Measurement for Energy {
    type Intermediate = u64;
    type Value = u64;

    fn start(&self) -> Self::Intermediate {
        read_single_core_msr_file(0).unwrap()
    }

    fn end(&self, intermediate: Self::Intermediate) -> Self::Value {
        read_single_core_msr_file(0).unwrap() - intermediate
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        v1.add(v2)
    }

    fn zero(&self) -> Self::Value {
        0u64
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        *value as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &EnergyFormatter
    }
}