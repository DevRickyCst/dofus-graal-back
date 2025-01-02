// src/models/item_effect.rs

use diesel::prelude::*;
use crate::schema::item_effects;
use crate::models::{Item, EffectSingle};

#[derive(Debug, Queryable, Identifiable, Associations, Insertable)]
#[diesel(table_name = item_effects)]
#[diesel(primary_key(item_id, effect_id))]
#[diesel(belongs_to(Item, foreign_key = item_id))]
#[diesel(belongs_to(EffectSingle, foreign_key = effect_id))]
pub struct ItemEffect {
    pub item_id: i32,
    pub effect_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = item_effects)]
pub struct NewItemEffect {
    pub item_id: i32,
    pub effect_id: i32,
}
