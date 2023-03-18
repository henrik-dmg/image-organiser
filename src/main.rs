use std::path::PathBuf;
use std::process::exit;
use std::{fs, thread::current};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use glob::glob;

mod cli_configuration;

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
    let config = parse_configuration();

    if !config.target_directory.exists() {
        eprintln!("Destination folder does not exist");
        exit(1);
    }

    let pattern: PathBuf = [
        config.source_directory.unwrap().to_str().unwrap(),
        &config.pattern,
    ]
    .iter()
    .collect();

    println!("{}", pattern.to_str().unwrap());

    for entry in glob(&pattern.to_str().unwrap()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path);
                let creation_date = match path.metadata() {
                    Ok(metadata) => metadata.created().with_context(|| {
                        format!(
                            "Could not read creation date of file {}",
                            path.to_str().unwrap()
                        )
                    })?,
                    Err(_) => continue,
                };

                let datetime: DateTime<Utc> = creation_date.into();
                let month_year = datetime.format("%Y-%m").to_string();
                let new_path = config
                    .target_directory
                    .join(month_year)
                    .join(path.file_name().unwrap());
                let parent_dir = new_path.parent().unwrap();
                // fs::create_dir_all(parent_dir).expect("Failed to create directory");
                // fs::rename(&path, &new_path).expect("Failed to move file");
            }
            Err(e) => {
                eprintln!("{:?}", e);
                continue;
            }
        }
    }

    Ok(())
}
