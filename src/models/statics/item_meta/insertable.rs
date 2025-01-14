// Relative path: src/models/statics/item_meta/insertable.rs
use crate::schema::*;
use diesel::Insertable;


#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name = image_urls)]
pub struct NewImageUrls {
    pub icon: String,
    pub sd: String,
    pub hq: Option<String>,
    pub hd: Option<String>,
}


#[derive(Insertable)]
#[diesel(table_name = item_categories)]
pub struct NewItemCategory<'a> {
    pub id: i32,
    pub name: &'a str,
}


#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name = item_types)]
pub struct NewItemType {
    pub id: i32,
    pub name: String,
}


#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name = ranges)]
pub struct NewRange {
    pub min: i32,
    pub max: i32,
}


#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name = effect_singles)]
pub struct NewEffect {
    pub int_minimum: i32,
    pub int_maximum: i32,
    pub element_id: i32,
    pub ignore_int_min: bool,
    pub ignore_int_max: bool,
    pub formatted: String,
}


#[derive(Insertable)]
#[diesel(table_name = item_effects)]
pub struct NewItemEffect {
    pub ankama_id: i32,
    pub effect_id: i32,
}


#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name = recipe_singles)]
pub struct NewRecipe {
    pub item_ankama_id: i32,
    pub item_subtype: String,
    pub quantity: i32,
}


#[derive(Insertable)]
#[diesel(table_name = item_recipes)]
pub struct NewItemRecipe {
    pub ankama_id: i32,
    pub recipe_id: i32,
}
