// src/main.rs

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod api;
mod lib;
use std::env;

use crate::lib::establish_connection;
use crate::api::data_insertion::fetch_and_insert_item_categories;
use log::{info, error};
use env_logger::Env;

#[tokio::main]
async fn main() {
    // Initialiser le logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Établir le pool de connexions
    let pool = establish_connection();

    // Appeler la fonction pour récupérer et insérer les catégories
    fetch_and_insert_item_categories(&pool).await;
}
