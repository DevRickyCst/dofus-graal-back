// Relative path: src/db_operations/crud.rs
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::LoadQuery;
use diesel::Insertable;

pub fn insert_and_retrieve_record<'a, Model, NewModel, Table, Values>(
    new_model: NewModel,
    table: Table,
    connection: &'a mut PgConnection,
) -> Result<Model, diesel::result::Error>
where
    NewModel: Insertable<Table, Values = Values>,
    InsertStatement<Table, Values>: LoadQuery<'a, PgConnection, Model>,
    Table: diesel::Table,
{
    diesel::insert_into(table)
        .values(new_model)
        .get_result::<Model>(connection)
}

#[cfg(test)]
mod insert_and_retrieve_record_test {
    use super::*;
    use crate::models::statics::item_meta::insertable::*;
    use crate::models::statics::item_meta::queryable::{
        Effect, ImageUrls, ItemType, Range, Recipe,
    };
    use crate::scripts::delete_items::*;
    use crate::test_utils::{get_test_db_connection, setup_test_environment};

    #[test]
    fn item_type() {
        use crate::schema::item_types::dsl::*;

        setup_test_environment();
        let mut conn = get_test_db_connection();

        let _ = delete_items(&mut conn);

        let new_record = NewItemType {
            id: 5,
            name: "Test Type".to_string(),
        };
        let record: ItemType = insert_and_retrieve_record(new_record, item_types, &mut conn)
            .expect("Erreur lors de l'insertion des types d'éléments");
        println!("{:#?}", record);
        assert_eq!(record.id, 5);
        assert_eq!(record.name, "Test Type");
    }

    #[test]
    fn image_urls() {
        use crate::schema::image_urls::dsl::*;

        setup_test_environment();
        let mut conn = get_test_db_connection();
        let _ = delete_items(&mut conn);

        let new_image_urls = NewImageUrls {
            icon: "testing".to_string(),
            sd: "testing".to_string(),
            hq: None,
            hd: None,
        };

        let record: ImageUrls = insert_and_retrieve_record(new_image_urls, image_urls, &mut conn)
            .expect("Erreur lors de l'insertion des images");

        assert_eq!(record.icon, "testing");
        assert_eq!(record.sd, "testing");
        assert_eq!(record.hq, None);
        assert_eq!(record.hd, None);
    }

    #[test]
    fn range() {
        use crate::schema::ranges::dsl::*;

        setup_test_environment();
        let mut conn = get_test_db_connection();
        let _ = delete_items(&mut conn);

        let new_range = NewRange { min: 0, max: 5 };
        let record: Range = insert_and_retrieve_record(new_range, ranges, &mut conn)
            .expect("Erreur lors de l'insertion des ranges");

        assert_eq!(record.min, 0);
        assert_eq!(record.max, 5);
    }

    #[test]
    fn test_recipe() {
        use crate::schema::recipes::dsl::*;

        setup_test_environment();
        let mut conn = get_test_db_connection();
        let _ = delete_items(&mut conn);

        let new_recipe = NewRecipe {
            item_id: None,
            item_ankama_id: 1,
            item_subtype: "Ingredient".to_string(),
            quantity: 10,
        };

        let record: Recipe = insert_and_retrieve_record(new_recipe, recipes, &mut conn)
            .expect("Erreur lors de l'insertion des recettes");

        assert_eq!(record.item_ankama_id, 1);
        assert_eq!(record.item_subtype, "Ingredient");
        assert_eq!(record.quantity, 10);
    }

    #[test]
    fn test_effect() {
        use crate::schema::effects::dsl::*;

        setup_test_environment();
        let mut conn = get_test_db_connection();
        let _ = delete_items(&mut conn);

        let new_effect = NewEffect {
            item_id: None,
            int_minimum: 5,
            int_maximum: 10,
            element_id: Some(1),
            ignore_int_min: false,
            ignore_int_max: true,
            formatted: "Test Effect".to_string(),
        };

        let record: Effect = insert_and_retrieve_record(new_effect, effects, &mut conn)
            .expect("Erreur lors de l'insertion des effets");

        assert_eq!(record.int_minimum, 5);
        assert_eq!(record.int_maximum, 10);
        assert_eq!(record.element_id, Some(1));
        assert_eq!(record.ignore_int_min, false);
        assert_eq!(record.ignore_int_max, true);
        assert_eq!(record.formatted, "Test Effect");
    }
}
