// Relative path: src/api/models.rs
use crate::models::statics::item_meta::insertable::*;

#[derive(serde::Deserialize)]
pub struct ApiResponse {
    pub _links: Links,
    pub items: Vec<DFDItem>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Links {
    //pub first: String,
    //pub prev: Option<String>,
    pub next: Option<String>,
    //pub last: String,
}

impl Links {
    /*pub fn is_first(&self) -> bool {
        self.prev.is_none()
    }*/

    pub fn is_last(&self) -> bool {
        self.next.is_none()
    }
}

#[derive(serde::Deserialize)]
pub struct DFDItem {
    pub ankama_id: i32,
    //pub effects: Option<Vec<DFDEffect>>,
    //pub image_urls: Option<DFDImageUrls>,
    //pub level: i16,
    //pub name: String,
    //pub recipes: Option<Vec<DFDRecipe>>,
    //#[serde(rename = "type")]
    //pub item_type: Option<DFDItemType>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DFDDetailedItem {
    pub ankama_id: i32,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub item_type: NewItemType,
    pub item_category: Option<NewItemCategory>,
    pub is_weapon: Option<bool>,
    pub is_two_handed: Option<bool>,
    pub level: i32,
    pub pods: Option<i32>,
    pub image_urls: NewImageUrls,
    pub effects: Option<Vec<NewEffect>>,
    pub critical_hit_probability: Option<i32>,
    pub critical_hit_bonus: Option<i32>,
    pub max_cast_per_turn: Option<i32>,
    pub ap_cost: Option<i32>,
    pub range: Option<NewRange>,
    pub recipes: Option<Vec<NewRecipe>>,
}
