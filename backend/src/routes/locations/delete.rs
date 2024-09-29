use mongodb::bson::Uuid;
use rocket::{delete, error, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, location::Location, tenant::Tenant}};

#[allow(unused)]
#[delete("/tenants/<tenant_id>/locations/<location_id>", format = "json")] 
pub async fn delete_location(db: Connection<ShelfWatcherDatabase>, tenant_id: &str, location_id: &str) -> Json<HttpResponse<()>> {
    let tenant_uuid = match Uuid::parse_str(tenant_id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid tenant UUID: {:?}", err),
            data: None
        })
    };

    let tenant = match Tenant::get_by_id(tenant_uuid, &db).await {
        Ok(tenant) => tenant,
        Err(err) => return Json(HttpResponse {
            status: err.status,
            message: err.message,
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

    let location = match Location::get_by_id(location_uuid, &db).await {
        Ok(location) => location,
        Err(err) => return Json(HttpResponse {
            status: err.status,
            message: err.message,
            data: None
        })
    };

    match location.delete(&db).await {
        Ok(location) => {
            // TODO: Implement author_id
            match AuditLog::new(location.id, AuditLogEntityType::Location, AuditLogAction::Delete, "Location deleted.".to_string(), Uuid::new(), None, None).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }

            Json(HttpResponse {
                status: 200,
                message: "Location deleted".to_string(),
                data: None,
            })
    },
        Err(err) => Json(err)
    }
}