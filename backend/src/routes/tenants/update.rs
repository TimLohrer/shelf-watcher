use std::collections::HashMap;
use mongodb::bson::Uuid;
use rocket::{error, patch, serde::{json::Json, Deserialize}};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, tenant::Tenant}};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateTenantData {
    name: Option<String>,
    #[serde(rename = "ownerId")]
    owner_id: Option<String>
}

#[allow(unused)]
#[patch("/tenants/<id>", format = "json", data = "<data>")] 
pub async fn update_tenant(db: Connection<ShelfWatcherDatabase>, id: &str, data: Json<UpdateTenantData>) -> Json<HttpResponse<Tenant>> { 
    let data = data.into_inner();

    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err),
            data: None
        })
    };

    let old_tenant = match Tenant::get_by_id(uuid, &db).await {
        Ok(tenant) => tenant,
        Err(err) => return Json(err)
    };

    let mut new_tenant = old_tenant.clone();

    let mut old_values: HashMap<String, String> = HashMap::new();
    let mut new_values: HashMap<String, String> = HashMap::new();

    if data.name.is_some() {
        new_tenant.name = data.name.unwrap();
        old_values.insert("name".to_owned(), old_tenant.name.clone());
        new_values.insert("name".to_owned(), new_tenant.name.clone());
    }
    if data.owner_id.is_some() {
        let owner_id = match Uuid::parse_str(data.owner_id.unwrap()) {
            Ok(owner_id) => owner_id,
            Err(err) => return Json(HttpResponse {
                status: 400,
                message: format!("Invalid owner ID: {:?}", err),
                data: None
            })
        };

        new_tenant.owner_id = owner_id;
        old_values.insert("ownerId".to_owned(), old_tenant.owner_id.clone().to_string());
        new_values.insert("ownerId".to_owned(), new_tenant.owner_id.clone().to_string());
    }

    if new_values.is_empty() {
        return Json(HttpResponse {
            status: 200,
            message: "No updates applied.".to_string(),
            data: Some(new_tenant)
        });
    }

    match new_tenant.update(&db).await {
        Ok(tenant) => {
            // TODO: Implement author_id
            match AuditLog::new(tenant.id, AuditLogEntityType::Tenant, AuditLogAction::Update, "Tenant updated.".to_string(), Uuid::new(), Some(old_values), Some(new_values)).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }
            
            Json(HttpResponse {
                status: 200,
                message: "Tenant updated".to_string(),
                data: Some(new_tenant)
            })
        },
        Err(err) => Json(err)
    }
}