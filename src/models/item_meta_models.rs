use diesel::prelude::*;
use crate::schema::*;
use diesel::{ Identifiable, Insertable };

// ImageUrls Model
#[derive(Debug, Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = image_urls)]
pub struct ImageUrls {
    pub id: i32,
    pub icon: String,
    pub sd: String,
    pub hq: Option<String>,
    pub hd: Option<String>,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = image_urls)]
pub struct NewImageUrls<'a> {
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

#[derive(Insertable, Debug)]
#[diesel(table_name = item_categories)]
pub struct NewItemCategory<'a> {
    pub id: i32,

    pub name: &'a str,
}

// ItemType Model
#[derive(Debug, Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = item_types)]
pub struct ItemType {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = item_types)]
pub struct NewItemType<'a> {
    pub id: i32,
    pub name: &'a str,
}

#[derive(Debug, Queryable, Identifiable, serde::Deserialize)]
#[diesel(table_name = ranges)]
pub struct Range {
    pub id: i32,
    pub min: i32,
    pub max: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = ranges)]
pub struct NewRange {
    pub min: i32,
    pub max: i32,
}