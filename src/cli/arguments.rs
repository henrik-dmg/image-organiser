use clap::Args;
use std::path::PathBuf;

use crate::dateformatter::strategy::DateGroupingStrategy;

#[derive(Args)]
pub struct Arguments {
    /// The directory to which files will be copied or moved
    pub target_directory: PathBuf,
    /// The glob pattern to match files
    pub pattern: String,
    /// The grouping strategy to use
    #[arg(value_enum)]
    pub strategy: Option<DateGroupingStrategy>,
}
