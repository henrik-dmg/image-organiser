use anyhow::{Context, Result};
use cli::configuration::Configuration;
use glob::glob;
use std::io::{self, BufWriter, StderrLock, StdoutLock, Write};
use std::path::PathBuf;
use std::process::exit;

mod cli;
mod dateformatter;
mod organiser;

use crate::cli::cli::parse_configuration;
use crate::dateformatter::formatter::DateFormatter;
use crate::organiser::organiser::Organiser;

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut stdout_handle = BufWriter::new(stdout.lock());
    let stderr = io::stderr();
    let mut stderr_handle = BufWriter::new(stderr.lock());

    let configuration = parse_configuration()?;

    if !configuration.target_directory.exists() {
        writeln!(stderr_handle, "Destination folder does not exist")?;
        exit(1);
    }

    let formatter = DateFormatter {
        strategy: configuration.strategy.to_owned(),
    };
    let mut organiser = Organiser { formatter };

    let pattern = &configuration.pattern;
    let globby = glob(pattern).context("Failed to read glob pattern")?;

    for entry in globby {
        match entry {
            Ok(path) => {
                handle_path(
                    &path,
                    &mut organiser,
                    &configuration,
                    &mut stdout_handle,
                    &mut stderr_handle,
                )?;
            }
            Err(error) => {
                writeln!(stderr_handle, "{}", &error.to_string())?;
                continue;
            }
        }
    }

    Ok(())
}

fn handle_path(
    path: &PathBuf,
    organiser: &mut Organiser,
    configuration: &Configuration,
    stdout_handle: &mut BufWriter<StdoutLock>,
    stderr_handle: &mut BufWriter<StderrLock>,
) -> Result<()> {
    match organiser.handle_path(&path, &configuration) {
        Ok(file_handled) => {
            if file_handled {
                writeln!(
                    stdout_handle,
                    "Successfully handled file {}",
                    path.display().to_string()
                )?;
            } else {
                writeln!(
                    stdout_handle,
                    "Skipped file {} because destination already exists",
                    path.display().to_string()
                )?;
            }
        }
        Err(error) => {
            writeln!(stderr_handle, "{}", &error.to_string())?;
        }
    }
    Ok(())
}
