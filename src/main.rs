use anyhow::Result;
use clap::Parser;
use cli_configuration::command_configuration::CommandConfiguration;
use cli_configuration::file_action::FileAction;
use glob::glob;
use std::path::PathBuf;
use std::process::exit;

mod cli_configuration;
mod dateformatter;
mod organiser;

use crate::cli_configuration::cli::{CLIAction, CLI};
use crate::organiser::default_organiser::DefaultOrganiser;
use crate::organiser::organiser::Organiser;

fn parse_configuration() -> CommandConfiguration {
    let cli = CLI::parse();

    let mut configuration: CommandConfiguration = match cli.action {
        CLIAction::Copy(configuration) => CommandConfiguration {
            pattern: configuration.pattern,
            target_directory: configuration.target_directory,
            source_directory: configuration.source_directory,
            action: FileAction::Copy,
        },
        CLIAction::Move(configuration) => CommandConfiguration {
            pattern: configuration.pattern,
            target_directory: configuration.target_directory,
            source_directory: configuration.source_directory,
            action: FileAction::Move,
        },
    };

    if configuration.source_directory.is_none() {
        match std::env::current_dir() {
            Ok(path) => configuration.source_directory = Some(path),
            Err(_) => {
                eprintln!("Source directory not specified and could not get current dir");
                exit(1);
            }
        }
    }

    configuration
}

fn main() -> Result<()> {
    let configuration = parse_configuration();

    if !configuration.target_directory.exists() {
        eprintln!("Destination folder does not exist");
        exit(1);
    }

    let pattern: PathBuf = [
        configuration
            .source_directory
            .as_ref()
            .unwrap()
            .to_str()
            .unwrap(),
        &configuration.pattern,
    ]
    .iter()
    .collect();

    println!("{}", pattern.to_str().unwrap());

    let organiser = DefaultOrganiser {};

    let glob_pattern = pattern.to_str().unwrap();
    for entry in glob(glob_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => organiser.handle_path(path, &configuration)?,
            Err(e) => {
                eprintln!("{:?}", e);
                continue;
            }
        }
    }

    Ok(())
}
