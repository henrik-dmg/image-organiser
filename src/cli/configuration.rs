use crate::cli::action::Action;
use crate::dateformatter::strategy::DateGroupingStragegy;
use std::path::PathBuf;

pub struct Configuration {
    /// The glob pattern to match files
    pub pattern: String,
    /// The directory to which files will be copied
    pub target_directory: PathBuf,
    /// The action to perform on the files
    pub action: Action,
    /// The strategy to use when organising files
    pub strategy: DateGroupingStragegy,
}
