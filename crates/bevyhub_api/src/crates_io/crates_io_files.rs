use anyhow::anyhow;
use anyhow::Result;
use axum::body::Bytes;
use flate2::read::GzDecoder;
use std::io::Cursor;
use std::io::Read;
use tar::Archive;
use crate::prelude::*;

/// Functions for getting files from tarballs in crates.io
pub struct CratesIoFiles;

/// The inner workings are not public 
impl CratesIoFiles {
	/// Fetch and cache the `Cargo.toml`
	/// If not found in storage this will trigger an unpack
	pub async fn cargo_manifest(api:&Services, crate_id: &CratesIoCrateId) -> Result<CargoManifest> {
		let path = storage_path::unpkg_path(crate_id, "Cargo.toml");
		let bytes = Self::get_or_unpack_tarball(api,crate_id, &path).await?;
		let cargo_manifest = toml_from_bytes(&bytes)?;
		Ok(cargo_manifest)
	}
	/// Fetch and cache the `Cargo.lock`
	/// If not found in storage this will trigger an unpack
	pub async fn cargo_lock(api:&Services, crate_id: &CratesIoCrateId) -> Result<CargoLock> {
		let path = storage_path::unpkg_path(crate_id, "Cargo.lock");
		let bytes = Self::get_or_unpack_tarball(api,crate_id, &path).await?;
		let cargo_manifest = toml_from_bytes(&bytes)?;
		Ok(cargo_manifest)
	}

	/// Arbitary files do not trigger an unpack because they may not be found
	pub async fn get_crate_file(api:&Services, crate_id: &CratesIoCrateId, file: &str) -> Result<Bytes> {
		let path = storage_path::unpkg_path(crate_id, file);
		if let Ok(file) = api.storage().get(&path).await {
			return Ok(file);
		}
		Self::unpack_cargo_if_needed(api,crate_id).await?;
		api.storage().get(&path).await
	}

	/// All crates should have a `Cargo.toml`.
	/// Checking for the file is a good way to see if the crate has been unpacked.
	pub async fn unpack_cargo_if_needed(api:&Services, crate_id: &CratesIoCrateId) -> Result<()> {
		let path = storage_path::unpkg_path(crate_id, "Cargo.toml");
		let _ = Self::get_or_unpack_tarball(api,crate_id, &path).await?;
		Ok(())
	}



	/// Only call this for files that should always be there. If the file is missing, an unpack is triggered.
	async fn get_or_unpack_tarball(
			api:&Services,
			crate_id: &CratesIoCrateId,
			path: &str,
		) -> Result<Bytes> {
			if let Ok(file) = api.storage().get(path).await {
				return Ok(file);
			}
			Self::unpack_tarball(api,crate_id).await?;
			let val = api.storage().get(path).await
			.map_err(|e| 
				anyhow! {"something went wrong, required file doesnt exist even after unpack\n{:?}",e})?;
			Ok(val)
	}
	
	/// Retrieves the tarball from registry and unpack it into storage
	/// Will error if no package found
	async fn unpack_tarball(api:&Services, crate_id: &CratesIoCrateId) -> Result<()> {
		let tarball = api.registry().tarball(crate_id).await?;
		let mut archive = Archive::new(GzDecoder::new(Cursor::new(tarball)));
	
		let to_store = archive.entries()?.map(|file|{
			// Make sure there wasn't an I/O error
			let mut file = file?;
	
			// read file contents
			let mut buff = Vec::new();
			let _len = file.read_to_end(&mut buff)?;
	
			// convert tarball path to storage path
			let path = file.header().path()?;
			let path = path.to_string_lossy()
			.to_string()
			.split("/").skip(1)
			.collect::<Vec<&str>>().join("/");
			let path =storage_path::unpkg_path(crate_id, &path);
	
			Ok((path, Bytes::from(buff)))
		}).collect::<Result<Vec<_>>>()?;
	
		api.storage().put_many(to_store).await?;
		
		// futures::future::try_join_all(futs).await?;
		Ok(())
	}
}



#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::Result;
	use flate2::read::GzDecoder;
	use std::io::Cursor;
	use sweet::*;
	use tar::Archive;

	#[tokio::test]
	async fn tarball() -> Result<()> {
		let api = Services::init().await?;
		let tarball = api
			.registry()
			.tarball(&CratesIoCrateId::bevyhub_template())
			.await?;

		let decoder = GzDecoder::new(Cursor::new(tarball));
		let mut arch = Archive::new(decoder);
		expect(arch.entries()?.count()).to_be_greater_than(10)?;

		// for file in arch.entries()? {
		// 	let file = file?;
		// 	let path = file.header().path()?;
		// 	println!("file: {}", path.to_string_lossy());
		// }
		Ok(())
	}
}
