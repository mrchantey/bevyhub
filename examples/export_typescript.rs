use anyhow::Result;
use bevyhub::prelude::*;


fn main() -> Result<()> {
	TypescriptExporter::<SerdeTypeRegistry>::new().export()?;
	Ok(())
}
