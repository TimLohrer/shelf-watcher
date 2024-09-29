use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{http_response::HttpResponse, location::Location}};

#[allow(unused)]
#[get("/locations", format = "json")] 
pub async fn get_all_locations(db: Connection<ShelfWatcherDatabase>) -> Json<HttpResponse<Vec<Location>>> {
    // TODO: Only allow this for admins
    match Location::get_all(&db).await {
        Ok(tenants) => Json(HttpResponse {
            status: 200,
            message: "Successfully retrieved all locations".to_string(),
            data: Some(tenants),
        }),
        Err(err) => Json(err)
    }
}