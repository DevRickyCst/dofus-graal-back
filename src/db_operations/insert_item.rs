use diesel::prelude::*;
use diesel::PgConnection;
use diesel::result::Error;

use crate::models::NewImageUrl

use crate::api::models::DetailedItem;

pub fn save_item(item: DetailedItem) -> Result<(), Error> {
    use crate::schema::image_urls::dsl::*;
    use crate::schema::items::dsl::*;
    // Sauvegarde des URLs d'images
    if let Some(ref _image_urls) = self.image_urls {
        let new_image_urls = NewImageUrl {
            icon: &_image_urls.icon,
            sd: &_image_urls.sd,
            hq: _image_urls.hq.as_deref(),
            hd: _image_urls.hd.as_deref(),
        };
    }
}