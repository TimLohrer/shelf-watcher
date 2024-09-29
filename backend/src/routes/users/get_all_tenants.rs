use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{http_response::HttpResponse, tenant::Tenant, user::User}};

#[allow(unused)]
#[get("/users/<id>/tenants", format = "json")] 
pub async fn get_all_tenants(id: &str, db: Connection<ShelfWatcherDatabase>) -> Json<HttpResponse<Vec<Tenant>>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return  Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err.to_string()),
            data: None
        })
    };

    let user = match User::get_by_id(uuid, &db).await {
        Ok(user) => user,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: err.message,
            data: None
        })
    };

    match Tenant::get_all(&db).await {
        Ok(tenants) => Json(HttpResponse {
            status: 200,
            message: "Successfully retrieved all tenants with set owner id".to_string(),
            data: Some(tenants.into_iter().filter(|tenant| user.tenants.contains(&tenant.id)).collect()),
        }),
        Err(err) => Json(err)
    }
}