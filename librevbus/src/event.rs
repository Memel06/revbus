use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub timestamp: u64,
    pub topic: String,
    pub payload: String,
}

impl Event {
    pub fn new(topic: impl Into<String>, payload: impl Into<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        Event {
            id: Uuid::new_v4(),
            timestamp: now,
            topic: topic.into(),
            payload: payload.into(),
        }
    }
}
