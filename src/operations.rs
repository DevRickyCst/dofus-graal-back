// Relative path: src/operations.rs

use diesel::prelude::*;
use diesel::dsl::delete;
use diesel::PgConnection;
use crate::db_operations::{
    insert_item::save_item,
    handler_item_meta::save_categories,
};
use crate::api::client::*;
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

        for id in list_id{
            let item = fetch_single_item(id, category_name).await?;
            match save_item(conn, item, category_id){
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


pub fn delete_table(conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
    println!("Suppression des données dans toutes les tables");

    let mut total_deleted = 0;

    // Liste des suppressions
    total_deleted += delete(crate::schema::item_types::table).execute(conn)?;
    total_deleted += delete(crate::schema::characters::table).execute(conn)?;
    total_deleted += delete(crate::schema::effect_singles::table).execute(conn)?;
    total_deleted += delete(crate::schema::recipe_singles::table).execute(conn)?;
    total_deleted += delete(crate::schema::servers::table).execute(conn)?;
    total_deleted += delete(crate::schema::ranges::table).execute(conn)?;
    total_deleted += delete(crate::schema::items::table).execute(conn)?;
    total_deleted += delete(crate::schema::image_urls::table).execute(conn)?;
    total_deleted += delete(crate::schema::item_effects::table).execute(conn)?;

    println!("Total des lignes supprimées : {}", total_deleted);

    Ok(total_deleted)
}

