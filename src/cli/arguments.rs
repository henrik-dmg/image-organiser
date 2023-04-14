use clap::Args;
use std::path::PathBuf;

use crate::dateformatter::strategy::DateGroupingStragegy;

#[derive(Args)]
pub struct Arguments {
    /// The glob pattern to match files
    pub pattern: String,
    /// The directory to which files will be copied
    pub target_directory: PathBuf,
    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
    #[arg(value_enum)]
    pub strategy: Option<DateGroupingStragegy>,
}
