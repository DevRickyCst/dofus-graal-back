mod models;
mod schema;
use dotenv::dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use models::item_type::NewItemType;
use schema::item_types;

fn insert_item_types(connection: &mut PgConnection) -> Result<(), diesel::result::Error> {
    let item_types = [
        "items-consumables",
        "items-cosmetics",
        "items-resources",
        "items-equipment",
        "items-quest_items",
        "mounts",
        "sets",
    ];

    let new_item_types: Vec<NewItemType> = item_types
        .iter()
        .map(|name| NewItemType { name })
        .collect();

    diesel::insert_into(item_types::table)
        .values(&new_item_types)
        .execute(connection)?;

    Ok(())
}

fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect("Error connecting to the database");

    match insert_item_types(&mut connection) {
        Ok(_) => println!("Item types imported successfully."),
        Err(err) => eprintln!("Failed to import item types: {}", err),
    }
}
