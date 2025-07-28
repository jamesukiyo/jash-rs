mod clap;

use std::env;

use clap::PwdArgs;
use jash_common::{CommandResult, ShellCommand};

pub struct PwdCommand;

impl ShellCommand for PwdCommand {
	type Args = PwdArgs;

	fn name() -> &'static str {
		"pwd"
	}

	fn execute_with_args(_args: PwdArgs) -> CommandResult {
		let cwd = env::current_dir().map_err(|e| format!("pwd: {e}"))?;
		println!("{}", cwd.display());
		Ok(())
	}
}
