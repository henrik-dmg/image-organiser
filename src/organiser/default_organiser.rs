use crate::cli_configuration::command_configuration::CommandConfiguration;
use crate::cli_configuration::file_action::FileAction;
use crate::dateformatter::formatter::DateFormatter;
use crate::organiser::organiser::Organiser;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::PathBuf;

pub struct DefaultOrganiser;

impl Organiser for DefaultOrganiser {
    fn handle_path(&self, path: PathBuf, configuration: &CommandConfiguration) -> Result<()> {
        let path_name = path
            .to_str()
            .with_context(|| format!("Could not convert path to string"))?;

        println!("{:?}", path);
        let creation_date = match path.metadata() {
            Ok(metadata) => metadata
                .created()
                .with_context(|| format!("Could not read creation date of file {}", path_name))?,
            Err(_) => {
                eprintln!("Could not read metadata of file {}", path_name);
                return Ok(());
            }
        };

        let datetime: DateTime<Utc> = creation_date.into();
        let date_string = self.make_folder_name(datetime);

        let file_name = path
            .file_name()
            .with_context(|| format!("Could not get file name of file {}", path_name))?;

        let new_path = configuration
            .target_directory
            .join(date_string)
            .join(file_name);
        let parent_dir = new_path.parent().with_context(|| {
            format!(
                "could not get parent folder of file `{}`",
                new_path.to_str().unwrap()
            )
        })?;

        match configuration.action {
            FileAction::Copy => {
                fs::create_dir_all(parent_dir).expect("Failed to create directory");
                fs::copy(&path, &new_path).expect("Failed to copy file");
            }
            FileAction::Move => {
                fs::create_dir_all(parent_dir).expect("Failed to create directory");
                fs::rename(&path, &new_path).expect("Failed to move file");
            }
        }

        Ok(())
    }
}

impl DateFormatter for DefaultOrganiser {
    fn make_folder_name(&self, datetime: DateTime<Utc>) -> String {
        let year = datetime.format("%Y/%m").to_string();
        println!("{}", year);
        return year;
    }
}
