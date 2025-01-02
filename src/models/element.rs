// src/models/element.rs

use diesel::prelude::*;
use crate::schema::elements;

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[diesel(table_name = elements)]
pub struct Element {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = elements)]
pub struct NewElement<'a> {
    pub name: &'a str,
}
