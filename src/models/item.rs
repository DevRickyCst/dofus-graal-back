// src/models/item.rs

use diesel::prelude::*;
use crate::schema::items;
use crate::models::{ItemCategory, ItemType, ImageUrls};

#[derive(Debug, Queryable, Identifiable, Associations, Insertable)]
#[diesel(table_name = items)]
#[diesel(belongs_to(ItemCategory, foreign_key = category_id))]
#[diesel(belongs_to(ItemType, foreign_key = type_id))]
#[diesel(belongs_to(ImageUrls, foreign_key = image_urls_id))]
pub struct Item {
    pub id: i32,
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
    pub critical_hit_probability: Option<i32>,
    pub critical_hit_bonus: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub id: i32,
    pub category_id: i32,
    pub type_id: i32,
    pub name: &'a str,
    pub description: &'a str,
    pub level: i32,
    pub pods: Option<i32>,
    pub image_urls_id: i32,
    pub ap_cost: Option<i32>,
    pub max_cast_per_turn: Option<i32>,
    pub is_weapon: bool,
    pub is_two_handed: Option<bool>,
    pub critical_hit_probability: Option<i32>,
    pub critical_hit_bonus: Option<i32>,
}
