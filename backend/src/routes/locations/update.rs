use std::collections::HashMap;
use mongodb::bson::Uuid;
use rocket::{error, patch, serde::{json::Json, Deserialize}};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, location::Location}};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateLocationData {
    name: Option<String>,
}

#[allow(unused)]
#[patch("/tenants/<tenant_id>/locations/<location_id>", format = "json", data = "<data>")] 
pub async fn update_location(db: Connection<ShelfWatcherDatabase>, tenant_id: &str, location_id: &str, data: Json<UpdateLocationData>) -> Json<HttpResponse<Location>> { 
    let data = data.into_inner();

    let tenant_uuid = match Uuid::parse_str(tenant_id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid tenant UUID: {:?}", err),
            data: None
        })
    };

    let location_uuid = match Uuid::parse_str(location_id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid location UUID: {:?}", err),
            data: None
        })
    };

    let old_location = match Location::get_by_id(location_uuid, &db).await {
        Ok(location) => location,
        Err(err) => return Json(err)
    };

    let mut new_location = old_location.clone();

    let mut old_values: HashMap<String, String> = HashMap::new();
    let mut new_values: HashMap<String, String> = HashMap::new();

    if data.name.is_some() {
        new_location.name = data.name.unwrap();
        old_values.insert("name".to_owned(), old_location.name.clone());
        new_values.insert("name".to_owned(), new_location.name.clone());
    }

    if new_values.is_empty() {
        return Json(HttpResponse {
            status: 200,
            message: "No updates applied.".to_string(),
            data: Some(new_location)
        });
    }

    match new_location.update(&db).await {
        Ok(location) => {
            // TODO: Implement author_id
            match AuditLog::new(location.id, AuditLogEntityType::Location, AuditLogAction::Update, "Location updated.".to_string(), Uuid::new(), Some(old_values), Some(new_values)).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }
            
            Json(HttpResponse {
                status: 200,
                message: "Location updated".to_string(),
                data: Some(new_location)
            })
        },
        Err(err) => Json(err)
    }
}