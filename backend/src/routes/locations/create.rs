use mongodb::bson::Uuid;
use rocket::{error, post, serde::{json::Json, Deserialize}};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, location::Location, tenant::Tenant}};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateLocationData {
    name: String
}

#[allow(unused)]
#[post("/tenants/<tenant_id>/locations", format = "json", data = "<data>")] 
pub async fn create_location(db: Connection<ShelfWatcherDatabase>, data: Json<CreateLocationData>, tenant_id: &str) -> Json<HttpResponse<Location>> { 
    let data = data.into_inner();

    let tenant_uuid = match Uuid::parse_str(tenant_id) {
        Ok(uuid) => uuid,
        Err(_) => return Json(HttpResponse {
            status: 400,
            message: "Invalid tenant_id".to_string(),
            data: None
        })
        
    };

    if Tenant::get_by_id(tenant_uuid, &db).await.is_err() {
        return Json(HttpResponse {
            status: 400,
            message: "Tenant does not exists".to_string(),
            data: None
        });
    }

    let existing = match Location::get_all_from_tenant(tenant_uuid, &db).await {
        Ok(locations) => locations,
        Err(err) => return Json(HttpResponse {
            status: err.status,
            message: err.message,
            data: None
        })
    };

    if existing.len() >= 3 {
        return Json(HttpResponse {
            status: 400,
            message: "Tenant has reached the maximum number of locations (3)".to_string(),
            data: None
        });
    }

    if existing.iter().any(|location| location.name == data.name) {
        return Json(HttpResponse {
            status: 400,
            message: "Location already exists".to_string(),
            data: None
        });
    }

    let location = Location::new(data.name, tenant_uuid);
    
    match location.insert(&db).await {
        Ok(location) => {
        // TODO: Implement author_id
            match AuditLog::new(location.id, AuditLogEntityType::Location, AuditLogAction::Create, "Location created.".to_string(), Uuid::new(), None, None).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }
            
            Json(HttpResponse {
                status: 201,
                message: "Location created".to_string(),
                data: Some(location)
            })
        },
        Err(err) => Json(err)
    }
}