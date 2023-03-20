use crate::cli::action::Action;
use crate::cli::arguments::Arguments;
use crate::cli::configuration::Configuration;
use crate::dateformatter::strategy::DateGroupingStragegy;
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

pub fn parse_configuration() -> Configuration {
    let cli = CLI::parse();

    let mut configuration: Configuration = match cli.action {
        CLIAction::Copy(configuration) => Configuration {
            pattern: configuration.pattern,
            target_directory: configuration.target_directory,
            source_directory: configuration.source_directory,
            action: Action::Copy,
            strategy: configuration
                .strategy
                .unwrap_or(DateGroupingStragegy::Month),
        },
        CLIAction::Move(configuration) => Configuration {
            pattern: configuration.pattern,
            target_directory: configuration.target_directory,
            source_directory: configuration.source_directory,
            action: Action::Move,
            strategy: configuration
                .strategy
                .unwrap_or(DateGroupingStragegy::Month),
        },
    };

    if configuration.source_directory.is_none() {
        match std::env::current_dir() {
            Ok(path) => configuration.source_directory = Some(path),
            Err(_) => {
                panic!("Source directory not specified and could not get current dir");
            }
        }
    }

    configuration
}
