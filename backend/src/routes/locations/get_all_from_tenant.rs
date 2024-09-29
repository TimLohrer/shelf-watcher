use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{http_response::HttpResponse, location::Location}};

#[allow(unused)]
#[get("/tenants/<tenant_id>/locations", format = "json")] 
pub async fn get_all_locations_from_tenant(db: Connection<ShelfWatcherDatabase>, tenant_id: &str) -> Json<HttpResponse<Vec<Location>>> {
    let tenant_uuid = match Uuid::parse_str(tenant_id) {
        Ok(uuid) => uuid,
        Err(_) => return Json(HttpResponse {
            status: 400,
            message: "Invalid tenant ID".to_string(),
            data: None,
        })
    };

    match Location::get_all_from_tenant(tenant_uuid, &db).await {
        Ok(locations) => Json(HttpResponse {
            status: 200,
            message: "Successfully retrieved all locations from tenant".to_string(),
            data: Some(locations),
        }),
        Err(err) => Json(err)
    }
}