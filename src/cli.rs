use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Database CLI")]
#[command(
    about = "Permet de gérer les opérations de la base de données.",
    version = "1.0"
)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Serve,
    Cli {
        #[arg(value_enum)]
        action: CliAction,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum CliAction {
    Sync,
    Delete,
}
