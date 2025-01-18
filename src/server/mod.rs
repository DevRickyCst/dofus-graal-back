
use axum::{
    routing::get,
    Router,
};

async fn hello_world() -> &'static str {
    println!("hello world !!");
    "Hello world!"
}


pub async fn start_server() {

    let hello_route = Router::new()
        .route(
            "/",
             get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, hello_route).await.unwrap();


}