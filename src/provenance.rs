use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Provenance {
    pub source_event_ids: Vec<Uuid>,
    pub source_timestamps: Vec<DateTime<Utc>>,
}

impl Provenance {
    pub fn is_complete(&self) -> bool {
        !self.source_event_ids.is_empty()
            && self.source_event_ids.len() == self.source_timestamps.len()
    }
}
