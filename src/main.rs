use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use std::io::{self, Write};

#[derive(Debug, Parser)]
struct CIArguments {
    configuration_path: std::path::PathBuf,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer

    let args = CIArguments::parse();
    let path = args.configuration_path.to_str().unwrap();

    let configuration_file_contents = std::fs::read_to_string(&args.configuration_path)
        .with_context(|| format!("Could not read file \"{}\"", path))?;
    writeln!(handle, "file content: {}", configuration_file_contents)?;
    Ok(())
}
