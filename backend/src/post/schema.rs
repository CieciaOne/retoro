use serde::{Deserialize, Serialize};
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
