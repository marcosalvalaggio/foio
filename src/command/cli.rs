use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
#[clap(version)]
pub struct FoioArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType
}


#[derive(Debug, Subcommand)]
pub enum EntityType {
    Init(InitCommand),
    Setup(SetupCommand),
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
    #[clap()]
    pub path: String,
}