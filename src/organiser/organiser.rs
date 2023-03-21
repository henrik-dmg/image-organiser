use crate::cli::action::Action;
use crate::cli::configuration::Configuration;
use crate::dateformatter::formatter::DateFormatter;
use crate::printer::printer::Printer;
use anyhow::{Context, Ok, Result};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::PathBuf;

pub struct Organiser {
    pub formatter: DateFormatter,
}

impl Organiser {
    pub fn handle_path(
        &mut self,
        path: &PathBuf,
        configuration: &Configuration,
        printer: &mut Printer,
    ) -> Result<()> {
        let path_name = path
            .to_str()
            .with_context(|| format!("Could not convert path to string"))?;
        let file_name = path
            .file_name()
            .with_context(|| format!("Could not get file name of file {}", path_name))?;
        printer.regular_text(&format!("Handling file {}", path_name))?;

        let creation_date = path
            .metadata()
            .context("Could not read metadata")?
            .created()
            .context("Could not read creation date of file")?;

        let datetime: DateTime<Utc> = creation_date.into();
        let date_string = self.formatter.make_folder_name(datetime);

        let new_path = configuration
            .target_directory
            .join(date_string)
            .join(file_name);
        let new_path_name = new_path
            .to_str()
            .context("Could not convert path to string")?;

        if new_path.exists() {
            printer.regular_text(&format!("File {} already exists", new_path_name))?;
            return Ok(());
        }

        let parent_dir = new_path
            .parent()
            .with_context(|| format!("could not get parent folder of file `{}`", new_path_name))?;

        match configuration.action {
            Action::Copy => {
                fs::create_dir_all(parent_dir)
                    .with_context(|| format!("Failed to create directory"))?;
                fs::copy(&path, &new_path).with_context(|| {
                    format!("Failed to copy {} to {}", path_name, new_path_name)
                })?;
                printer.regular_text(&format!("Copied {} to {}", path_name, new_path_name))?;
            }
            Action::Move => {
                fs::create_dir_all(parent_dir)
                    .with_context(|| format!("Failed to create directory"))?;
                fs::rename(&path, &new_path).with_context(|| {
                    format!("Failed to move {} to {}", path_name, new_path_name)
                })?;
                printer.regular_text(&format!("Moved {} to {}", path_name, new_path_name))?;
            }
        }

        Ok(())
    }
}
