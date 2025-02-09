// Relative path: src/operations.rs

use diesel::dsl::delete;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn delete_items(conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
    println!("Suppression des données dans toutes les tables");

    let mut total_deleted = 0;

    // Liste des suppressions
    total_deleted += delete(crate::schema::item_types::table).execute(conn)?;
    total_deleted += delete(crate::schema::characters::table).execute(conn)?;
    total_deleted += delete(crate::schema::effects::table).execute(conn)?;
    total_deleted += delete(crate::schema::recipes::table).execute(conn)?;
    total_deleted += delete(crate::schema::servers::table).execute(conn)?;
    total_deleted += delete(crate::schema::ranges::table).execute(conn)?;
    total_deleted += delete(crate::schema::items::table).execute(conn)?;
    total_deleted += delete(crate::schema::image_urls::table).execute(conn)?;
    total_deleted += delete(crate::schema::item_categories::table).execute(conn)?;

    println!("Total des lignes supprimées : {}", total_deleted);

    Ok(total_deleted)
}
