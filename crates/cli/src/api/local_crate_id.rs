use anyhow::anyhow;
use anyhow::Result;
use bevyhub_api::prelude::*;
use semver::Version;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Extract the version and name from a path, can handle workspace crates
#[derive(Debug, Clone)]
pub struct LocalCrateId {
	pub crate_name: String,
	pub version: Version,
	pub path: PathBuf,
}

impl Into<CratesIoCrateId> for LocalCrateId {
	fn into(self) -> CratesIoCrateId {
		CratesIoCrateId {
			crate_name: self.crate_name,
			version: self.version,
		}
	}
}

impl Into<CrateId> for LocalCrateId {
	fn into(self) -> CrateId { CrateId::CratesIo(self.into()) }
}

impl LocalCrateId {
	pub fn parse(path: &str) -> Result<Self> {
		let path = PathBuf::from(path);
		let path = path.canonicalize()?;

		let cargo_toml = fs::read_to_string(path.join("Cargo.toml"))?;
		let toml = toml::from_str::<CargoManifest>(&cargo_toml)?;
		let name = toml
			.package
			.ok_or_else(|| anyhow!("missing package in Cargo.toml"))?
			.name;
		let version = get_version(&path)?;

		Ok(Self {
			crate_name: name,
			version: Version::parse(&version)?,
			path,
		})
	}
}

fn get_version(path: &Path) -> Result<String> {
	let path = path.canonicalize()?;
	let cargo_toml = fs::read_to_string(path.join("Cargo.toml"))?;
	let toml = toml::from_str::<CargoManifest>(&cargo_toml)?;

	if let Some(package) = toml.package {
		if let Some(version) = package.version {
			if let Some(version) = version.as_local() {
				return Ok(version);
			}
		}
	}
	if let Some(workspace) = toml.workspace {
		if let Some(package) = workspace.package {
			if let Some(version) = package.version {
				return Ok(version);
			}
		}
	}

	let mut anscestor = path.parent();
	// find nearest anscestor with Cargo.toml
	while let Some(path) = anscestor {
		if fs::exists(path.join("Cargo.toml"))? {
			return get_version(&path);
		}
		anscestor = path.parent();
	}
	anyhow::bail!("failed to get workspace Cargo.toml");
}


#[cfg(test)]
mod test {
	// use crate::prelude::*;
	use super::get_version;
	use anyhow::Result;
	use std::path::PathBuf;
	use std::str::FromStr;
	use sweet::*;

	#[test]
	fn works() -> Result<()> {
		expect(get_version(&PathBuf::from_str(".")?)) // cli
			.to_be_ok()?;
		expect(get_version(&PathBuf::from_str("../../")?)).to_be_ok()?; // bevyhub_api
		Ok(())
	}
}
