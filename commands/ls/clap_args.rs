use clap::{Parser, arg};

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser)]
#[command(name = "ls")]
#[command(about = "List contents of a directory")]
pub struct LsArgs {
	/// Directory path to list
	#[arg(default_value = ".")]
	pub path: String,

	/// Show hidden files
	#[arg(short = 'a', long = "all")]
	pub all: bool,

	/// Use long format (currently no permissions)
	#[arg(short = 'l', long = "long")]
	pub long: bool,

	/// Human readable output, only applies when used with -l
	#[arg(short = 'H', long = "human-readable")]
	pub human_readable: bool,

	/// Reverse the sort order
	#[arg(short = 'r', long = "reverse")]
	pub reverse: bool,
}
