use super::cargo_registry::CargoRegistry;
use super::crates_io_api::CratesIoApi;
use crate::prelude::*;
use anyhow::Result;
use axum::body::Bytes;
use std::path::PathBuf;
use tokio::fs;

/// First attempts to load from fs before hitting crates.io
#[derive(Default, Clone)]
pub struct LocalCacheRegistry {
	crates_io: CratesIoApi,
	/// Only read from local, but dont write, useful for staging
	read_only: bool,
}

impl LocalCacheRegistry {
	// was used by staging, for staging tests we shouldnt write to the cache?
	pub fn read_only() -> Self {
		Self {
			crates_io: CratesIoApi::default(),
			read_only: true,
		}
	}
	pub fn tarball_path(
		prefix: &str,
		crate_name: &str,
		version: &str,
	) -> PathBuf {
		format!("{}/{}-{}.crate", prefix, crate_name, version).into()
	}
}

#[async_trait::async_trait]
impl CargoRegistry for LocalCacheRegistry {
	async fn crate_index(&self, crate_name: &str) -> Result<CrateIndex> {
		self.crates_io.crate_index(crate_name).await
	}

	async fn tarball(&self, crate_id: &CratesIoCrateId) -> Result<Bytes> {
		let dir = "target/tarball-cache";
		let path = Self::tarball_path(
			dir,
			&crate_id.crate_name,
			&crate_id.version.to_string(),
		);
		if let Ok(bytes) = fs::read(&path).await {
			return Ok(bytes.into());
		}
		// println!(
		// 	"Local cache - downloading from registry: {}",
		// 	path.display()
		// );
		let buff = self.crates_io.tarball(crate_id).await?;

		if !self.read_only {
			fs::create_dir_all(dir).await?;
			fs::write(&path, &buff).await?;
		}
		Ok(buff)
	}
}



#[cfg(test)]
mod test {
	use crate::prelude::*;
	use semver::Version;
	use sweet::prelude::*;

	#[tokio::test]
	async fn versions() {
		let registry = LocalCacheRegistry::default();
		let versions = registry.versions("bevyhub_template").await.unwrap();
		expect(versions[0].to_string()).to_be("0.0.1-rc.1".to_string());
	}
	#[tokio::test]
	async fn crate_index() {
		let registry = LocalCacheRegistry::default();
		let index = registry.crate_index("bevyhub_template").await.unwrap();
		expect(index.len()).to_be_greater_or_equal_to(1);
	}

	#[tokio::test]
	async fn tarball() {
		let registry = LocalCacheRegistry::default();
		let tarball = registry
			.tarball(&CratesIoCrateId::new(
				"bevyhub_template",
				Version::parse("0.0.1-rc.1").unwrap(),
			))
			.await
			.unwrap();
		expect(tarball.len()).to_be_greater_than(0);
	}
}
