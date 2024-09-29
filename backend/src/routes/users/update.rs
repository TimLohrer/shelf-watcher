use std::collections::HashMap;
use mongodb::bson::Uuid;
use pwhash::bcrypt;
use rocket::{error, patch, serde::{json::Json, Deserialize}};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, user::{User, UserMinimal}}};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateUserData {
    email: Option<String>,
    password: Option<String>,
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    #[serde(rename = "lastName")]
    last_name: Option<String>
}

#[allow(unused)]
#[patch("/users/<id>", format = "json", data = "<data>")] 
pub async fn update_user(db: Connection<ShelfWatcherDatabase>, id: &str, data: Json<UpdateUserData>) -> Json<HttpResponse<UserMinimal>> { 
    let data = data.into_inner();

    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err),
            data: None
        })
    };

    let old_user = match User::get_by_id(uuid, &db).await {
        Ok(user) => user,
        Err(err) => return Json(err)
    };

    let mut new_user = match old_user.clone().to_full(&db).await {
        Ok(user) => user,
        Err(err) => return Json(err)
    };

    let mut old_values: HashMap<String, String> = HashMap::new();
    let mut new_values: HashMap<String, String> = HashMap::new();

    if data.email.is_some() {
        new_user.email = data.email.unwrap();
        old_values.insert("email".to_string(), old_user.email.clone());
        new_values.insert("email".to_string(), new_user.email.clone());
    }
    if data.password.is_some() {
        let password_hash = match bcrypt::hash(data.password.unwrap()) {
            Ok(hash) => hash,
            Err(err) => return Json(HttpResponse {
                status: 500,
                message: format!("Failed to hash password: {:?}", err),
                data: None
            })
        };
        
        new_user.password_hash = password_hash;
        old_values.insert("password".to_string(), "HIDDEN".to_string());
        new_values.insert("password".to_string(), "HIDDEN".to_string());
    }
    if data.first_name.is_some() {
        new_user.first_name = data.first_name.unwrap();
        old_values.insert("firstName".to_string(), old_user.first_name.clone());
        new_values.insert("firstName".to_string(), new_user.first_name.clone());
    }
    if data.last_name.is_some() {
        new_user.last_name = data.last_name.unwrap();
        old_values.insert("lastName".to_string(), old_user.last_name.clone());
        new_values.insert("lastName".to_string(), new_user.last_name.clone());
    }

    if new_values.is_empty() {
        return Json(HttpResponse {
            status: 200,
            message: "No updates applied.".to_string(),
            data: Some(new_user.to_minimal())
        });
    }

    match new_user.update(&db).await {
        Ok(user) => {
            // TODO: Implement author_id -> maybe admin action
            match AuditLog::new(user.id, AuditLogEntityType::User, AuditLogAction::Update, "User updated.".to_string(), user.id, Some(old_values), Some(new_values)).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }
            
            Json(HttpResponse {
                status: 200,
                message: "User updated".to_string(),
                data: Some(user)
            })
        },
        Err(err) => Json(err)
    }
}