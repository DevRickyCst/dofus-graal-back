// Relative path: src/db_operations/handler_item_meta.rs
use diesel::result::Error;
use diesel::PgConnection;
use crate::db_operations::crud::*;
use crate::models::statics::item_meta::{insertable::*, queryable::*};

pub fn save_categories(conn: &mut PgConnection) -> Result<(), Error> {
    use crate::schema::item_categories::dsl::*;
    use diesel::prelude::*;

    for category in crate::constant::ITEM_CATEGORIES {
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
            let new_item_type : ItemType = insert_and_retrieve_record(item_type, item_types, conn)?;
            Ok(new_item_type.id)
        }
        Err(e) => {
            eprintln!("Database query error: {:?}", e);
            Err(e)
        }
    }
}

pub fn handle_image_url(conn: &mut PgConnection, image_urls: &NewImageUrls) -> Result<i32, Error> {
    use crate::schema::image_urls::dsl as image_urls_table;
    let new_image : ImageUrls = insert_and_retrieve_record(image_urls, image_urls_table::image_urls, conn)?;
    Ok(new_image.id)
}


pub fn handle_item_range(conn: &mut PgConnection, range: Option<&NewRange>) -> Result<Option<i32>, Error> {
    if let Some(range) = range {
        use crate::schema::ranges::dsl::*;

        let range_record: Range = insert_and_retrieve_record(range, ranges, conn)?;
        Ok(Some(range_record.id))
    } else {
        Ok(None)
    }
}

pub fn handle_effects(conn: &mut PgConnection, effects: Option<Vec<NewEffect>>) -> Result<Option<Vec<i32>>, Error> {
    if let Some(effects) = effects {
        let mut effect_ids = Vec::new();
        for effect in effects {
            use crate::schema::effect_singles::dsl::*;

            let effect_record: Effect = insert_and_retrieve_record(effect, effect_singles, conn)?;
            effect_ids.push(effect_record.id);
        }
        Ok(Some(effect_ids))
    } else {
        Ok(None)
    }
}

pub fn handle_recipes(conn: &mut PgConnection, recipes: Option<Vec<NewRecipe>>) -> Result<Option<Vec<i32>>, Error> {
    if let Some(recipes) = recipes {
        let mut recipe_ids = Vec::new();
        for recipe in recipes {
            use crate::schema::recipe_singles::dsl::*;

            let recipe_record: Recipe = insert_and_retrieve_record(recipe, recipe_singles, conn)?;
            
            recipe_ids.push(recipe_record.id);
        }
        Ok(Some(recipe_ids))
    } else {
        Ok(None)
    }
}

pub fn handle_item_effects(
    conn: &mut PgConnection,
    _ankama_id: i32,
    effect_ids: Option<Vec<i32>>,
) -> Result<(), Error> {
    if let Some(_effect_ids) = effect_ids {
        for _effect_id in _effect_ids {
            use crate::schema::item_effects::dsl::*;

            let new_item_effect = NewItemEffect {
                ankama_id: _ankama_id,
                effect_id: _effect_id,
            };

            let _ : ItemEffect= insert_and_retrieve_record(new_item_effect, item_effects, conn)?;
        }
    }
    Ok(())
}

pub fn handle_item_recipes(
    conn: &mut PgConnection,
    _ankama_id: i32,
    recipe_ids: Option<Vec<i32>>,
) -> Result<(), Error> {
    if let Some(_recipe_ids) = recipe_ids {
        for _recipe_id in _recipe_ids {
            use crate::schema::item_recipes::dsl::*;

            let new_item_recipe = NewItemRecipe {
                ankama_id: _ankama_id,
                recipe_id: _recipe_id,
            };

            let _ : ItemRecipe=insert_and_retrieve_record(new_item_recipe, item_recipes, conn)?;
        }
    }
    Ok(())
}