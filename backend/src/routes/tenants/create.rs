use mongodb::bson::Uuid;
use rocket::{error, post, serde::{json::Json, Deserialize}};
use rocket_db_pools::Connection;

use crate::{db::ShelfWatcherDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, tenant::Tenant}};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateTenantData {
    name: String
}

#[allow(unused)]
#[post("/tenants", format = "json", data = "<data>")] 
pub async fn create_tenant(db: Connection<ShelfWatcherDatabase>, data: Json<CreateTenantData>) -> Json<HttpResponse<Tenant>> { 
    let data = data.into_inner();

    if Tenant::get_by_name(data.name.clone(), &db).await.is_ok() {
        return Json(HttpResponse {
            status: 400,
            message: "Tenant already exists".to_string(),
            data: None
        });
    }

    let tenant = Tenant::new(data.name);
    
    match tenant.insert(&db).await {
        Ok(tenant) => {
            // TODO: Implement author_id
            match AuditLog::new(tenant.id, AuditLogEntityType::Tenant, AuditLogAction::Create, "Tenant created.".to_string(), Uuid::new(), None, None).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }
            
            Json(HttpResponse {
                status: 201,
                message: "Tenant created".to_string(),
                data: Some(tenant)
            })
        },
        Err(err) => Json(err)
    }
}