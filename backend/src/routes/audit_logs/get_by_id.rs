use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{audit_log::{AuditLog, AuditLogEntityType}, http_response::HttpResponse}};

#[allow(unused)]
#[get("/audit-logs/<type>/id/<id>", format = "json")] 
pub async fn get_audit_log_by_id(db: Connection<ShelfWatcherDatabase>, r#type: &str, id: &str) -> Json<HttpResponse<AuditLog>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err),
            data: None
        })
    };

    let entity_type = match AuditLogEntityType::from_string(&r#type) {
        Ok(entity_type) => entity_type,
        Err(err) => return Json(err)
    };

    match AuditLog::get_by_id(uuid, entity_type, &db).await {
        Ok(audit_log) => Json(HttpResponse {
            status: 200,
            message: "Audit Log found by id".to_string(),
            data: Some(audit_log),
        }),
        Err(err) => Json(err)
    }
}