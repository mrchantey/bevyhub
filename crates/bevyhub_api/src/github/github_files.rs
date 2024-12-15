use crate::prelude::*;
use anyhow::Result;

pub struct GithubFiles;


impl GithubFiles {
	pub async fn cargo_manifest(
		crate_id: &GithubCrateId,
	) -> Result<CargoManifest> {
		let bytes = GithubApi::file(
			&crate_id.owner,
			&crate_id.repo,
			&crate_id.commit_hash,
			&crate_id.manifest_dir,
		)
		.await?;
		let cargo_manifest = toml_from_bytes(&bytes)?;
		Ok(cargo_manifest)
	}

	pub async fn cargo_lock(crate_id: &GithubCrateId) -> Result<CargoLock> {
		let bytes = GithubApi::file(
			&crate_id.owner,
			&crate_id.repo,
			&crate_id.commit_hash,
			&crate_id.manifest_lock_dir(),
		)
		.await?;
		let cargo_lock = toml_from_bytes(&bytes)?;
		Ok(cargo_lock)
	}
}
