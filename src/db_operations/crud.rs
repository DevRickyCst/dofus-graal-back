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
    InsertStatement<Table, Values>:  LoadQuery<'a, PgConnection, Model>,
    Table: diesel::Table,
{
    diesel::insert_into(table)
        .values(new_model)
        .get_result::<Model>(connection)
}
