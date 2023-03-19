use crate::cli_configuration::file_action::FileAction;
use std::path::PathBuf;

pub struct CommandConfiguration {
    /// The glob pattern to match files
    pub pattern: String,
    /// The directory to which files will be copied
    pub target_directory: PathBuf,
    /// The directory from which files are copied
    pub source_directory: Option<PathBuf>,
    /// The action to perform on the files
    pub action: FileAction,
}
