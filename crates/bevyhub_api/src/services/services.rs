use crate::prelude::*;
use anyhow::Result;

#[derive(Clone)]
pub struct Services {
	pub storage: ObjectStorageEnum,
	pub registry: CargoRegistryEnum,
	pub db: DocumentDbEnum,
	pub env: ApiEnvironment,
}
impl Services {
	pub fn storage(&self) -> &dyn ObjectStorage { self.storage.inner() }
	pub fn registry(&self) -> &dyn CargoRegistry { self.registry.inner() }
	pub fn db(&self) -> &dyn DocumentDb { self.db.inner() }

	pub async fn init() -> Result<Self> {
		Self::init_with_env(ApiEnvironment::default()).await
	}
	pub async fn init_with_env(env: ApiEnvironment) -> Result<Self> {
		Ok(Self {
			storage: ObjectStorageEnum::new(env).await?,
			registry: CargoRegistryEnum::new(env)?,
			db: DocumentDbEnum::new(env).await?,
			env,
		})
	}

	pub async fn cargo_manifest(
		&self,
		crate_id: &CrateId,
	) -> Result<CargoManifest> {
		match crate_id {
			CrateId::CratesIo(crates_io_crate_id) => {
				CratesIoFiles::cargo_manifest(self, crates_io_crate_id).await
			}
			CrateId::Github(github_crate_id) => {
				GithubFiles::cargo_manifest(github_crate_id).await
			}
		}
	}
	pub async fn cargo_lock(&self, crate_id: &CrateId) -> Result<CargoLock> {
		match crate_id {
			CrateId::CratesIo(crates_io_crate_id) => {
				CratesIoFiles::cargo_lock(self, crates_io_crate_id).await
			}
			CrateId::Github(github_crate_id) => {
				GithubFiles::cargo_lock(github_crate_id).await
			}
		}
	}
}
