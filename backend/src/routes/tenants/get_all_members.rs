use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{http_response::HttpResponse, tenant::Tenant, user::{User, UserMinimal}}};

#[allow(unused)]
#[get("/tenants/<id>/membes", format = "json")] 
pub async fn get_all_members(id: &str, db: Connection<ShelfWatcherDatabase>) -> Json<HttpResponse<Vec<UserMinimal>>> {
    // TODO: Only allow this for team members & admins
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err),
            data: None
        })
    };

    let users = match User::get_all(&db).await {
        Ok(users) => users,
        Err(err) => return Json(err)
    };

    match Tenant::get_by_id(uuid, &db).await {
        Ok(tenant) => {
            let members = users.iter().filter(|user| user.tenants.contains(&tenant.id)).cloned().collect();
        
            Json(HttpResponse {
                status: 200,
                message: "Successfully retrieved all members".to_string(),
                data: Some(members),
            })
        },
        Err(err) => Json(HttpResponse {
            status: 500,
            message: format!("Failed to retrieve all members: {:?}", err),
            data: None,
        })
    }
}