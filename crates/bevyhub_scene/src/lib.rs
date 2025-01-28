#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
#![feature(trait_upcasting)]

pub mod scene_exporter;
#[cfg(feature = "export_types")]
pub mod type_registry_exporter;
pub mod utils;

pub mod prelude {
	pub use crate::scene_exporter::*;
	#[cfg(feature = "export_types")]
	pub use crate::type_registry_exporter::*;
	pub use crate::utils::*;
}
