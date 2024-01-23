use crate::cli::action::Action;
use crate::cli::arguments::Arguments;
use crate::cli::configuration::Configuration;
use crate::dateformatter::strategy::DateGroupingStrategy;
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
            target_directory: configuration.target_directory,
            pattern: configuration.pattern,
            action: Action::Copy,
            strategy: configuration
                .strategy
                .unwrap_or(DateGroupingStrategy::Month),
        },
        CLIAction::Move(configuration) => Configuration {
            target_directory: configuration.target_directory,
            pattern: configuration.pattern,
            action: Action::Move,
            strategy: configuration
                .strategy
                .unwrap_or(DateGroupingStrategy::Month),
        },
    };

    Ok(configuration)
}
