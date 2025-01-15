// Relative path: src/constant.rs
use crate::models::statics::item_meta::insertable::NewItemCategory;

pub const ITEM_CATEGORIES: &[NewItemCategory] = &[
    NewItemCategory {
        id: 1,
        name: "consumables",
    },
    NewItemCategory {
        id: 2,
        name: "cosmetics",
    },
    NewItemCategory {
        id: 3,
        name: "equipment",
    },
    NewItemCategory {
        id: 4,
        name: "quest",
    },
];

pub const SERVERS: &[(&str, &str)] = &[
    ("brial 1", "pionnier"),
    ("brial 2", "pionnier"),
    ("kourial 1", "pionnier"),
    ("kourial 2", "pionnier"),
    ("mikhal 1", "pionnier"),
    ("mikhal 2", "pionnier"),
    ("rafal 1", "pionnier"),
    ("rafal 2", "pionnier"),
    ("salar 1", "pionnier"),
    ("salar 2", "pionnier"),
    ("dakal 1", "pionnier monocompte"),
    ("dakal 2", "pionnier monocompte"),
    ("dakal 3", "pionnier monocompte"),
    ("dakal 4", "pionnier monocompte"),
    ("dakal 5", "pionnier monocompte"),
    ("dakal 6", "pionnier monocompte"),
    ("dakal 7", "pionnier monocompte"),
    ("dakal 8", "pionnier monocompte"),
    ("dakal 9", "pionnier monocompte"),
    ("dakal 10", "pionnier monocompte"),
    ("dakal 11", "pionnier monocompte"),
    ("dakal 12", "pionnier monocompte"),
    ("tal kasha", "classique"),
    ("orukam", "classique"),
    ("imagiro", "classique"),
    ("hell mina", "classique"),
    ("tilezia", "classique"),
    ("draconiros", "classique monocompte"),
    ("ombre", "epique"),
];

pub const DOFUS_CLASSES: &[&str] = &[
    "feca",
    "osamodas",
    "enutrof",
    "sram",
    "xelor",
    "ecaflip",
    "eniripsa",
    "iop",
    "cra",
    "sadida",
    "sacrieur",
    "pandawa",
    "roublard",
    "zobal",
    "foggernaut",
    "eliotrope",
    "huppermage",
    "ouginak",
];
