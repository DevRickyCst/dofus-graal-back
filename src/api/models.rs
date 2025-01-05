use serde::Deserialize;
use diesel::PgConnection;
use crate::schema::effect_singles::dsl::*;
use diesel::QueryResult;
use crate::models::item_model::*;
use crate::models::dofus_models::*;
use crate::models::item_meta_models::*;
use diesel::RunQueryDsl;


#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub _links: Links,
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    pub first: String,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: String,
}

impl Links {
    pub fn is_first(&self) -> bool {
        self.prev.is_none()
    }

    pub fn is_last(&self) -> bool {
        self.next.is_none()
    }
}


#[derive(Debug, Deserialize)]
pub struct Item {
    pub ankama_id: i32,
    pub description: Option<String>,
    pub effects: Option<Vec<Effect>>,
    pub image_urls: Option<ImageUrls>,
    pub level: i16,
    pub name: String,
    pub recipes: Option<Vec<Recipe>>,
    pub item_type: Option<ItemType>
}


#[derive(Debug, Deserialize)]
pub struct DetailedItem {
    pub ankama_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub item_type: Option<ItemType>,
    pub is_weapon: Option<bool>,
    pub level: i16,
    pub pods: Option<i32>,
    pub image_urls: Option<ImageUrls>,
    pub effects: Option<Vec<Effect>>,
    pub critical_hit_probability: Option<i32>,
    pub critical_hit_bonus: Option<i32>,
    pub max_cast_per_turn: Option<i32>,
    pub ap_cost: Option<i32>,
    pub range: Option<Range>,
    pub recipe: Option<Vec<Recipe>>,
}

