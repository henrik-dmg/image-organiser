use crate::cli::action::Action;
use crate::dateformatter::strategy::DateGroupingStrategy;
use std::path::PathBuf;

pub struct Configuration {
    /// The directory to which files will be copied
    pub target_directory: PathBuf,
    /// The glob pattern to match files
    pub pattern: String,
    /// The action to perform on the files
    pub action: Action,
    /// The strategy to use when organising files
    pub strategy: DateGroupingStrategy,
}
