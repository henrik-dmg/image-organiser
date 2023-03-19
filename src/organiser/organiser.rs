use crate::cli::configuration::Configuration;
use anyhow::Result;
use std::path::PathBuf;

pub trait Organiser {
    fn handle_path(&self, path: PathBuf, configuration: &Configuration) -> Result<()>;
}
