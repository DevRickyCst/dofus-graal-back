// Relative path: src/models/mod.rs
pub mod statics;
use crate::schema::*;
use diesel::prelude::*;

use statics::dofus_meta::insertable::*;

// SetCaracteristique Model
#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = caracteristiques)]
pub struct Caracteristique {
    pub id: i32,
    pub vitalite: i32,
    pub sagesse: i32,
    pub agilite: i32,
    pub intelligence: i32,
    pub chance: i32,
    pub force: i32,
}

// SetStuff Model
#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = stuffs)]
pub struct Stuff {
    pub id: i32,
    pub chapeau_id: Option<i32>,
    pub collier_id: Option<i32>,
    pub anneau_1_id: Option<i32>,
    pub anneau_2_id: Option<i32>,
    pub ceinture_id: Option<i32>,
    pub botte_id: Option<i32>,
    pub bouclier_id: Option<i32>,
    pub arme_id: Option<i32>,
}

#[derive(Debug, Queryable, Associations)]
#[diesel(table_name = sets)]
#[diesel(belongs_to(Character))]
#[diesel(belongs_to(Caracteristique))]
#[diesel(belongs_to(Stuff))]
pub struct Set {
    pub id: i32,
    pub character_id: i32,
    pub caracteristique_id: i32,
    pub stuff_id: i32,
}
