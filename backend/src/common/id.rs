use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct IdQuery {
    pub id: Uuid,
}
