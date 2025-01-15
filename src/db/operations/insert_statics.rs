// Relative path: src/db_operations/insert_statics.rs
/*
use crate::models::character_class::NewCharacterClass;
use crate::schema::servers::dsl::servers;
use crate::models::server::NewServer;
use crate::schema::dofus_classes::dsl::dofus_classes;
use crate::constant::{SERVERS, DOFUS_CLASSES}; // Import de la constante SERVERS


pub fn insert_statics(conn: &mut PgConnection) -> Result<(), Error> {
    let _ = insert_servers(conn);
    let _ = insert_class(conn);
    Ok(())
}




pub fn insert_servers(conn: &mut PgConnection) -> Result<(), Error> {
    println!("Insertion des serveurs dans la table 'servers'.");

    for (index, &(srv_name, srv_category)) in SERVERS.iter().enumerate() {
        println!(
            "Insertion n°{} => name: {}, category: {}",
            index + 1,
            srv_name,
            srv_category
        );

        let new_server = NewServer {
            name: srv_name,
            category: srv_category,
        };

        let rows_inserted = diesel::insert_into(servers)
        .values(&new_server)
        .on_conflict_do_nothing()
        .execute(conn)?;

        println!("Nombre de lignes insérées : {}", rows_inserted);

    }

    println!("Serveurs insérés avec succès.");
    Ok(())
}



pub fn insert_class(conn: &mut PgConnection) -> Result<(), Error> {
    // 2) Insertion des classes
    println!("Insertion des classes dans la table 'classes'.");

    for (index, &class_name) in DOFUS_CLASSES.iter().enumerate() {
        println!("Insertion n°{} => class: {}", index + 1, class_name);

        let new_class = NewCharacterClass {
            name: class_name,
        };

        let rows_inserted = diesel::insert_into(character_class)
            .values(&new_class)
            .on_conflict_do_nothing() // Ajoutez ou non, selon votre logique
            .execute(conn)?;

        println!("Nombre de lignes insérées (classes) : {}", rows_inserted);
    }

    println!("Classes insérées avec succès.");
    Ok(())
}*/
