use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub id: Uuid,
    pub uid: Uuid,
    pub val: String,
    pub updated: usize,
    pub fetched: usize,
}