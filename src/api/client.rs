use reqwest::Error;
use super::{ApiParams, ApiResponse, DFDDetailedItem};
use crate::api::models::Links;
const BASE_URL: &str = "https://api.dofusdu.de/dofus3/v1/fr";


pub async fn fetch_items_category_id(category_name: &str) -> Result<Vec<i32>, Error> {

    let mut all_ids = Vec::new();
    let mut current_page = 1;

    loop {
        let api_params = ApiParams {
            page_number: current_page,
            page_size: 100,
            item_category: category_name,
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

async fn fetch_items_id(params: ApiParams<'_>) -> Result<(Vec<i32>, Links), Error> {

    let extra_url = params.to_extra_url();

    let url = format!("{}{}", BASE_URL, extra_url);

    println!("{}", url);
    let response = reqwest::get(url).await?.json::<ApiResponse>().await?;

    let item_ids: Vec<i32> = response.items.into_iter().map(|item| item.ankama_id).collect();

    Ok((item_ids, response._links))
}

pub async fn fetch_single_item(
    ankama_id: i32,
    category_name: &str, 
) -> Result<DFDDetailedItem, Error> {

    let extra_url = format!("/items/{}/{}", category_name, ankama_id);
    let url = format!("{}{}", BASE_URL, extra_url);

    let response = reqwest::get(&url).await?.json::<DFDDetailedItem>().await?;

    Ok(response)
}