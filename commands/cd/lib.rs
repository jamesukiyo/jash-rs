mod clap;

use std::env;

use clap::CdArgs;
use jash_common::{CommandResult, ShellCommand};

pub struct CdCommand;

impl ShellCommand for CdCommand {
	type Args = CdArgs;

	fn name() -> &'static str {
		"cd"
	}

	fn execute_with_args(args: CdArgs) -> CommandResult {
		let target_dir = match args.directory {
			Some(dir) => {
				if dir == "~" {
					env::var("HOME")
						.map_err(|_| "cd: HOME not set".to_string())?
				} else {
					dir
				}
			}
			None => {
				// No arg so go to ~ like other shells
				env::var("HOME").map_err(|_| "cd: HOME not set".to_string())?
			}
		};

		env::set_current_dir(&target_dir).map_err(|e| format!("cd: {e}"))?;

		Ok(())
	}
}
