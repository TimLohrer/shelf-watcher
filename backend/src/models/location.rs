use anyhow::Result;
use mongodb::bson::{doc, DateTime, Uuid};
use rocket_db_pools::{mongodb::Collection, Connection};
use rocket::{futures::StreamExt, serde::{Deserialize, Serialize}};
use crate::db::{get_main_db, ShelfWatcherDatabase};

use super::http_response::HttpResponse;

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[serde(crate = "rocket::serde")] 
pub struct Location {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl Location {
    pub const COLLECTION_NAME: &'static str = "locations";

    pub fn new(name: String, tenant_id: Uuid) -> Self {
        Self {
            id: Uuid::new(),
            name,
            tenant_id,
            created_at: DateTime::now().to_string(),
        }
    }

    #[allow(unused)]
    pub async fn get_by_id(id: Uuid, connection: &Connection<ShelfWatcherDatabase>) -> Result<Self, HttpResponse<Self>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": id
        };
        match db.find_one(filter, None).await.unwrap() {
            Some(location) => Ok(location),
            None => Err(HttpResponse {
                status: 404,
                message: "Location not found".to_string(),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_all(connection: &Connection<ShelfWatcherDatabase>) -> Result<Vec<Self>, HttpResponse<Vec<Self>>> {
        let db = Self::get_collection(connection);

        match db.find(None, None).await {
            Ok(cursor) => {
                let locations = cursor.map(|doc| doc.unwrap()).collect::<Vec<Self>>().await;
                Ok(locations)
            },
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching locations: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_all_from_tenant(tenant_id: Uuid, connection: &Connection<ShelfWatcherDatabase>) -> Result<Vec<Self>, HttpResponse<Vec<Self>>> {
        let db = Self::get_collection(connection);

        match db.find(doc! { "tenantId": tenant_id }, None).await {
            Ok(cursor) => {
                let locations = cursor.map(|doc| doc.unwrap()).collect::<Vec<Self>>().await;
                Ok(locations)
            },
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching locations from tenant: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn insert(&self, connection: &Connection<ShelfWatcherDatabase>) -> Result<Self, HttpResponse<Self>> {
        let db = Self::get_collection(connection);

        match db.insert_one(self.clone(), None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error inserting location: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn update(&self, connection: &Connection<ShelfWatcherDatabase>) -> Result<Self, HttpResponse<Self>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": self.id
        };
        match db.replace_one(filter, self.clone(), None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error updating location: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn delete(&self, connection: &Connection<ShelfWatcherDatabase>) -> Result<Self, HttpResponse<()>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": self.id
        };
        match db.delete_one(filter, None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error deleting location: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    fn get_collection(connection: &Connection<ShelfWatcherDatabase>) -> Collection<Self> {
        let db = get_main_db(connection);
        db.collection(Self::COLLECTION_NAME)
    }
}