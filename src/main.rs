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
    fn run_with_configuration_file(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl ImageOrganiser for std::path::PathBuf {
    fn run_with_configuration_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.to_str().unwrap();
        let configuration_file_contents = std::fs::read_to_string(&self)
            .with_context(|| format!("Could not read file \"{}\"", path))?;
        println!("file content: {}", configuration_file_contents);
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer

    let args = CIArguments::parse();

    args.configuration_path.run_with_configuration_file()?;
    Ok(())
}
