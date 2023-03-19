use crate::cli_configuration::command_configuration::CommandConfiguration;
use anyhow::Result;
use std::path::PathBuf;

pub trait Organiser {
    fn handle_path(&self, path: PathBuf, configuration: &CommandConfiguration) -> Result<()>;
}
