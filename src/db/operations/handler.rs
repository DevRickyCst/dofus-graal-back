// Relative path: src/db_operations/handler_item_meta.rs
use crate::db::operations::crud::*;
use crate::models::statics::item_meta::{insertable::*, queryable::*};
use diesel::result::Error;
use diesel::PgConnection;

pub fn save_categories(conn: &mut PgConnection) -> Result<(), Error> {
    use crate::schema::item_categories::dsl::*;
    use diesel::prelude::*;

    for category in crate::constant::get_item_categories() {
        diesel::insert_into(item_categories)
            .values(category)
            .on_conflict_do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

pub fn handle_item_type(conn: &mut PgConnection, item_type: &NewItemType) -> Result<i32, Error> {
    use crate::schema::item_types::dsl::*;
    use diesel::prelude::*;

    match item_types
        .filter(id.eq(item_type.id))
        .select(id)
        .first::<i32>(conn)
    {
        Ok(existing_id) => Ok(existing_id),
        Err(diesel::result::Error::NotFound) => {
            let new_item_type: ItemType = insert_and_retrieve_record(item_type, item_types, conn)?;
            Ok(new_item_type.id)
        }
        Err(e) => {
            eprintln!("Database query error: {:?}", e);
            Err(e)
        }
    }
}

pub fn handle_effects(
    conn: &mut PgConnection,
    ankama_id: i32,
    effects: Option<Vec<NewEffect>>,
) -> Result<(), Error> {
    if let Some(mut effects) = effects {
        for effect in &mut effects {
            use crate::schema::effects::dsl::*;
            effect.item_id = Some(ankama_id);

            let _: Effect = insert_and_retrieve_record(effect.clone(), effects, conn)?;
        }
    }
    Ok(())
}

pub fn handle_recipes(
    conn: &mut PgConnection,
    ankama_id: i32,
    recipes: Option<Vec<NewRecipe>>,
) -> Result<(), Error> {
    if let Some(mut recipes) = recipes {
        for recipe in &mut recipes {
            use crate::schema::recipes::dsl::*;

            recipe.item_id = Some(ankama_id);

            // Insérer chaque recette dans la base de données
            let _: Recipe = insert_and_retrieve_record(recipe.clone(), recipes, conn)?;
        }
    }
    Ok(())
}
