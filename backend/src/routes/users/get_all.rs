use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{http_response::HttpResponse, user::{User, UserMinimal}}};

#[allow(unused)]
#[get("/users", format = "json")] 
pub async fn get_all_users(db: Connection<ShelfWatcherDatabase>) -> Json<HttpResponse<Vec<UserMinimal>>> {
    // TODO: Only allow this for admins
    match User::get_all(&db).await {
        Ok(users) => Json(HttpResponse {
            status: 200,
            message: "Successfully retrieved all users".to_string(),
            data: Some(users),
        }),
        Err(err) => Json(err)
    }
}