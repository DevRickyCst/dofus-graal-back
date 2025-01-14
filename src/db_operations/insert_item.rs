// Relative path: src/db_operations/insert_item.rs
use diesel::PgConnection;
use diesel::result::Error;

use crate::models::statics::item::{insertable::*, queryable::*};
use crate::api::models::DFDDetailedItem;
use crate::db_operations::handler_item_meta::*;
use crate::db_operations::crud::*;


pub fn save_item(conn: &mut PgConnection, item: DFDDetailedItem, categ_id: i32) -> Result<(), Error> {


    let _image_url_id: i32 = handle_image_url(conn, &item)?;
    let _type_id: i32 = handle_item_type(conn, &item)?;
    let item_range_id: Option<i32> = handle_item_range(conn, &item)?;

    use crate::schema::items::dsl::*;

    let _new_item = NewItem {
        ankama_id: item.ankama_id, // Assurez-vous que `item.id` est optionnel et fourni si nécessaire
        category_id: categ_id,
        type_id: _type_id,
        name: &item.name,
        description: &item.description,
        level: item.level,
        pods: item.pods,
        image_urls_id: _image_url_id,
        ap_cost: item.ap_cost,
        max_cast_per_turn: item.max_cast_per_turn,
        is_weapon: item.is_weapon.unwrap_or(false),
        is_two_handed: item.is_two_handed,
        critical_hit_probability: item.critical_hit_probability,
        critical_hit_bonus: item.critical_hit_bonus,
        range_id: item_range_id
    };

    // Insérer l'item dans la base de données
    let new_item :Item = insert_and_retrieve_record(_new_item, items, conn)?;

    // Gestion des Effets
    let effect_ids : Option<Vec<i32>> = handle_effects(conn, &item)?;
    handle_item_effects(conn, new_item.ankama_id, effect_ids)?;

    let recipe_ids : Option<Vec<i32>> = handle_recipes(conn, &item)?;
    handle_item_recipes(conn, new_item.ankama_id, recipe_ids)?;

    Ok(())
}