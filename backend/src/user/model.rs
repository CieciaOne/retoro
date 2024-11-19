use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::Utc, FromRow};
use uuid::Uuid;

use super::schema::UserAuthResponse;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub salt: String,
}

impl User {
    pub fn as_auth_reponse(self) -> UserAuthResponse {
        UserAuthResponse {
            id: self.id,
            username: self.name,
            created_at: self.created_at,
        }
    }
}
