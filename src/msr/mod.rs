pub mod profiler;
pub(crate) mod energyformatter;

// If doing coverage, replace measurement.rs with coverage.rs;
// which is an implementation of the same struct, but calling minicov
#[cfg_attr(feature = "coverage", path = "coverage.rs")]
pub mod measurement;

// We need utils if not doing coverage
#[cfg(not(feature = "coverage"))]
mod util;

// Criterion versions 3 and 4 are mutually exclusive
#[cfg(all(feature = "criterion3", feature = "criterion4"))]
compile_error!("feature \"criterion3\" and feature \"criterion4\" cannot be enabled at the same time");

// But we do need either one of criterion 3 or 4
#[cfg(all(not(feature = "criterion3"), not(feature = "criterion4")))]
compile_error!("select at least one feature from \"criterion3\" or \"criterion4\" based on the version your project uses.");

