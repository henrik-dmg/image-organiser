use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::io;

#[derive(Debug, Parser)]
struct CIArguments {
    configuration_path: std::path::PathBuf,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Configuration {
    target_directory: String,
    source_directory: String,
}

trait ImageOrganiser {
    fn run_with_configuration_file(
        &self,
        configuration: Configuration,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

struct DefaultImageOrganiser;

impl ImageOrganiser for DefaultImageOrganiser {
    fn run_with_configuration_file(
        &self,
        configuration: Configuration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Hello, world!");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer

    let args = CIArguments::parse(); // parse CLI arguments
    let configuration_file_path = args.configuration_path.to_str().unwrap(); // unwrap configuration file path
    let configuration_file_contents = std::fs::read_to_string(&args.configuration_path)
        .with_context(|| format!("Could not read file \"{}\"", configuration_file_path))?;
    let configuration: Configuration = serde_json::from_str(&configuration_file_contents)
        .with_context(|| format!("Could not parse file \"{}\"", configuration_file_path))?;

    let organiser = DefaultImageOrganiser {};
    return organiser.run_with_configuration_file(configuration);
}
