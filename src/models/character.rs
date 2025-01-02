// src/models/character.rs

use diesel::prelude::*;
use crate::schema::characters;
use crate::models::{Server, CharacterClass};

#[derive(Debug, Queryable, Identifiable, Associations, Insertable)]
#[diesel(table_name = characters)]
#[diesel(belongs_to(Server, foreign_key = server_id))]
#[diesel(belongs_to(CharacterClass, foreign_key = character_class_id))]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub server_id: Option<i32>,
    pub character_class_id: Option<i32>,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = characters)]
pub struct NewCharacter<'a> {
    pub name: &'a str,
    pub level: i32,
    pub server_id: Option<i32>,
    pub character_class_id: Option<i32>,
    pub user_id: i32,
}
