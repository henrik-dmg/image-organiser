use anyhow::Result;
use clap::Parser;
use cli::action::Action;
use cli::configuration::Configuration;
use glob::glob;
use std::path::PathBuf;
use std::process::exit;

mod cli;
mod dateformatter;
mod organiser;

use crate::cli::cli::{CLIAction, CLI};
use crate::organiser::default_organiser::DefaultOrganiser;
use crate::organiser::organiser::Organiser;

fn parse_configuration() -> Configuration {
    let cli = CLI::parse();

    let mut configuration: Configuration = match cli.action {
        CLIAction::Copy(configuration) => Configuration {
            pattern: configuration.pattern,
            target_directory: configuration.target_directory,
            source_directory: configuration.source_directory,
            action: Action::Copy,
        },
        CLIAction::Move(configuration) => Configuration {
            pattern: configuration.pattern,
            target_directory: configuration.target_directory,
            source_directory: configuration.source_directory,
            action: Action::Move,
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
