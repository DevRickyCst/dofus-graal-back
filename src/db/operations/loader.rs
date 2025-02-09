use crate::{
    models::statics::{
        item::queryable::Item,
        item_meta::queryable::{Effect, ImageUrls, ItemCategory, ItemType, Range, Recipe},
    },
    schema::{effects, image_urls, item_categories, item_types, items, ranges, recipes},
};
use crate::api::models::DFDDetailedItem;

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::models::statics::item_meta::insertable::{
    NewEffect, NewImageUrls, NewItemCategory, NewItemType, NewRange, NewRecipe,
};

pub fn load_items_with_meta(
    conn: &mut PgConnection,
    limit: i32,
    offset: i32,
) -> Vec<(Item, ItemType, ItemCategory, ImageUrls, Option<Range>)> {
    items::table
        .inner_join(item_types::table)
        .inner_join(item_categories::table)
        .inner_join(image_urls::table)
        .left_join(ranges::table)
        .select((
            Item::as_select(),
            ItemType::as_select(),
            ItemCategory::as_select(),
            ImageUrls::as_select(),
            Option::<Range>::as_select(),
        ))
        .limit(limit.into())
        .offset(offset.into())
        .load::<(Item, ItemType, ItemCategory, ImageUrls, Option<Range>)>(conn)
        .expect("Erreur lors de la récupération des données")
}

pub fn load_effects(conn: &mut PgConnection, item_ids: &[i32]) -> Vec<Effect> {
    effects::table
        .filter(effects::item_id.eq_any(item_ids))
        .load::<Effect>(conn)
        .expect("Erreur lors de la récupération des effets")
}

pub fn load_recipes(conn: &mut PgConnection, item_ids: &[i32]) -> Vec<Recipe> {
    recipes::table
        .filter(recipes::item_id.eq_any(item_ids))
        .load::<Recipe>(conn)
        .expect("Erreur lors de la récupération des recettes")
}




fn map_items(
    results: Vec<(Item, ItemType, ItemCategory, ImageUrls, Option<Range>)>,
    effects: Vec<Effect>,
    recipes: Vec<Recipe>,
) -> Vec<DFDDetailedItem> {
    results
        .into_iter()
        .map(|(item, item_type, item_category, image_urls, range)| {
            let item_effects: Vec<NewEffect> = effects
                .iter()
                .filter(|e| e.item_id == item.ankama_id)
                .cloned()
                .map(NewEffect::from)
                .collect();
            let item_recipes: Vec<NewRecipe> = recipes
                .iter()
                .filter(|r| r.item_id == item.ankama_id)
                .cloned()
                .map(NewRecipe::from)
                .collect();

            DFDDetailedItem {
                ankama_id: item.ankama_id,
                name: item.name,
                description: item.description,
                item_type: NewItemType::from(item_type),
                is_weapon: Some(item.is_weapon),
                item_category: Some(NewItemCategory::from(item_category)),
                image_urls: NewImageUrls::from(image_urls),
                range: range.map(NewRange::from),
                effects: Some(item_effects),
                recipes: Some(item_recipes),
                is_two_handed: item.is_two_handed,
                level: item.level,
                pods: item.pods,
                critical_hit_probability: item.critical_hit_probability,
                critical_hit_bonus: item.critical_hit_bonus,
                max_cast_per_turn: item.max_cast_per_turn,
                ap_cost: item.ap_cost,
            }
        })
        .collect()
}



pub fn load_items(conn: &mut PgConnection, limit: i32, offset: i32) -> Vec<DFDDetailedItem> {
    let items_with_meta = load_items_with_meta(conn, limit, offset);

    let item_ids: Vec<i32> = items_with_meta
        .iter()
        .map(|(item, _, _, _, _)| item.ankama_id)
        .collect();

    let effects = load_effects(conn, &item_ids);

    let recipes = load_recipes(conn, &item_ids);

    let mapped_items = map_items(items_with_meta, effects, recipes);

    mapped_items
}