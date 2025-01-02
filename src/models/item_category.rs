// src/models/item_category.rs

use diesel::prelude::*;
use crate::schema::item_categories;

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[diesel(table_name = item_categories)]
pub struct ItemCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = item_categories)]
pub struct NewItemCategory<'a> {
    pub name: &'a str,
}
