use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[serde(crate = "rocket::serde")] 
pub struct HttpResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}