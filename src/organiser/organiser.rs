use crate::cli_configuration::command_configuration::CommandConfiguration;
use anyhow::Result;
use std::path::PathBuf;

trait Organiser {
    fn handle_path(path: PathBuf, configuration: &CommandConfiguration) -> Result<()>;
}
