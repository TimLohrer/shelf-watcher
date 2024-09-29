use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{http_response::HttpResponse, location::Location}};

#[allow(unused)]
#[get("/tenants/<tenant_id>/locations/<location_id>", format = "json")] 
pub async fn get_location_by_id(db: Connection<ShelfWatcherDatabase>, tenant_id: &str, location_id: &str) -> Json<HttpResponse<Location>> {
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

    match Location::get_by_id(location_uuid, &db).await {
        Ok(tenant) => Json(HttpResponse {
            status: 200,
            message: "Found location by id".to_string(),
            data: Some(tenant),
        }),
        Err(err) => Json(err)
    }
}