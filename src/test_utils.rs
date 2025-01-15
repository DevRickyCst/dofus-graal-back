use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::db::connection::establish_connection;
use std::env;

/// Retourne une connexion à la base de données pour les tests
pub fn get_test_db_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL doit être défini");
    let pool = establish_connection(&database_url);
    pool.get().expect("Impossible d'obtenir une connexion")
}

pub fn setup_test_environment() {
    dotenv::from_filename(".env.test").ok();
}
