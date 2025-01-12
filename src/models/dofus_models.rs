use diesel::prelude::*;
use crate::schema::*;
use diesel::{ Identifiable, Insertable };


// EffectSingle Model
#[derive(Debug, Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = effect_singles)]
pub struct Effect {
    pub id: i32,
    pub int_minimum: i32,
    pub int_maximum: i32,
    pub element_id: i32,
    pub ignore_int_min: bool,
    pub ignore_int_max: bool,
    pub formatted: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = effect_singles)]
pub struct NewEffect<'a> {
    pub int_minimum: i32,
    pub int_maximum: i32,
    pub element_id: i32,
    pub ignore_int_min: bool,
    pub ignore_int_max: bool,
    pub formatted: &'a str,
}


// RecipeSingle Model
#[derive(Debug, Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = recipe_singles)]
pub struct Recipe {
    pub id: i32,
    pub item_ankama_id: i32,
    pub item_subtype: String,
    pub quantity: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = recipe_singles)]
pub struct NewRecipe<'a> {
    pub item_ankama_id: i32,
    pub item_subtype: &'a str,
    pub quantity: i32,
}



#[derive(Debug, Queryable, Identifiable, Insertable)]
#[diesel(table_name = item_effects)]
#[diesel(primary_key(ankama_id, effect_id))] // Clé primaire composite
pub struct ItemEffect {
    pub ankama_id: i32,
    pub effect_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = item_effects)]
pub struct NewItemEffect {
    pub ankama_id: i32,
    pub effect_id: i32,
}

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[diesel(table_name = item_recipes)]
#[diesel(primary_key(ankama_id, recipe_id))] // Clé primaire composite
pub struct ItemRecipe {
    pub ankama_id: i32,
    pub recipe_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = item_recipes)]
pub struct NewItemRecipe {
    pub ankama_id: i32,
    pub recipe_id: i32,
}