// Relative path: src/models/statics/item_meta/insertable.rs
use crate::schema::*;
use diesel::Insertable;

#[derive(Insertable, serde::Deserialize, serde::Serialize)]
#[diesel(table_name = image_urls)]
pub struct NewImageUrls {
    pub icon: String,
    pub sd: String,
    pub hq: Option<String>,
    pub hd: Option<String>,
}

#[derive(Insertable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = item_categories)]
pub struct NewItemCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, serde::Deserialize, serde::Serialize)]
#[diesel(table_name = item_types)]
pub struct NewItemType {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, serde::Deserialize, serde::Serialize)]
#[diesel(table_name = ranges)]
pub struct NewRange {
    pub min: i32,
    pub max: i32,
}

#[derive(Insertable, serde::Deserialize, serde::Serialize, Clone, Debug)]
#[diesel(table_name = effects)]
pub struct NewEffect {
    pub item_id: Option<i32>,
    pub int_minimum: i32,
    pub int_maximum: i32,
    pub element_id: Option<i32>,
    pub ignore_int_min: bool,
    pub ignore_int_max: bool,
    pub formatted: String,
}

#[derive(Insertable, serde::Deserialize, serde::Serialize, Clone, Debug)]
#[diesel(table_name = recipes)]
pub struct NewRecipe {
    pub item_id: Option<i32>,
    pub item_ankama_id: i32,
    pub item_subtype: String,
    pub quantity: i32,
}
