#![feature(exit_status_error)]
use forky::prelude::Subcommand;
mod build_web;
mod api;
mod aws;

fn main() { Cli.run_with_cli_args().unwrap(); }

struct Cli;

impl Subcommand for Cli {
	fn name(&self) -> &'static str { "Bevyhub CLI" }
	fn about(&self) -> &'static str { "Welcome to the Bevyhub CLI" }

	fn append_command(&self, command: clap::Command) -> clap::Command {
		command.subcommand_required(true)
	}

	fn subcommands(&self) -> Vec<Box<dyn Subcommand>> {
		vec![
			Box::new(build_web::BuildBevyhubWeb),
			Box::new(aws::S3Command),
			Box::new(api::PopulateCommand),
		]
	}
}
