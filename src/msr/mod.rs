pub mod profiler;
pub mod measurement;
pub(crate) mod energyformatter;
mod util;

#[cfg(all(feature = "criterion3", feature = "criterion4"))]
compile_error!("feature \"criterion3\" and feature \"criterion4\" cannot be enabled at the same time");

#[cfg(all(not(feature = "criterion3"), not(feature = "criterion4")))]
compile_error!("select at least one feature from \"criterion3\" or \"criterion4\" based on the version your project uses.");

