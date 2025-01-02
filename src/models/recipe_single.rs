// src/models/recipe_single.rs

use diesel::prelude::*;
use crate::schema::recipe_singles;

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[diesel(table_name = recipe_singles)]
pub struct RecipeSingle {
    pub id: i32,
    pub item_ankama_id: i32,
    pub item_subtype: String,
    pub quantity: i32,
}
