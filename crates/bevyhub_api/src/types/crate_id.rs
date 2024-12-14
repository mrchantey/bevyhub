use crate::prelude::*;
use mongodb::bson::to_bson;
use mongodb::bson::Bson;
use semver::Version;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

/// A unique identifier for a crate, whether on crate.io or GitHub.
/// This is the *minimal* amount of information needed to identify a crate.
/// For the contents of a Cargo Manifest see [CrateDoc].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(tag = "kind")]
pub enum CrateId {
	CratesIo(CratesIoCrateId),
	Github(GitHubCrateId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
pub struct CratesIoCrateId {
	pub crate_name: String,
	pub version: Version,
}

impl CratesIoCrateId {
	pub fn new(crate_name: impl Into<String>, version: Version) -> Self {
		Self {
			crate_name: crate_name.into(),
			version,
		}
	}
	pub fn slash_formatted(&self) -> String {
		format!("{}/{}", self.crate_name, self.version)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
pub struct GitHubCrateId {
	/// ie `mrchantey`
	owner: String,
	/// ie `bevyhub`
	repo: String,
	/// Defaults to `Cargo.toml` but can be any file in the repo
	/// ie `crates/bevyhub_api/Cargo.toml`
	manifest_dir: String,
	/// Defaults to `main`
	/// ie `main`
	branch: String,
	/// ie `390djjksd092`
	commit_hash: String,
}

impl GitHubCrateId {
	pub fn into_repo_url(&self) -> String {
		format!("https://github.com/{}/{}", self.owner, self.repo)
	}
}

impl Into<CrateId> for CratesIoCrateId {
	fn into(self) -> CrateId { CrateId::CratesIo(self) }
}

impl Into<CrateId> for GitHubCrateId {
	fn into(self) -> CrateId { CrateId::Github(self) }
}

impl CrateId {
	pub fn new_crates_io(
		crate_name: impl Into<String>,
		version: Version,
	) -> Self {
		Self::CratesIo(CratesIoCrateId {
			crate_name: crate_name.into(),
			version,
		})
	}
	pub fn into_scene_id(&self, scene_name: impl Into<String>) -> SceneId {
		SceneId::new(self.clone(), scene_name)
	}

	/// A human, fs & url friendly string unique to this crate
	pub fn path(&self) -> String {
		match self {
			Self::CratesIo(CratesIoCrateId {
				crate_name,
				version,
			}) => format!("crates_io/{crate_name}/{version}"),
			Self::Github(GitHubCrateId {
				owner,
				repo,
				branch,
				commit_hash,
				..
			}) => format!("github/{owner}/{repo}/{branch}/{commit_hash}"),
		}
	}

	/// String in format `crates.io/crate_name/version`
	pub fn into_doc_id(&self) -> DocId { DocId(self.path()) }
}
impl Into<Bson> for CrateId {
	fn into(self) -> Bson { to_bson(&self).expect("CrateId to Bson failed") }
}


impl std::fmt::Display for CrateId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.path())
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

impl CratesIoCrateId {
	pub fn bevyhub_template() -> Self {
		let version = Version::parse("0.0.1-rc.1").unwrap();
		Self::new("bevyhub_template", version)
	}
	pub fn bevyhub_template_bad_version() -> Self {
		Self::new("bevyhub_template", Version::new(0, 0, 0))
	}
}
