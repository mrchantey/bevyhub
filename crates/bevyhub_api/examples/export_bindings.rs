use bevyhub_api::prelude::*;
use std::fs;
use std::path::PathBuf;
use ts_rs::TS;





fn main() -> anyhow::Result<()> {
	let path = PathBuf::from("bindings");
	fs::remove_dir_all(&path).ok();
	fs::create_dir_all(&path).ok();
	SceneDoc::export_all_to(&path)?;
	CrateDoc::export_all_to(&path)?;
	export_index()?;
	Ok(())
}


fn export_index() -> anyhow::Result<()> {
	let exports = fs::read_dir("bindings")?
		.filter_map(|entry| {
			let entry = entry.ok()?;
			let path = entry.path();
			if path.is_file() {
				let filename = path.file_name()?.to_string_lossy().to_string();
				Some(format!("export * from './{}';", filename))
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
		.join("\n");

	fs::write("bindings/index.ts", exports)?;

	Ok(())
}
