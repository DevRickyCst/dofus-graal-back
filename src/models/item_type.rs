// src/models/item_type.rs

use diesel::prelude::*;
use crate::schema::item_types;

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[diesel(table_name = item_types)]
pub struct ItemType {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = item_types)]
pub struct NewItemType<'a> {
    pub name: &'a str,
}