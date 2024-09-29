use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{http_response::HttpResponse, tenant::Tenant}};

#[allow(unused)]
#[get("/tenants", format = "json")] 
pub async fn get_all_tenants(db: Connection<ShelfWatcherDatabase>) -> Json<HttpResponse<Vec<Tenant>>> {
    // TODO: Only allow this for admins
    match Tenant::get_all(&db).await {
        Ok(tenants) => Json(HttpResponse {
            status: 200,
            message: "Successfully retrieved all tenants".to_string(),
            data: Some(tenants),
        }),
        Err(err) => Json(err)
    }
}