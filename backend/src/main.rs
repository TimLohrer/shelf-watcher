mod db;
mod models;
mod routes;
mod middleware;

use rocket::{
    http::Method::{Connect, Delete, Get, Patch, Post, Put},
    launch, routes,
};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rocket_db_pools::Database;

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Get, Post, Put, Patch, Delete, Connect]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true);

    rocket::build()
        .attach(db::ShelfWatcherDatabase::init())
        .attach(cors.to_cors().unwrap())
        .mount(
            "/api",
            routes![
                // Audit Log routes
                routes::audit_logs::get_by_type::get_audit_logs_by_type,
                routes::audit_logs::get_by_id::get_audit_log_by_id,
                routes::audit_logs::get_by_entity_id::get_audit_log_by_entity_id,
                routes::audit_logs::get_by_user_id::get_audit_logs_by_user_id,

                // User Routes
                routes::users::create::create_user,
                routes::users::get_all::get_all_users,
                routes::users::get_by_id::get_user_by_id,
                routes::users::get_all_tenants::get_all_tenants,
                routes::users::update::update_user,
                routes::users::delete::delete_user,

                // Tenant routes
                routes::tenants::create::create_tenant,
                routes::tenants::get_all::get_all_tenants,
                routes::tenants::get_by_id::get_tenant_by_id,
                routes::tenants::get_all_members::get_all_members,
                routes::tenants::update::update_tenant,
                routes::tenants::delete::delete_tenant,

                // Location routes
                routes::locations::create::create_location,
                routes::locations::get_all::get_all_locations,
                routes::locations::get_by_id::get_location_by_id,
                routes::locations::get_all_from_tenant::get_all_locations_from_tenant,
                routes::locations::update::update_location,
                routes::locations::delete::delete_location,
            ],
        )
}
