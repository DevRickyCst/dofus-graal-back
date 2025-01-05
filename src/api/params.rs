use std::fmt;

#[derive(Debug, Clone)]
pub enum Item_categories{
    Consumables,
    Cosmetics,
    Equipment,
    Mounts,
    Quest,
}


// Implement Display for Item_categories
impl fmt::Display for Item_categories {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let category_str = match self {
            Item_categories::Consumables => "consumables",
            Item_categories::Cosmetics => "cosmetics",
            Item_categories::Equipment => "equipment",
            Item_categories::Mounts => "mounts",
            Item_categories::Quest => "quest",
        };
        write!(f, "{}", category_str)
    }
}
#[derive(Debug)]
pub struct ApiParams {
    pub page_number: i32,
    pub page_size: i32,
    pub item_category: Item_categories,
}

impl ApiParams {
    pub fn to_extra_url(&self) -> String {
        let params = [format!("page[size]={}", self.page_size),
            format!("page[number]={}", self.page_number)];

        // If mounts, dont add item/
        match self.item_category {
            Item_categories::Mounts => {
                format!("/mounts?{}", params.join("&"))
            }
            _ => {
                format!("/items/{}?{}", self.item_category, params.join("&"))
            }
        }
    }
}
