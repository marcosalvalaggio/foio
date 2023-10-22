use clap::{Parser, Subcommand, Args};


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FoioArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    Init(InitCommand),
}


#[derive(Debug, Args)]
pub struct InitCommand {
    #[clap(short, long)]
    pub init: String
}

