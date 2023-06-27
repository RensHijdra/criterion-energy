use std::path::Path;

use criterion::profiler::Profiler;

struct EnergyProfiler {}

impl Profiler for EnergyProfiler {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {}

    fn stop_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {}
}

