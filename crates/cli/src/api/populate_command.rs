use super::*;
use anyhow::Result;
use bevyhub_api::prelude::*;
use clap::Parser;

/// Populate the db and storage with some test data
/// Note that populating Production will hit crates.io
/// meaning all packages must actually be published
#[derive(Parser)]
pub struct PopulateCommand {
	/// number of files to create
	#[arg()]
	paths: Vec<String>,
	/// repackage tarballs even if they exist, useful if the crate source has changed
	#[arg(short, long)]
	force_tarball: bool,
}


impl PopulateCommand {
	pub fn run(self) -> Result<()> {
		tokio::runtime::Runtime::new()?.block_on(async move {
			let crate_ids = self
				.paths
				.iter()
				.map(|p| LocalCrateId::parse(p))
				.collect::<Result<Vec<_>>>()?;

			let mut num_skipped = 0;
			let mut num_packaged = 0;
			for id in crate_ids.iter() {
				if !package_locally_if_needed(id, self.force_tarball)? {
					num_skipped += 1;
				} else {
					num_packaged += 1;
				}
			}
			println!(
				"packaged {num_packaged} tarballs and skipped {num_skipped}"
			);

			let api = Services::init().await?;
			if api.env == ApiEnvironment::Prod {
				println!("populating production is not allowed");
				return Ok::<(), anyhow::Error>(());
			}

			println!("populating with env {:?}", api.env);

			futures::future::try_join_all(vec![
				api.db().crates().clear(),
				api.db().scenes().clear(),
			])
			.await?;

			// let storage_futs =
			// 	crate_ids.iter().map(|id| api.crate_scenes(&id.crate_id));
			// let crates = futures::future::try_join_all(storage_futs).await?;
			let mut scene_lists = Vec::new();
			for id in crate_ids {
				// we need to do it sequentially to avoid crate upload before scene upload race
				scene_lists.push(api.all_scene_docs(&id.into()).await?);
			}
			let num_scenes = scene_lists.iter().map(|c| c.len()).sum::<usize>();


			println!(
				"populated {} crates with {} scenes",
				scene_lists.len(),
				num_scenes
			);

			Ok::<(), anyhow::Error>(())
		})
	}
}
