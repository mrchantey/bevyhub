#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

pub mod events;
pub mod extensions;
pub mod networking;
pub mod replication;
#[cfg(feature = "tokio")]
pub mod tokio_client;
pub mod utils;
#[cfg(target_arch = "wasm32")]
pub mod web_transport;

pub mod prelude {
	pub use crate::events::*;
	pub use crate::extensions::*;
	pub use crate::networking::*;
	pub use crate::replication::*;
	#[cfg(feature = "tokio")]
	pub use crate::tokio_client::*;
	pub use crate::utils::*;
	#[cfg(target_arch = "wasm32")]
	pub use crate::web_transport::*;
}
