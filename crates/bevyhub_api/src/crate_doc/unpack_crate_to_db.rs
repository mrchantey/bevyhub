use crate::prelude::*;
use anyhow::Result;

impl Services {
	/// unpack the [crate_doc] and every [scene_doc] in the crate to the document db
	pub async fn unpack_crate_to_db(
		&self,
		crate_id: &CrateId,
	) -> Result<(CrateDoc, Vec<SceneDoc>)> {
		let manifest = self.cargo_manifest(crate_id).await?;

		let Some(package_toml) = &manifest.package else {
			anyhow::bail!("Cargo.toml missing package field");
		};

		// why were we reconstructing the crate_id here?
		// let Some(version) = &package_toml.version else {
		// 	anyhow::bail!("Cargo.toml missing package.version field");
		// };
		// let MaybeInherited::Local(version) = version else {
		// 	anyhow::bail!("Workspace manifests are not supported");
		// };

		// let version = Version::parse(version)?;
		// let crate_id = CrateId {
		// 	version,
		// 	..crate_id.clone()
		// };

		let crate_doc = CrateDoc::from_package(crate_id, package_toml.clone())?;

		let scene_docs = if let Some(metadata) = &package_toml.metadata {
			let cargo_lock = self.cargo_lock(&crate_id).await?;

			let futs = metadata
				.scene
				.iter()
				.map(|scene| {
					SceneDoc::from_manifest(
						self,
						&crate_doc,
						&cargo_lock,
						&crate_id,
						&metadata,
						scene,
					)
				})
				.collect::<Vec<_>>();

			futures::future::try_join_all(futs).await?
		} else {
			Default::default()
		};
		self.db().crates().insert(&crate_doc).await?;
		self.db().scenes().insert_many(&scene_docs).await?;
		SetLatestScenesInDb::set_latest_scenes_in_db(self, &crate_id).await?;

		Ok((crate_doc, scene_docs))
	}
}
