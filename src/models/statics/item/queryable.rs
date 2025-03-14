// Relative path: src/models/statics/item/queryable.rs
use crate::schema::*;
use diesel::prelude::*;
use diesel::Identifiable;

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(ankama_id))]
#[diesel(table_name = items)]
pub struct Item {
    pub ankama_id: i32,
    pub category_id: i32,
    pub type_id: i32,
    pub name: String,
    pub description: String,
    pub level: i32,
    pub pods: Option<i32>,
    pub image_urls_id: i32,
    pub ap_cost: Option<i32>,
    pub max_cast_per_turn: Option<i32>,
    pub is_weapon: bool,
    pub is_two_handed: Option<bool>,
    pub range_id: Option<i32>,
    pub critical_hit_probability: Option<i32>,
    pub critical_hit_bonus: Option<i32>,
}
