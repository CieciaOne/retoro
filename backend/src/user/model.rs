use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::Utc, FromRow};
use uuid::Uuid;

use super::schema::UserResponse;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub salt: String,
}

impl User {
    pub fn as_reponse(&self) -> UserResponse {
        UserResponse {
            id: self.id,
            username: self.name.clone(),
            created_at: self.created_at,
            last_active: self.last_active,
        }
    }
}
