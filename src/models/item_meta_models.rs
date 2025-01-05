use diesel::prelude::*;
use crate::schema::*;
use diesel::{ Identifiable, Insertable };

// ImageUrls Model
#[derive(Debug, Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = image_urls)]
pub struct ImageUrls {
    pub id: Option<i32>,
    pub icon: String,
    pub sd: String,
    pub hq: Option<String>,
    pub hd: Option<String>,
}


#[derive(Insertable)]
#[diesel(table_name = image_urls)]
pub struct NewImageUrl<'a> {
    pub icon: &'a str,
    pub sd: &'a str,
    pub hq: Option<&'a str>,
    pub hd: Option<&'a str>,
}


// ItemCategory Model
#[derive(Debug, Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = item_categories)]
pub struct ItemCategory {
    pub id: i32,
    pub name: String,
}

// ItemType Model
#[derive(Debug, Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = item_types)]
pub struct ItemType {
    pub id: i32,
    pub name: String,
}