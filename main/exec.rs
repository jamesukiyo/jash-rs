use jash_cd::CdCommand;
use jash_common::ShellCommand;
use jash_ls::LsCommand;
use jash_pwd::PwdCommand;

pub fn exec_command(input: &str) -> Result<(), String> {
	let parts: Vec<&str> = input.split_whitespace().collect();
	if parts.is_empty() {
		return Ok(());
	}

	let cmd = parts[0];
	let args = &parts[1..];

	if is_builtin(cmd) {
		exec_builtin(cmd, args)
	} else {
		Err(format!("Command not found: '{cmd}'"))
	}
}

pub fn is_builtin(cmd: &str) -> bool {
	matches!(cmd, "cd" | "exit" | "pwd" | "ls" | "help" | "reload")
}

pub fn exec_builtin(cmd: &str, args: &[&str]) -> Result<(), String> {
	match cmd {
		"cd" => {
			CdCommand::execute(args)?;
			Ok(())
		}
		"exit" => {
			std::process::exit(0);
		}
		"pwd" => {
			PwdCommand::execute(args)?;
			Ok(())
		}
		"ls" => {
			LsCommand::execute(args)?;
			Ok(())
		}
		"help" => {
			help();
			Ok(())
		}
		"reload" => {
			println!("reload: command not yet implemented");
			Ok(())
		}
		_ => Ok(()),
	}
}

fn help() {
	println!(" cd <dir>  - Change directory");
	println!(" pwd       - Print working directory");
	println!(" ls        - List files in the current directory");
	println!(" exit      - Exit jash");
	println!(" reload    - Reserved keyword config reloading");
}

// TODO: add config reloading
#[allow(dead_code)]
fn reload() {
	println!("reload: command not yet implemented");
}
