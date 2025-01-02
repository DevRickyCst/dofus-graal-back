use crate::models::item_category::{NewItemCategory, ApiCategory};
use crate::schema::item_categories;
use diesel::prelude::*;
use diesel::result::Error;
use reqwest::Client;
use serde::Deserialize;
use log::{info, error};


pub async fn fetch_and_insert_item_categories(pool: &crate::utils::db::PgPool) {
    // Créer un client HTTP
    let client = Client::new();
    
    // Effectuer la requête GET
    let response = client
        .get("https://api.dofusdu.de/dofus3/v1/meta/search/types")
        .header("Accept", "application/json")
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let categories: Vec<String> = match resp.json().await {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Erreur lors de la désérialisation de la réponse JSON : {}", e);
                        return;
                    }
                };

                // Transformer les catégories en NewItemCategory
                let new_categories: Vec<NewItemCategory> = categories.iter()
                    .map(|name| NewItemCategory { name: name.as_str() })
                    .collect();

                // Insérer dans la base de données
                let conn = match pool.get() {
                    Ok(conn) => conn,
                    Err(e) => {
                        error!("Erreur lors de l'obtention d'une connexion à la base de données : {}", e);
                        return;
                    }
                };

                // Utiliser une transaction pour garantir l'intégrité
                match conn.transaction::<_, Error, _>(|| {
                    for new_cat in &new_categories {
                        diesel::insert_into(item_categories::table)
                            .values(new_cat)
                            .on_conflict(item_categories::name)
                            .do_nothing()
                            .execute(conn)?;
                    }
                    Ok(())
                }) {
                    Ok(_) => info!("Catégories insérées avec succès."),
                    Err(e) => error!("Erreur lors de l'insertion des catégories : {}", e),
                }
            } else {
                error!("Requête échouée avec le statut : {}", resp.status());
            }
        },
        Err(e) => {
            error!("Erreur lors de la requête GET : {}", e);
        }
    }
}