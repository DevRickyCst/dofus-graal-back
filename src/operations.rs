
use diesel::prelude::*;
use diesel::dsl::delete;
use diesel::PgConnection;
use diesel::result::Error;


pub async fn sync_items(conn: &mut PgConnection) -> Result<(), Error> {


    Ok(())
}


pub fn delete_table(table: &str, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
    println!("Suppression des données dans la table : {}", table);

    match table {
        "item_types" => delete(crate::schema::item_types::table).execute(conn),
        "characters" => delete(crate::schema::characters::table).execute(conn),
        "effect_singles" => delete(crate::schema::effect_singles::table).execute(conn),
        "servers" => delete(crate::schema::servers::table).execute(conn),
        "ranges" => delete(crate::schema::ranges::table).execute(conn),
        "items" => delete(crate::schema::items::table).execute(conn),
        "item_effects" => delete(crate::schema::item_effects::table).execute(conn),
        _ => {
            println!("Table inconnue : {}", table);
            Err(diesel::result::Error::NotFound)
        }
    }
}

