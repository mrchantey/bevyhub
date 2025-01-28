#![cfg(not(target_arch = "wasm32"))]
pub mod server;

pub mod prelude {
	pub use crate::server::*;
}
