// Relative path: src/db_operations/insert_item.rs
use diesel::result::Error;
use diesel::PgConnection;

use crate::api::models::DFDDetailedItem;
use crate::db::operations::crud::*;
use crate::db::operations::handler::*;
use crate::models::statics::item::{insertable::*, queryable::*};
use crate::models::statics::item_meta::queryable::*;

pub fn save_item(
    conn: &mut PgConnection,
    item: DFDDetailedItem,
    categ_id: i32,
) -> Result<(), Error> {
    use crate::schema::image_urls::dsl as image_urls_table;
    use crate::schema::ranges::dsl as ranges_table;

    let image_urls: ImageUrls =
        insert_and_retrieve_record(&item.image_urls, image_urls_table::image_urls, conn)?;

    let _type_id: i32 = handle_item_type(conn, &item.item_type)?;

    let _range_id: Option<i32> = if let Some(range_data) = item.range.as_ref() {
        
        let _range : Range = insert_and_retrieve_record(
            range_data,
            ranges_table::ranges,
            conn,
        )?;

        Some(_range.id)
    } else {
        None
    };

    use crate::schema::items::dsl::*;

    let _new_item = NewItem {
        ankama_id: item.ankama_id, // Assurez-vous que `item.id` est optionnel et fourni si nécessaire
        category_id: categ_id,
        type_id: _type_id,
        name: &item.name,
        description: &item.description,
        level: item.level,
        pods: item.pods,
        image_urls_id: image_urls.id,
        ap_cost: item.ap_cost,
        max_cast_per_turn: item.max_cast_per_turn,
        is_weapon: item.is_weapon.unwrap_or(false),
        is_two_handed: item.is_two_handed,
        critical_hit_probability: item.critical_hit_probability,
        critical_hit_bonus: item.critical_hit_bonus,
        range_id: _range_id,
    };

    // Insérer l'item dans la base de données
    let new_item: Item = insert_and_retrieve_record(_new_item, items, conn)?;

    // Gestion des Effets
    let effect_ids: Option<Vec<i32>> = handle_effects(conn, item.effects)?;
    handle_item_effects(conn, new_item.ankama_id, effect_ids)?;

    let recipe_ids: Option<Vec<i32>> = handle_recipes(conn, item.recipe)?;
    handle_item_recipes(conn, new_item.ankama_id, recipe_ids)?;

    Ok(())
}
