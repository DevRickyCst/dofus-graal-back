mod schema;
mod cli;
mod operations;
mod api;
use dotenv::dotenv;
mod db_operations;
pub mod models;
pub mod constant;
use cli::{CliArgs, build_cli};
use operations::sync_items;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};


fn main() {

    dotenv().ok();

    // Database initialisation
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Erreur lors de la création du pool de connexions");

    let mut conn = pool.get().expect("Impossible d'obtenir une connexion");

    // Cli 
    let args = CliArgs::from_matches(build_cli().get_matches());
    match args.mode.as_str() {
        /*"delete" => match delete_table(&args.table, &mut conn) {
            Ok(rows_deleted) => println!("{} lignes supprimées dans la table '{}'.", rows_deleted, args.table),
            Err(e) => eprintln!("Erreur lors de la suppression : {:?}", e),
        },*/
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
        },
        _ => {
            eprintln!("Mode inconnu : {}", args.mode);
            std::process::exit(1);
        }
    }
}
