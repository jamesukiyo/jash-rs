use clap::Parser;

// shared result type for all commands
pub type CommandResult = Result<(), String>;

// trait for all commands with clap
pub trait ShellCommand {
	type Args: Parser;

	fn name() -> &'static str;

	// cmd with parsed args
	fn execute_with_args(args: Self::Args) -> CommandResult;

	// cmd with raw string args
	fn execute(args: &[&str]) -> CommandResult {
		let args_with_program =
			std::iter::once(Self::name()).chain(args.iter().copied());

		match Self::Args::try_parse_from(args_with_program) {
			Ok(parsed_args) => Self::execute_with_args(parsed_args),
			Err(e) => match e.kind() {
				clap::error::ErrorKind::DisplayHelp => {
					print!("{e}");
					Ok(())
				}
				_ => {
					eprintln!("{e}");
					Err("Invalid arguments".to_string())
				}
			},
		}
	}
}
