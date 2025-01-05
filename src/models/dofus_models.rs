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

#[derive(Insertable)]
#[diesel(table_name = effect_singles)]
pub struct NewEffect<'a> {
    pub int_minimum: i32,
    pub int_maximum: i32,
    pub element_id: i32,
    pub ignore_int_min: bool,
    pub ignore_int_max: bool,
    pub formatted: &'a str,
}

// Range Model
#[derive(Debug, Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = ranges)]
pub struct Range {
    pub id: i32,
    pub min: i32,
    pub max: i32,
}

// RecipeSingle Model
#[derive(Debug, Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = recipe_singles)]
pub struct Recipe {
    pub id: i32,
    pub ankama_id: i32,
    pub item_subtype: String,
    pub quantity: i32,
}





