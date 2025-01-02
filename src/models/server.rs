// src/models/server.rs

use diesel::prelude::*;
use crate::schema::servers;

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[diesel(table_name = servers)]
pub struct Server {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = servers)]
pub struct NewServer<'a> {
    pub name: &'a str,
}
