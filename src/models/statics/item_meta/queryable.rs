// Relative path: src/models/statics/item_meta/queryable.rs
use super::insertable::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::Identifiable;

#[derive(Queryable, Identifiable, serde::Deserialize, Debug, Selectable)]
#[diesel(table_name = image_urls)]
pub struct ImageUrls {
    pub id: i32,
    pub icon: String,
    pub sd: String,
    pub hq: Option<String>,
    pub hd: Option<String>,
}

#[derive(Queryable, Identifiable, serde::Deserialize, Debug, Selectable, Clone)]
#[diesel(table_name = item_categories)]
pub struct ItemCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, serde::Deserialize, Debug, Selectable, Clone)]
#[diesel(table_name = item_types)]
pub struct ItemType {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, serde::Deserialize, Debug, Selectable, Clone)]
#[diesel(table_name = ranges)]
pub struct Range {
    pub id: i32,
    pub min: i32,
    pub max: i32,
}

#[derive(Queryable, Identifiable, serde::Deserialize, Debug, Selectable, Clone)]
#[diesel(table_name = effects)]
pub struct Effect {
    pub id: i32,
    pub item_id: i32,
    pub int_minimum: i32,
    pub int_maximum: i32,
    pub element_id: Option<i32>,
    pub ignore_int_min: bool,
    pub ignore_int_max: bool,
    pub formatted: String,
}

#[derive(Queryable, Identifiable, serde::Deserialize, Debug, Selectable, Clone)]
#[diesel(table_name = recipes)]
pub struct Recipe {
    pub id: i32,
    pub item_id: i32,
    pub item_ankama_id: i32,
    pub item_subtype: String,
    pub quantity: i32,
}

impl From<ImageUrls> for NewImageUrls {
    fn from(image_urls: ImageUrls) -> Self {
        NewImageUrls {
            icon: image_urls.icon,
            sd: image_urls.sd,
            hq: image_urls.hq,
            hd: image_urls.hd,
        }
    }
}

impl From<ItemType> for NewItemType {
    fn from(item_type: ItemType) -> Self {
        NewItemType {
            id: item_type.id,
            name: item_type.name,
        }
    }
}

impl From<Range> for NewRange {
    fn from(range: Range) -> Self {
        NewRange {
            min: range.min,
            max: range.max,
        }
    }
}

impl From<ItemCategory> for NewItemCategory {
    fn from(item_category: ItemCategory) -> Self {
        NewItemCategory {
            id: item_category.id,
            name: item_category.name,
        }
    }
}

impl From<Recipe> for NewRecipe {
    fn from(recipe: Recipe) -> Self {
        NewRecipe {
            item_id: Some(recipe.item_id),
            item_ankama_id: recipe.item_ankama_id,
            item_subtype: recipe.item_subtype,
            quantity: recipe.quantity,
        }
    }
}

impl From<Effect> for NewEffect {
    fn from(effect: Effect) -> Self {
        NewEffect {
            item_id: Some(effect.item_id),
            int_minimum: effect.int_minimum,
            int_maximum: effect.int_maximum,
            element_id: effect.element_id,
            ignore_int_min: effect.ignore_int_min,
            ignore_int_max: effect.ignore_int_max,
            formatted: effect.formatted,
        }
    }
}
