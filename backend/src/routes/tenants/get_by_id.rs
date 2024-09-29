use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{http_response::HttpResponse, tenant::Tenant}};

#[allow(unused)]
#[get("/tenants/<id>", format = "json")] 
pub async fn get_tenant_by_id(db: Connection<ShelfWatcherDatabase>, id: &str) -> Json<HttpResponse<Tenant>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err),
            data: None
        })
    };


    match Tenant::get_by_id(uuid, &db).await {
        Ok(tenant) => Json(HttpResponse {
            status: 200,
            message: "Found tenant by id".to_string(),
            data: Some(tenant),
        }),
        Err(err) => Json(err)
    }
}