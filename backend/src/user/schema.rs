use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthneticateUserSchema {
    pub name: String,
    pub password: String,
}
pub struct GetUserSchema {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserSchema {
    pub name: String,
    pub password: String,
}
