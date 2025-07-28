use clap::Parser;

#[derive(Parser)]
#[command(name = "cd")]
#[command(about = "Change directory")]
pub struct CdArgs {
	/// Directory to change to (defaults to home directory)
	pub directory: Option<String>,
}