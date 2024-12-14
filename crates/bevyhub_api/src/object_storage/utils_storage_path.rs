pub mod storage_path {
	use crate::prelude::*;
	/// The directory where crates are unpackaged to
	pub const UNPKG_DIR: &'static str = "unpkg";
	/// Create a path to an unpackaged tarball.
	pub fn unpkg_path(crate_id: &CratesIoCrateId, path: &str) -> String {
		format!("{}/{}/{}", UNPKG_DIR, crate_id.slash_formatted(), path)
	}
}
