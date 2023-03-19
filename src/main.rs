use anyhow::Result;
use glob::glob;
use std::path::PathBuf;
use std::process::exit;

mod cli;
mod dateformatter;
mod organiser;

use crate::cli::cli::parse_configuration;
use crate::dateformatter::formatter::DateFormatter;
use crate::dateformatter::strategy::DateGroupingStragegy;
use crate::organiser::organiser::Organiser;

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

    let formatter = DateFormatter {
        strategy: DateGroupingStragegy::Month,
    };
    let organiser = Organiser { formatter };

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
