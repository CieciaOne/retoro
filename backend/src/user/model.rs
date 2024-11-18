use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::Utc, FromRow};
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub salt: String,
}
