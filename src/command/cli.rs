use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
#[clap(version)]
pub struct FoioArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType
}


#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create the "foio" folder and initialize the files for working with "foio."
    Init(InitCommand),
    /// Create the config file and the "foio" shell script
    Setup(SetupCommand),
    /// Command to open a page, calendar, or mm-dd-year file (if present in the "foio" directory).
    Open(OpenCommand)
}


#[derive(Debug, Parser)]
pub struct InitCommand {
    #[clap(flatten)]
    pub log_options: Option<LogOptions>,
}


#[derive(Debug, Parser)]
pub struct SetupCommand {
    #[clap(flatten)]
    pub log_options: Option<LogOptions>,
}


#[derive(Debug, Parser)]
pub struct LogOptions {
    #[clap(short, long)]
    pub log: bool,
}


#[derive(Debug, Parser)]
pub struct OpenCommand {
    /// page, calendar or mm-dd-year 
    #[clap()]
    pub path: String,
}