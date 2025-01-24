use anyhow::Result;
use clap::Parser;
use forky::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Build an app for Bevyhub Web.
/// This tool is similar to trunk but with a focus on binaries and scenes
/// instead of html and other web assets.
#[derive(Debug, Parser)]
pub struct BuildBevyhubWeb {
	#[arg(short, long)]
	pub example: Option<String>,
	/// Specify the crate name
	#[arg(short, long)]
	package: Option<String>,

	/// Skip cargo build, wasm-bindgen and wasm-opt
	#[arg(long)]
	skip_build: bool,

	/// Build for release and optimize
	#[arg(long)]
	release: bool,

	/// Build for release and optimize
	#[arg(short, long, default_value = "target/wasm")]
	out_dir: String,

	/// Copy wasm files to a local directory
	#[arg(long)]
	copy_local: Option<String>,

	/// Copy specified scenes dir to directory specified by copy-local,
	/// for example `target/scenes`
	#[arg(long)]
	copy_scenes: Option<String>,

	/// Copy specified registries dir to directory specified by copy-local,
	/// for example `target/registries`
	#[arg(long)]
	copy_registries: Option<String>,
}





impl BuildBevyhubWeb {
	// untested, i prefer justfile
	pub fn run(self) -> Result<()> {
		println!("🚀 Building Bevyhub Web app...\n{:#?}", &self);

		println!("🚀 Running Cargo Build");
		self.run_cargo_build()?;
		run_print_size(
			"🧪 Cargo Build Succeeded - Size: ",
			&self.cargo_build_wasm_path(),
		)?;
		println!("🚀 Running Wasm Bindgen");
		self.run_wasm_bindgen()?;
		run_print_size(
			"🧪 Wasm Bindgen Succeeded - Size: ",
			&self.bindgen_path_wasm(),
		)?;
		println!("🚀 Running Wasm Opt");
		self.run_wasm_opt()?;
		run_print_size(
			"🧪 Wasm Opt Succeeded - Size: ",
			&self.bindgen_path_wasm(),
		)?;
		println!("🚀 Copying Local Files");
		self.run_copy_local()?;
		// run_commit_local(&args)?;

		println!("🚀 Build Succeeded");
		Ok(())
	}

	fn app_name(&self) -> &str { self.package.as_deref().unwrap_or("main") }

	fn cargo_build_wasm_path(&self) -> String {
		let build_config = if self.release { "release" } else { "debug" };

		let cargo_target_dir = std::env::var("CARGO_TARGET_DIR")
			.unwrap_or_else(|_| "target".to_string());

		let mut path = PathBuf::from(cargo_target_dir);
		path.push("wasm32-unknown-unknown");
		path.push(build_config);
		if let Some(example) = &self.example {
			path.push(format!("examples/{}.wasm", example));
		} else {
			path.push(format!("{}.wasm", self.app_name()));
		}
		path.to_string_lossy().to_string()
	}

	fn bindgen_path_wasm(&self) -> String {
		format!("{}/{}_bg.wasm", self.out_dir, self.app_name())
	}
	fn bindgen_path_js(&self) -> String {
		format!("{}/{}.js", self.out_dir, self.app_name())
	}
	fn run_cargo_build(&self) -> Result<()> {
		if self.skip_build {
			return Ok(());
		}

		let mut build_args =
			vec!["build", "--target", "wasm32-unknown-unknown"];
		if let Some(package) = &self.package {
			build_args.push("-p");
			build_args.push(package);
		}
		if self.release {
			build_args.push("--release");
		}
		if let Some(example) = &self.example {
			build_args.push("--example");
			build_args.push(example);
		}

		// Build the project
		let status = Command::new("cargo").args(&build_args).status()?;
		if !status.success() {
			anyhow::bail!("cargo build failed");
		}

		Ok(())
	}


	fn run_wasm_bindgen(&self) -> Result<()> {
		if self.skip_build {
			return Ok(());
		}

		fs::create_dir_all(&self.out_dir).ok();
		let wasm_path = self.cargo_build_wasm_path();

		let build_args = [
			"--out-name",
			&self.app_name(),
			"--out-dir",
			&self.out_dir,
			"--target",
			"web",
			"--no-typescript",
			&wasm_path,
		];
		// println!("wasm-bindgen {}", build_args.join(" "));

		let status = Command::new("wasm-bindgen").args(&build_args).status()?;
		if !status.success() {
			anyhow::bail!("wasm-bindgen failed");
		}

		Ok(())
	}

	fn run_wasm_opt(&self) -> Result<()> {
		if self.skip_build || !self.release {
			return Ok(());
		}

		let wasm_bindgen_path = self.bindgen_path_wasm();

		let status = Command::new("wasm-opt")
			.args(&["-Oz", "--output", &wasm_bindgen_path, &wasm_bindgen_path])
			.status()?;
		if !status.success() {
			anyhow::bail!("wasm-opt failed");
		}


		Ok(())
	}
	fn run_copy_local(&self) -> Result<()> {
		let Some(target_dir) = &self.copy_local else {
			return Ok(());
		};

		let crate_name = self.package.clone().unwrap_or_else(|| {
			std::env::current_dir()
				.unwrap()
				.file_name()
				.unwrap()
				.to_string_lossy()
				.to_string()
		});
		let target_dir =
			PathBuf::from(target_dir).canonicalize()?.join(crate_name);
		fs::create_dir_all(&target_dir).ok();
		fs::copy(
			&self.bindgen_path_wasm(),
			target_dir.join(format!("{}_bg.wasm", self.app_name())),
		)?;
		fs::copy(
			&self.bindgen_path_js(),
			target_dir.join(format!("{}.js", self.app_name())),
		)?;

		if let Some(scenes_dir_src) = &self.copy_scenes {
			FsExt::copy_recursive(scenes_dir_src, target_dir.join("scenes"))?;
		}
		if let Some(registries_dir_src) = &self.copy_registries {
			FsExt::copy_recursive(
				registries_dir_src,
				target_dir.join("registries"),
			)?;
		}

		Ok(())
	}
}


fn run_print_size(prefix: &str, path: &str) -> Result<()> {
	let metadata = fs::metadata(path)?;
	let size_b = metadata.len();
	let size_mb = size_b as f64 / 1024.0 / 1024.0;
	println!("{prefix}: {:.0} MB", size_mb);
	Ok(())
}



// fn run_commit_local(args: &Args) -> Result<()> {
// 	if !args.commit_local {
// 		return Ok(());
// 	}

// 	let Some(target_dir) = &args.copy_local else {
// 		anyhow::bail!("copy-local is required for commit-local");
// 	};

// 	let target_dir = PathBuf::from(target_dir).canonicalize()?;
// 	let target_dir = target_dir.to_string_lossy();
// 	let target_dir_cmd = format!("cd {}", target_dir);

// 	let commands = vec![
// 		&target_dir_cmd,
// 		"&& git config --global user.name \"github-actions[bot]\"",
// 		"&& git config --global user.email \"github-actions[bot]@users.noreply.github.com\"",
// 		"&& git add .",
// 		"&& git commit -m \"Deploy from GitHub Actions\"",
// 		"&& git push origin main",
// 	];

// 	println!("Running commands: {:#?}", target_dir);

// 	let status = parse_commands(commands).status()?;
// 	if !status.success() {
// 		anyhow::bail!("commit failed");
// 	}

// 	Ok(())
// }


// fn parse_commands(commands: Vec<&str>) -> Command {
// 	let mut command = Command::new(&commands[0]);
// 	for c in commands.iter().skip(1) {
// 		let split_cmd = c.split_whitespace().collect::<Vec<&str>>();
// 		command.args(split_cmd);
// 	}
// 	command
// }
