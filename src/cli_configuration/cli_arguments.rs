use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct CLIArguments {
    /// The glob pattern to match files
    pub pattern: String,
    /// The directory to which files will be copied
    pub target_directory: PathBuf,
    /// The directory from which files are copied
    pub source_directory: Option<PathBuf>,
    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
}
