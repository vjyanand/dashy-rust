use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub id: Uuid,
    pub uid: Uuid,
    pub meta: Option<Value>,
    pub updated: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct StatPath {
    pub id: Uuid,
    pub uid: Uuid,
}

#[derive(Deserialize)]
pub struct StatsPath {
    pub uid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct StatPayload {
    pub number: Option<StatPayloadNumber>,
    pub string: Option<StatPayloadString>,
}

#[derive(Serialize, Deserialize)]
pub struct StatPayloadNumber {
    pub value: f32,
    pub max: Option<f32>,
    pub min: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct StatPayloadString {
    pub value: String,
}
