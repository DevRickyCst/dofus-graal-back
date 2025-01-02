// src/models/effect_single.rs

use diesel::prelude::*;
use crate::schema::effect_singles;
use crate::models::element::Element;

#[derive(Debug, Queryable, Identifiable, Associations, Insertable)]
#[diesel(table_name = effect_singles)]
#[diesel(belongs_to(Element, foreign_key = element_id))]
pub struct EffectSingle {
    pub id: i32,
    pub int_minimum: i32,
    pub int_maximum: i32,
    pub element_id: i32,
    pub ignore_int_min: bool,
    pub ignore_int_max: bool,
    pub formatted: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = effect_singles)]
pub struct NewEffectSingle<'a> {
    pub int_minimum: i32,
    pub int_maximum: i32,
    pub element_id: i32,
    pub ignore_int_min: bool,
    pub ignore_int_max: bool,
    pub formatted: &'a str,
}
