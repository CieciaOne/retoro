use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::Utc, FromRow};
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Thread {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
