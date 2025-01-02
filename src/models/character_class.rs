// src/models/character_class.rs

use diesel::prelude::*;
use crate::schema::character_classes;

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[diesel(table_name = character_classes)]
pub struct CharacterClass {
    pub id: i32,
    pub name: String,
    pub logo_url: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = character_classes)]
pub struct NewCharacterClass<'a> {
    pub name: &'a str,
    pub logo_url: &'a str,
}
