// src/models/image_urls.rs

use diesel::prelude::*;
use crate::schema::image_urls;

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[diesel(table_name = image_urls)]
pub struct ImageUrls {
    pub id: i32,
    pub icon: String,
    pub sd: String,
    pub hq: String,
    pub hd: String,
}

#[derive(Insertable)]
#[diesel(table_name = image_urls)]
pub struct NewImageUrls<'a> {
    pub icon: &'a str,
    pub sd: &'a str,
    pub hq: &'a str,
    pub hd: &'a str,
}
