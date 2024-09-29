use rocket::{error, post, serde::{json::Json, Deserialize}};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, user::{User, UserMinimal}}};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateUserData {
    email: String,
    password: String,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
}

#[allow(unused)]
#[post("/users", format = "json", data = "<data>")] 
pub async fn create_user(db: Connection<ShelfWatcherDatabase>, data: Json<CreateUserData>) -> Json<HttpResponse<UserMinimal>> { 
    let data = data.into_inner();

    if User::get_by_email(&data.email, &db).await.is_ok() {
        return Json(HttpResponse {
            status: 400,
            message: "User with that email already exists".to_string(),
            data: None
        });
    }

    let user = match User::new(data.email, data.password, data.first_name, data.last_name) {
        Ok(user) => user,
        Err(err) => return Json(err)
    };
    
    match user.insert(&db).await {
        Ok(user) => {
            match AuditLog::new(user.id, AuditLogEntityType::User, AuditLogAction::Create, "User created.".to_string(), user.id, None, None).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }
            
            Json(HttpResponse {
                status: 201,
                message: "User created".to_string(),
                data: Some(user)
            })
        },
        Err(err) => Json(err)
    }
}