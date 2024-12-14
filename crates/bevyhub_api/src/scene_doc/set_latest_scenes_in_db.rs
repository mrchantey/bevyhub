use crate::prelude::*;
use anyhow::Result;
use mongodb::bson::doc;
use mongodb::bson::Document;

pub struct SetLatestScenesInDb;


impl SetLatestScenesInDb {
	/// Finds all scenes that are latest and ensures `is_latest: true`.
	/// Also finds all scenes that are not latest and ensures `is_latest: false`.
	pub async fn set_latest_scenes_in_db(
		api: &Services,
		crate_id: &CrateId,
	) -> Result<()> {
		match crate_id {
			CrateId::CratesIo(crate_id) => {
				Self::set_latest_scenes_in_db_crates_io(api, crate_id).await
			}
			CrateId::Github(_crate_id) => todo!(),
		}
	}
	async fn set_latest_scenes_in_db_crates_io(
		api: &Services,
		crate_id: &CratesIoCrateId,
	) -> Result<()> {
		let latest_version =
			api.registry().latest_version(&crate_id.crate_name).await?;

		// entries that:
		// 1. have the same crate name
		// 2. not the latest version
		// 3. are marked as latest
		let mut should_not_be_latest = Self::get_scenes(api, doc! {
			"scene_id.crate_id":{
					"crate_name": &crate_id.crate_name,
					"version":	{
						"$ne": latest_version.to_string()
					}
			},
			"is_latest": true
		})
		.await?;


		for scene in should_not_be_latest.iter_mut() {
			scene.is_latest = false;
		}

		// entries that:
		// 1. are the latest version of this crate
		// 2. are not marked as latest
		let mut should_be_latest = Self::get_scenes(api, doc! {
			"scene_id.crate_id": CrateId::new_crates_io(&crate_id.crate_name, latest_version.clone()),
			"is_latest": false
		})
		.await?;
		for scene in should_be_latest.iter_mut() {
			scene.is_latest = true;
		}

		let all_scenes = should_not_be_latest
			.into_iter()
			.chain(should_be_latest.into_iter())
			.collect::<Vec<_>>();

		api.db().scenes().insert_many(&all_scenes).await?;

		Ok(())
	}

	async fn get_scenes(
		api: &Services,
		filter: Document,
	) -> Result<Vec<SceneDoc>> {
		let scenes = api
			.db()
			.scenes()
			.find()
			.filter(filter)
			.send()
			.await?
			.try_collect()
			.await?;
		Ok(scenes)
	}
}



#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::Result;
	use mongodb::bson::doc;
	use sweet::*;

	//TODO this test is inadequate, just checks whether some happened to be latest
	// which does mean the above function at least ran
	#[tokio::test]
	async fn works() -> Result<()> {
		let api = Services::init().await?;


		expect(
			api.db()
				.scenes()
				.find()
				.filter(doc! {"is_latest":true })
				.send()
				.await?
				.try_collect()
				.await?
				.len(),
		)
		.to_be_greater_than(1)?;

		Ok(())
	}
}
