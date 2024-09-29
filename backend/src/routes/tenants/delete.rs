use mongodb::bson::Uuid;
use rocket::{delete, error, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, tenant::Tenant}};

#[allow(unused)]
#[delete("/tenants/<id>", format = "json")] 
pub async fn delete_tenant(db: Connection<ShelfWatcherDatabase>, id: &str) -> Json<HttpResponse<()>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err),
            data: None
        })
    };

    let tenant = match Tenant::get_by_id(uuid, &db).await {
        Ok(tenant) => tenant,
        Err(err) => return Json(HttpResponse {
            status: err.status,
            message: err.message,
            data: None
        })
    };


    match tenant.delete(&db).await {
        Ok(tenant) => {
            // TODO: Implement author_id
            match AuditLog::new(tenant.id, AuditLogEntityType::Tenant, AuditLogAction::Delete, "Tenant deleted.".to_string(), Uuid::new(), None, None).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }

            Json(HttpResponse {
                status: 200,
                message: "Tenant deleted".to_string(),
                data: None,
            })
    },
        Err(err) => Json(err)
    }
}