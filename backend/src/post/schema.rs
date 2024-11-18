use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPostSchema {
    pub thread_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetNPostsSchema {
    pub n: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePostSchema {
    pub id: Uuid,
}
