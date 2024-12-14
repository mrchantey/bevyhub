use crate::prelude::*;
use mongodb::bson::to_bson;
use mongodb::bson::Bson;
use semver::Version;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

/// A unique identifier for a crate, whether on crate.io or GitHub
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
pub struct CrateId {
	pub crate_name: String,
	// ts-rs represents versions as strings
	pub version: Version,
	pub source: CrateIdSource,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
pub enum CrateIdSource {
	CratesIo,
	GitHub {
		owner: String,
		repo: String,
		commit_hash: String,
	},
}

impl CrateId {
	pub fn new_crates_io(name: impl Into<String>, version: Version) -> Self {
		Self {
			crate_name: name.into(),
			version,
			source: CrateIdSource::CratesIo,
		}
	}
	pub fn into_scene_id(&self, scene_name: impl Into<String>) -> SceneId {
		SceneId::new(self.clone(), scene_name)
	}

	/// String in format `crate_name/version`
	pub fn path(&self) -> String {
		format!("crates_io/{}/{}", self.crate_name, self.version)
	}

	/// String in format `crates.io/crate_name/version`
	pub fn into_doc_id(&self) -> DocId {
		DocId(format!("crates.io/{}/{}", self.crate_name, self.version))
	}
}
impl Into<Bson> for CrateId {
	fn into(self) -> Bson { to_bson(&self).expect("CrateId to Bson failed") }
}


impl std::fmt::Display for CrateId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}/{}", self.crate_name, self.version)
	}
}

// #[cfg(test)]
impl CrateId {
	pub fn bevyhub_template() -> Self {
		let version = Version::parse("0.0.1-rc.1").unwrap();
		Self::new_crates_io("bevyhub_template", version)
	}
	pub fn bevyhub_template_bad_version() -> Self {
		Self::new_crates_io("bevyhub_template", Version::new(0, 0, 0))
	}
}
