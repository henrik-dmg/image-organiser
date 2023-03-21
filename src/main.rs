use anyhow::{Context, Result};
use glob::glob;
use std::path::PathBuf;
use std::process::exit;

mod cli;
mod dateformatter;
mod organiser;
mod printer;

use crate::cli::cli::parse_configuration;
use crate::dateformatter::formatter::DateFormatter;
use crate::organiser::organiser::Organiser;
use crate::printer::printer::Printer;

fn main() -> Result<()> {
    let configuration = parse_configuration()?;
    let mut printer = Printer::default();

    let action = configuration.action;
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

    let formatter = DateFormatter {
        strategy: configuration.strategy.clone(),
    };
    let mut organiser = Organiser { formatter };

    let glob_pattern = pattern.to_str().context("Invalid glob pattern provided")?;

    // writeln!(stdout_handle, "Glob pattern: {}", glob_pattern)?;

    let globby = glob(glob_pattern).context("Failed to read glob pattern")?;

    for entry in globby {
        match entry {
            Ok(path) => {
                organiser.handle_path(&path, &configuration, &mut printer)?;
                let file_name = path
                    .file_name()
                    .context("Could not get file name of file")?
                    .to_str()
                    .context("Could not convert file name to string")?;
                printer.notify_path_handled_successfully(file_name, action)?;
            }
            Err(_) => {
                printer.notify_path_handling_failed("test")?;
                continue;
            }
        }
    }

    Ok(())
}
