// Relative path: src/operations.rs

use crate::api::client::*;
use crate::db::operations::{handler::save_categories, item::save_item};
use diesel::PgConnection;
use reqwest::Error;

pub async fn sync_items(conn: &mut PgConnection) -> Result<(), Error> {
    // Insérer les catégories en base une fois
    match save_categories(conn) {
        Ok(_) => println!("Les catégories ont été insérées avec succès."),
        Err(e) => eprintln!("Erreur lors de l'insertion des catégories : {:?}", e),
    };

    for category in crate::constant::ITEM_CATEGORIES {
        let category_id = category.id;
        let category_name = category.name;

        let mut imported_count = 0;
        let mut failed_count = 0;

        let list_id = fetch_items_category_id(category_name).await?;

        for id in list_id {
            let item = fetch_single_item(id, category_name).await?;
            match save_item(conn, item, category_id) {
                Ok(_) => imported_count += 1,
                Err(_) => failed_count += 1,
            }
        }
        println!(
            "Catégorie {}: {} items importés avec succès, {} items échoués.",
            category_name, imported_count, failed_count
        );
    }

    Ok(())
}