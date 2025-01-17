// Relative path: src/main.rs
mod api;
pub mod cli;
pub mod constant;
pub mod db;
pub mod models;
mod schema;
pub mod test_utils;
use dotenv::dotenv;
mod scripts;

use cli::{CliArgs, CliAction, Commands};
use db::connection::establish_connection;
use scripts::{delete_items::*, sync_items::*};
use clap::Parser;



fn main() {
    dotenv().ok();

    // Database initialisation
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    let pool = establish_connection(&database_url);
    let mut conn = pool.get().expect("Impossible d'obtenir une connexion");

    // Parse les arguments CLI
    let args = CliArgs::parse();

    match args.command {
        Commands::Serve => {
            println!("Lancement du serveur...");
        }
        Commands::Cli { action } => match action {
            CliAction::Sync => {
                tokio::runtime::Runtime::new()
                .expect("Impossible de créer le runtime tokio")
                .block_on(async {
                    match sync_items(&mut conn).await {
                        Ok(_) => println!("Appel API réussi."),
                        Err(e) => eprintln!("Erreur lors de l'appel API : {:?}", e),
                    }
                });            }
            CliAction::Delete => match delete_items(&mut conn) {
                Ok(rows_deleted) => println!("{} lignes supprimées.", rows_deleted),
                Err(e) => eprintln!("Erreur lors de la suppression : {:?}", e),
            }
        },
    }
}
