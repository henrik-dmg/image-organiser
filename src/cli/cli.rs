use crate::cli::action::Action;
use crate::cli::arguments::Arguments;
use crate::cli::configuration::Configuration;
use crate::dateformatter::strategy::DateGroupingStragegy;
use anyhow::Result;
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
    Copy(Arguments),
    /// Moves the matched files to the target directory
    Move(Arguments),
}

pub fn parse_configuration() -> Result<Configuration> {
    let cli = CLI::parse();

    let configuration = match cli.action {
        CLIAction::Copy(configuration) => Configuration {
            pattern: configuration.pattern,
            target_directory: configuration.target_directory,
            action: Action::Copy,
            strategy: configuration
                .strategy
                .unwrap_or(DateGroupingStragegy::Month),
        },
        CLIAction::Move(configuration) => Configuration {
            pattern: configuration.pattern,
            target_directory: configuration.target_directory,
            action: Action::Move,
            strategy: configuration
                .strategy
                .unwrap_or(DateGroupingStragegy::Month),
        },
    };

    Ok(configuration)
}
