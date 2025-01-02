// src/models/range.rs

use diesel::prelude::*;
use crate::schema::ranges;
use crate::models::Item;

#[derive(Debug, Queryable, Identifiable, Associations, Insertable)]
#[diesel(table_name = ranges)]
#[diesel(primary_key(item_id))]
#[diesel(belongs_to(Item, foreign_key = item_id))]
pub struct Range {
    pub item_id: i32,
    pub min: i32,
    pub max: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = ranges)]
pub struct NewRange {
    pub item_id: i32,
    pub min: i32,
    pub max: i32,
}
