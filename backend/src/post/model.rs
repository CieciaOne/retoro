use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::Utc, FromRow};
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
