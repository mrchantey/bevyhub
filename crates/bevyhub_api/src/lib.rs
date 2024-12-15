#![feature(async_fn_traits, const_trait_impl, async_closure)]
#![allow(async_fn_in_trait)]
pub mod cargo_registry;
pub mod crate_doc;
pub mod crates_io;
pub mod document_db;
pub mod github;
pub mod object_storage;
pub mod scene_doc;
pub mod server;
pub mod services;
pub mod types;


pub mod prelude {
	pub use crate::cargo_registry::*;
	pub use crate::crate_doc::*;
	pub use crate::crates_io::*;
	pub use crate::document_db::*;
	pub use crate::github::*;
	pub use crate::object_storage::*;
	pub use crate::scene_doc::*;
	pub use crate::server::layers::*;
	pub use crate::server::*;
	pub use crate::services::*;
	pub use crate::types::*;
}
