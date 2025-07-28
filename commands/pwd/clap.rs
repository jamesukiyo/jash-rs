use clap::Parser;

#[derive(Parser)]
#[command(name = "pwd")]
#[command(about = "Print working directory")]
pub struct PwdArgs {
	// no args but available for later
}
