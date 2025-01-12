use serde::Deserialize;
use crate::models::dofus_models::*;
use crate::models::item_meta_models::*;


#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub _links: Links,
    pub items: Vec<DFDItem>,
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
pub struct DFDImageUrls {
    pub icon: String,
    pub sd: String,
    pub hq: Option<String>,
    pub hd: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DFDItemType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct DFDRange {
    pub min: i32,
    pub max: i32,
}

#[derive(Debug, Deserialize)]
pub struct DFDItem {
    pub ankama_id: i32,
    pub effects: Option<Vec<DFDEffect>>,
    pub image_urls: Option<DFDImageUrls>,
    pub level: i16,
    pub name: String,
    pub recipes: Option<Vec<DFDRecipe>>,
    #[serde(rename = "type")]
    pub item_type: Option<DFDItemType>
}

#[derive(Debug, Deserialize)]
pub struct DFDEffect {
    pub int_minimum: i32,
    pub int_maximum: i32,
    pub element_id: Option<i32>,
    pub ignore_int_min: bool,
    pub ignore_int_max: bool,
    pub formatted: String,
}

#[derive(Debug, Deserialize)]
pub struct DFDRecipe {
    pub item_ankama_id: i32,
    pub item_subtype: String,
    pub quantity: i32,
}


#[derive(Debug, Deserialize)]
pub struct DFDDetailedItem {
    pub ankama_id: i32,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub item_type: DFDItemType,
    pub is_weapon: Option<bool>,
    pub is_two_handed: Option<bool>,
    pub level: i32,
    pub pods: Option<i32>,
    pub image_urls: Option<DFDImageUrls>,
    pub effects: Option<Vec<DFDEffect>>,
    pub critical_hit_probability: Option<i32>,
    pub critical_hit_bonus: Option<i32>,
    pub max_cast_per_turn: Option<i32>,
    pub ap_cost: Option<i32>,
    pub range: Option<DFDRange>,
    pub recipe: Option<Vec<DFDRecipe>>,
}


impl<'a> From<&'a DFDImageUrls> for NewImageUrls<'a> {
    fn from(df_image: &'a DFDImageUrls) -> Self {
        NewImageUrls {
            icon: &df_image.icon,
            sd: &df_image.sd,
            hq: df_image.hq.as_deref(),
            hd: df_image.hd.as_deref(),
        }
    }
}

impl<'a> From<&'a DFDItemType> for NewItemType<'a> {
    fn from(df_item_type: &'a DFDItemType) -> Self {
        NewItemType {
            id: df_item_type.id,
            name: &df_item_type.name,
        }
    }
}

impl<'a> From<&'a DFDRange> for NewRange {
    fn from(df_range: &'a DFDRange) -> Self {
        NewRange {
            min: df_range.min,
            max: df_range.max,
        }
    }
}

impl<'a> From<&'a DFDEffect> for NewEffect<'a> {
    fn from(df_effect: &'a DFDEffect) -> Self {
        NewEffect {
            int_minimum: df_effect.int_minimum,
            int_maximum: df_effect.int_maximum,
            element_id: df_effect.element_id.unwrap_or(0), // Valeur par défaut si None
            ignore_int_min: df_effect.ignore_int_min,
            ignore_int_max: df_effect.ignore_int_max,
            formatted: &df_effect.formatted,
        }
    }
}

impl<'a> From<&'a DFDRecipe> for NewRecipe<'a> {
    fn from(df_recipe: &'a DFDRecipe) -> Self {
        NewRecipe {
            item_ankama_id: df_recipe.item_ankama_id,
            item_subtype: &df_recipe.item_subtype,
            quantity: df_recipe.quantity,
        }
    }
}
