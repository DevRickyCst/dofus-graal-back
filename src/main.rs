// Relative path: src/main.rs
mod api;
pub mod cli;
pub mod constant;
pub mod db;
pub mod models;
mod operations;
mod schema;
pub mod test_utils;
use dotenv::dotenv;

use cli::{build_cli, CliArgs};

use db::connection::establish_connection;
use operations::{delete_table, sync_items};
fn main() {
    dotenv().ok();

    // Database initialisation
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    let pool = establish_connection(&database_url);
    let mut conn = pool.get().expect("Impossible d'obtenir une connexion");

    // Cli
    let args = CliArgs::from_matches(build_cli().get_matches());
    match args.mode.as_str() {
        "delete" => match delete_table(&mut conn) {
            Ok(rows_deleted) => println!("{} lignes supprimées.", rows_deleted),
            Err(e) => eprintln!("Erreur lors de la suppression : {:?}", e),
        },
        "sync" => {
            // Appel à `call_api` dans un contexte async
            tokio::runtime::Runtime::new()
                .expect("Impossible de créer le runtime tokio")
                .block_on(async {
                    match sync_items(&mut conn).await {
                        Ok(_) => println!("Appel API réussi."),
                        Err(e) => eprintln!("Erreur lors de l'appel API : {:?}", e),
                    }
                });
        }
        _ => {
            eprintln!("Mode inconnu : {}", args.mode);
            std::process::exit(1);
        }
    }
}
