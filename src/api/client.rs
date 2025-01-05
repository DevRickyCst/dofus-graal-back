use reqwest::Error;
use super::{ApiParams, ApiResponse, DetailedItem};
use crate::api::params::Item_categories;
use crate::api::models::Links;
use diesel::pg::PgConnection;

const BASE_URL: &str = "https://api.dofusdu.de/dofus3/v1/fr";



pub async fn fetch_all_items_id() -> Result<Vec<i32>, Error>  {
    let mut all_ids = Vec::new();

    for category in [
        Item_categories::Consumables,
        Item_categories::Cosmetics,
        //Item_categories.Equipment,
        //Item_categories.Mounts,
        //Item_categories.Quest,
    ]{
        let list_id = fetch_items_category_id(category).await?;
        all_ids.extend(list_id);
    }
    Ok(all_ids)
}


pub async fn fetch_items_category_id(item_category: Item_categories) -> Result<Vec<i32>, Error> {

    let mut all_ids = Vec::new();

    let mut current_page = 1;
    println!("Fetch {} category", item_category);
    loop {
        let api_params = ApiParams {
            page_number: current_page,
            page_size: 100,
            item_category: item_category.clone(),
        };

        let (list_id, links) = fetch_items_id(api_params).await?;

        all_ids.extend(list_id);

        if links.is_last() {
            break;
        }

        current_page += 1;
    }

    Ok(all_ids)
}

async fn fetch_items_id(params: ApiParams) -> Result<(Vec<i32>, Links), Error> {

    let extra_url = params.to_extra_url();

    let url = format!("{}{}", BASE_URL, extra_url);

    let response = reqwest::get(url).await?.json::<ApiResponse>().await?;

    let item_ids: Vec<i32> = response.items.into_iter().map(|item| item.ankama_id).collect();

    Ok((item_ids, response._links))
}

pub async fn fetch_single_item(
    ankama_id: i32,
    item_category: Item_categories,
) -> Result<DetailedItem, Error> {

    let extra_url = format!("/items/{}/{}", item_category, ankama_id);
    let url = format!("{}{}", BASE_URL, extra_url);

    let response = reqwest::get(&url).await?.json::<DetailedItem>().await?;

    Ok(response)
}