use crate::db::connection::DbPool;
use axum::extract::State;
use axum::{response::Json, routing::get, Router};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task;
use axum::extract::Query;
use crate::db::operations::loader::load_items;
#[derive(serde::Deserialize)]
pub struct PaginationParams {
    page: Option<i32>,
    page_size: Option<i32>,
}

pub async fn get_items(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Query(pagination): Query<PaginationParams>,
) -> Json<Value> {
    let results = task::spawn_blocking(move || {
        let mut conn = pool.get().expect("Impossible d'obtenir une connexion");

        let page = pagination.page.unwrap_or(1);
        let page_size = pagination.page_size.unwrap_or(10).min(20); // Limite la taille de la page à 20
        let offset = (page - 1) * page_size;

        let items = load_items(&mut conn, page_size, offset);

        items
    }).await.expect("Erreur lors de l'exécution du bloc de tâche");

    Json(json!(results))
}

async fn hello_world(pool: State<Arc<DbPool>>) -> &'static str {
    println!("hello world !!");
    "Hello trdt!"
}

pub async fn start_server(pool: Arc<Pool<ConnectionManager<PgConnection>>>) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = Router::new()
        .route("/items", get(get_items))
        .route("/", get(hello_world))
        .with_state(pool.clone());

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
