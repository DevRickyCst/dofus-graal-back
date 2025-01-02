// src/models/item_recipe.rs

use diesel::prelude::*;
use crate::schema::item_recipes;
use crate::models::{Item, RecipeSingle};

#[derive(Debug, Queryable, Identifiable, Associations, Insertable)]
#[diesel(table_name = item_recipes)]
#[diesel(primary_key(item_id, recipe_id))]
#[diesel(belongs_to(Item, foreign_key = item_id))]
#[diesel(belongs_to(RecipeSingle, foreign_key = recipe_id))]
pub struct ItemRecipe {
    pub item_id: i32,
    pub recipe_id: i32,
}
