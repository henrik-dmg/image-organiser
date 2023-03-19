use crate::cli_configuration::command_configuration::CommandConfiguration;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum FileAction {
    /// Copies the matched files to the target directory
    Copy(CommandConfiguration),
    /// Moves the matched files to the target directory
    Move(CommandConfiguration),
}
