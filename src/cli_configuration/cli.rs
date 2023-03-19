use crate::cli_configuration::file_action::FileAction;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CLI {
    #[command(subcommand)]
    pub action: FileAction,
}
