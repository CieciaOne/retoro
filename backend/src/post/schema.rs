use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPostRequest {
    pub author_id: Option<Uuid>,
    pub thread_id: Uuid,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePostRequest {
    pub id: Uuid,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub author_id: Option<Uuid>,
    pub author_name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
