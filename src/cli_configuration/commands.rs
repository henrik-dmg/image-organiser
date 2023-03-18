use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Args)]
pub struct CommandConfiguration {
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CLI {
    #[command(subcommand)]
    pub action: FileAction,
}

#[derive(Subcommand)]
pub enum FileAction {
    /// Copies the matched files to the target directory
    Copy(CommandConfiguration),
    /// Moves the matched files to the target directory
    Move(CommandConfiguration),
}
