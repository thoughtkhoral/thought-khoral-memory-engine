use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CommittedRoomEvent {
    pub event_id: Uuid,
    pub room_id: Uuid,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub payload: Value,
}
