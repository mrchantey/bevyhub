use forky::prelude::Subcommand;
use subcommands::*;
mod subcommands;

fn main() { Cli.run_with_cli_args().unwrap(); }

struct Cli;

impl Subcommand for Cli {
	fn name(&self) -> &'static str { "Bevyhub cli" }
	fn about(&self) -> &'static str { "Welcome to the Bevyhub cli" }

	fn append_command(&self, command: clap::Command) -> clap::Command {
		command.subcommand_required(true)
	}

	fn subcommands(&self) -> Vec<Box<dyn Subcommand>> {
		vec![Box::new(BuildBevyhubWeb)]
	}
}
