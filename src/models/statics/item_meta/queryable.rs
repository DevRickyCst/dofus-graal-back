// Relative path: src/models/statics/item_meta/queryable.rs
use diesel::prelude::*;
use crate::schema::*;
use diesel::Identifiable;


#[derive(Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = image_urls)]
pub struct ImageUrls {
    pub id: i32,
    pub icon: String,
    pub sd: String,
    pub hq: Option<String>,
    pub hd: Option<String>,
}


#[derive(Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = item_categories)]
pub struct ItemCategory {
    pub id: i32,
    pub name: String,
}


#[derive(Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = item_types)]
pub struct ItemType {
    pub id: i32,
    pub name: String,
}


#[derive(Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = ranges)]
pub struct Range {
    pub id: i32,
    pub min: i32,
    pub max: i32,
}


#[derive(Queryable, Identifiable, serde::Deserialize)]
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


#[derive(Queryable, Identifiable, Insertable)]
#[diesel(table_name = item_effects)]
#[diesel(primary_key(ankama_id, effect_id))]
pub struct ItemEffect {
    pub ankama_id: i32,
    pub effect_id: i32,
}


#[derive(Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = recipe_singles)]
pub struct Recipe {
    pub id: i32,
    pub item_ankama_id: i32,
    pub item_subtype: String,
    pub quantity: i32,
}


#[derive(Queryable, Identifiable, Insertable)]
#[diesel(table_name = item_recipes)]
#[diesel(primary_key(ankama_id, recipe_id))]
pub struct ItemRecipe {
    pub ankama_id: i32,
    pub recipe_id: i32,
}
