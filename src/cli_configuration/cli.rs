use crate::cli_configuration::cli_arguments::CLIArguments;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CLI {
    #[command(subcommand)]
    pub action: CLIAction,
}

#[derive(Subcommand)]
pub enum CLIAction {
    /// Copies the matched files to the target directory
    Copy(CLIArguments),
    /// Moves the matched files to the target directory
    Move(CLIArguments),
}
