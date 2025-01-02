// @generated automatically by Diesel CLI.

diesel::table! {
    character_classes (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        logo_url -> Varchar,
    }
}

diesel::table! {
    characters (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        level -> Int4,
        server_id -> Nullable<Int4>,
        character_class_id -> Nullable<Int4>,
        user_id -> Int4,
    }
}

diesel::table! {
    effect_singles (id) {
        id -> Int4,
        int_minimum -> Int4,
        int_maximum -> Int4,
        element_id -> Int4,
        ignore_int_min -> Bool,
        ignore_int_max -> Bool,
        #[max_length = 200]
        formatted -> Varchar,
    }
}

diesel::table! {
    elements (id) {
        id -> Int4,
        #[max_length = 200]
        name -> Varchar,
    }
}

diesel::table! {
    image_urls (id) {
        id -> Int4,
        #[max_length = 200]
        icon -> Varchar,
        #[max_length = 200]
        sd -> Varchar,
        #[max_length = 200]
        hq -> Varchar,
        #[max_length = 200]
        hd -> Varchar,
    }
}

diesel::table! {
    item_categories (id) {
        id -> Int4,
        #[max_length = 20]
        name -> Varchar,
    }
}

diesel::table! {
    item_effects (item_id, effect_id) {
        item_id -> Int4,
        effect_id -> Int4,
    }
}

diesel::table! {
    item_recipes (item_id, recipe_id) {
        item_id -> Int4,
        recipe_id -> Int4,
    }
}

diesel::table! {
    item_types (id) {
        id -> Int4,
        #[max_length = 20]
        name -> Varchar,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        category_id -> Int4,
        type_id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 1000]
        description -> Varchar,
        level -> Int4,
        pods -> Nullable<Int4>,
        image_urls_id -> Int4,
        ap_cost -> Nullable<Int4>,
        max_cast_per_turn -> Nullable<Int4>,
        is_weapon -> Bool,
        is_two_handed -> Nullable<Bool>,
        critical_hit_probability -> Nullable<Int4>,
        critical_hit_bonus -> Nullable<Int4>,
    }
}

diesel::table! {
    ranges (item_id) {
        item_id -> Int4,
        min -> Int4,
        max -> Int4,
    }
}

diesel::table! {
    recipe_singles (id) {
        id -> Int4,
        item_ankama_id -> Int4,
        #[max_length = 200]
        item_subtype -> Varchar,
        quantity -> Int4,
    }
}

diesel::table! {
    servers (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
    }
}

diesel::table! {
    set_caracteristiques (id) {
        id -> Int4,
        vitalite -> Int4,
        sagesse -> Int4,
        agilite -> Int4,
        intelligence -> Int4,
        chance -> Int4,
        force -> Int4,
    }
}

diesel::table! {
    set_stuffs (id) {
        id -> Int4,
        chapeau_id -> Nullable<Int4>,
        collier_id -> Nullable<Int4>,
        anneau_1_id -> Nullable<Int4>,
        anneau_2_id -> Nullable<Int4>,
        ceinture_id -> Nullable<Int4>,
        botte_id -> Nullable<Int4>,
        bouclier_id -> Nullable<Int4>,
        arme_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sets (id) {
        id -> Int4,
        character_id -> Int4,
        caracteristique_id -> Nullable<Int4>,
        stuff_id -> Nullable<Int4>,
    }
}

diesel::joinable!(characters -> character_classes (character_class_id));
diesel::joinable!(characters -> servers (server_id));
diesel::joinable!(effect_singles -> elements (element_id));
diesel::joinable!(item_effects -> effect_singles (effect_id));
diesel::joinable!(item_effects -> items (item_id));
diesel::joinable!(item_recipes -> items (item_id));
diesel::joinable!(item_recipes -> recipe_singles (recipe_id));
diesel::joinable!(items -> image_urls (image_urls_id));
diesel::joinable!(items -> item_categories (category_id));
diesel::joinable!(items -> item_types (type_id));
diesel::joinable!(ranges -> items (item_id));
diesel::joinable!(sets -> characters (character_id));
diesel::joinable!(sets -> set_caracteristiques (caracteristique_id));
diesel::joinable!(sets -> set_stuffs (stuff_id));

diesel::allow_tables_to_appear_in_same_query!(
    character_classes,
    characters,
    effect_singles,
    elements,
    image_urls,
    item_categories,
    item_effects,
    item_recipes,
    item_types,
    items,
    ranges,
    recipe_singles,
    servers,
    set_caracteristiques,
    set_stuffs,
    sets,
);
