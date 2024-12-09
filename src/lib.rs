#![doc = include_str!("../README.md")]
pub use bevyhub_core as core;
#[cfg(feature = "net")]
pub use bevyhub_net as net;
#[cfg(feature = "scene")]
pub use bevyhub_scene as scene;

pub mod prelude {
	pub use crate::core::prelude::*;
	#[cfg(feature = "net")]
	pub use crate::net::prelude::*;
	#[cfg(feature = "scene")]
	pub use crate::scene::prelude::*;
}
