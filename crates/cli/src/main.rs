#![feature(exit_status_error)]
use clap::Parser;
use clap::Subcommand;
mod api;
mod build_web;
use anyhow::Result;

fn main() -> Result<()> { BevyhubCli::run() }

/// Welcome to the Bevyhub CLI
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct BevyhubCli {
	#[command(subcommand)]
	command: Commands,
}
#[derive(Subcommand)]
enum Commands {
	BuildWeb(build_web::BuildBevyhubWeb),
	Populate(api::PopulateCommand),
}

impl BevyhubCli {
	pub fn run() -> Result<()> {
		match Self::parse().command {
			Commands::BuildWeb(cmd) => cmd.run(),
			Commands::Populate(cmd) => cmd.run(),
		}
	}
}
