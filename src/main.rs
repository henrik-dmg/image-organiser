use anyhow::Result;
use clap::Parser;
use glob::glob;
use std::path::PathBuf;
use std::process::exit;

mod cli_configuration;
mod file_organiser;

fn parse_configuration() -> cli_configuration::CommandConfiguration {
    let cli = cli_configuration::CLI::parse();

    let mut configuration = match cli.action {
        cli_configuration::FileAction::Copy(configuration) => configuration,
        cli_configuration::FileAction::Move(configuration) => configuration,
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

    let glob_pattern = pattern.to_str().unwrap();
    for entry in glob(glob_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => file_organiser::handle_path(path, &configuration)?,
            Err(e) => {
                eprintln!("{:?}", e);
                continue;
            }
        }
    }

    Ok(())
}
