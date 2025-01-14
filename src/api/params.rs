// Relative path: src/api/params.rs

#[derive(Debug)]
pub struct ApiParams<'a> {
    pub page_number: i32,
    pub page_size: i32,
    pub item_category: &'a str,
}

impl<'a> ApiParams<'a> {
    pub fn to_extra_url(&self) -> String {
        let params = [format!("page[size]={}", self.page_size),
            format!("page[number]={}", self.page_number)];

        // If mounts, dont add item/
        if self.item_category == "Mounts" {
            format!("/mounts?{}", params.join("&"))
        } else {
            format!("/items/{}?{}", self.item_category, params.join("&"))
        }
    }
}
