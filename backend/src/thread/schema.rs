use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddThreadRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetNThreadsRequest {
    pub n: i64,
}
pub struct GetThreadPosts {
    pub thread: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePostSchema {
    pub id: Uuid,
}