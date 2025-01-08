use crate::prelude::*;
use mongodb::bson::to_bson;
use mongodb::bson::Bson;
use semver::Version;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

/// A unique identifier for a specific immutable crate, whether on crate.io or Github.
/// This is the *minimal* amount of information needed to identify a crate.
/// For the contents of a Cargo Manifest see [CrateDoc].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(tag = "kind")]
pub enum CrateId {
	CratesIo(CratesIoCrateId),
	Github(GithubCrateId),
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

/// A unique identifier for a crate on Github.
/// The `commit_hash` ensures that the crate is immutable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
pub struct GithubCrateId {
	/// ie `mrchantey`
	pub owner: String,
	/// ie `bevyhub`
	pub repo: String,
	/// ie `390djjksd092`
	pub commit_hash: String,
	/// Defaults to `Cargo.toml` but can be any file in the repo
	/// ie `crates/bevyhub_api/Cargo.toml`
	pub manifest_dir: String,
}

impl GithubCrateId {
	pub fn into_repo_url(&self) -> String {
		format!("https://github.com/{}/{}", self.owner, self.repo)
	}

	pub fn relative_to_manifest_dir(manifest_dir: &str, path: &str) -> String {
		let mut parts = manifest_dir.split('/').collect::<Vec<_>>();
		parts.pop();
		if parts.is_empty() {
			path.to_string()
		} else {
			format!("{}/{}", parts.join("/"), path)
		}
	}
}

impl Into<CrateId> for CratesIoCrateId {
	fn into(self) -> CrateId { CrateId::CratesIo(self) }
}

impl Into<CrateId> for GithubCrateId {
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
	pub fn new_github(
		owner: &str,
		repo: &str,
		commit_hash: &str,
		manifest_dir: Option<&str>,
	) -> Self {
		Self::Github(GithubCrateId {
			owner: owner.to_string(),
			repo: repo.to_string(),
			commit_hash: commit_hash.to_string(),
			manifest_dir: manifest_dir.unwrap_or("Cargo.toml").to_string(),
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
			Self::Github(GithubCrateId {
				owner,
				repo,
				commit_hash,
				manifest_dir,
			}) => format!("github/{owner}/{repo}/{commit_hash}/{manifest_dir}"),
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


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::Result;
	use sweet::prelude::*;

	#[test]
	fn relative_to_manifest_dir() -> Result<()> {
		expect(
			GithubCrateId::relative_to_manifest_dir(
				"crates/bevyhub_api/Cargo.toml",
				"Cargo.lock",
			)
			.as_str(),
		)
		.to_be("crates/bevyhub_api/Cargo.lock");
		expect(
			GithubCrateId::relative_to_manifest_dir(
				"Cargo.toml",
				"scenes/my-scene.json",
			)
			.as_str(),
		)
		.to_be("scenes/my-scene.json");

		Ok(())
	}
}
