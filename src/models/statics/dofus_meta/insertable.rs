use diesel::prelude::*;
use crate::schema::*;


#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = servers)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub category: String,
}


#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = dofus_classes)]
pub struct DofusClass {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = characters)]
#[diesel(belongs_to(Server, foreign_key = server_id))]
#[diesel(belongs_to(DofusClass, foreign_key = dofus_classes_id))]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub server_id: Option<i32>,
    pub dofus_classes_id: Option<i32>,
    pub user_id: i32,
}