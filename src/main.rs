use anyhow::{Context, Result};
use clap::Parser;
use std::io;

#[derive(Debug, Parser)]
struct CIArguments {
    configuration_path: std::path::PathBuf,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

trait ImageOrganiser {
    fn run_with_configuration_file(
        &self,
        path: std::path::PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

struct DefaultImageOrganiser;

impl ImageOrganiser for DefaultImageOrganiser {
    fn run_with_configuration_file(
        &self,
        path: std::path::PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let configuration_file_contents = std::fs::read_to_string(&path)
            .with_context(|| format!("Could not read file \"{}\"", path.to_str().unwrap()))?;
        println!("file content: {}", configuration_file_contents);
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer

    let args = CIArguments::parse();
    let organiser = DefaultImageOrganiser {};
    return organiser.run_with_configuration_file(args.configuration_path);
}
