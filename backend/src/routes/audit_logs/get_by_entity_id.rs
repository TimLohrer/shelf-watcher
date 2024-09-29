use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{audit_log::{AuditLog, AuditLogEntityType}, http_response::HttpResponse}};

#[allow(unused)]
#[get("/audit-logs/<type>/entity/<id>", format = "json")] 
pub async fn get_audit_log_by_entity_id(db: Connection<ShelfWatcherDatabase>, r#type: &str, id: &str) -> Json<HttpResponse<Vec<AuditLog>>> {
    let entity_uuid = match Uuid::parse_str(id) {
        Ok(entity_uuid) => entity_uuid,
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

    match AuditLog::get_by_entity_id(entity_uuid, entity_type, &db).await {
        Ok(audit_log) => Json(HttpResponse {
            status: 200,
            message: "Audit Logs found by entity id".to_string(),
            data: Some(audit_log),
        }),
        Err(err) => Json(err)
    }
}