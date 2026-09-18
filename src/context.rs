use std::fmt::{Display, Formatter};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::provenance::Provenance;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum DecisionSpecStatus {
    Active,
    Superseded,
    Dismissed,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DecisionSpec {
    pub room_id: Uuid,
    pub decision_id: Uuid,
    pub status: DecisionSpecStatus,
    pub title: String,
    pub summary: String,
    pub provenance: Provenance,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SupportingContextItem {
    pub room_id: Uuid,
    pub content: Value,
    pub provenance: Provenance,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RoomContextSnapshot {
    pub room_id: Uuid,
    pub active_specs: Vec<DecisionSpec>,
    pub supporting_context: Vec<SupportingContextItem>,
    pub generated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContextError {
    DecisionFromAnotherRoom(Uuid),
    SupportingContextFromAnotherRoom(Uuid),
}

impl Display for ContextError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for ContextError {}

pub fn project_room_context(
    room_id: Uuid,
    decision_specs: &[DecisionSpec],
    supporting_context: &[SupportingContextItem],
    generated_at: DateTime<Utc>,
    ttl: Duration,
) -> Result<RoomContextSnapshot, ContextError> {
    let mut active_specs = Vec::new();
    for spec in decision_specs {
        if spec.room_id != room_id {
            return Err(ContextError::DecisionFromAnotherRoom(spec.decision_id));
        }
        if spec.status == DecisionSpecStatus::Active {
            active_specs.push(spec.clone());
        }
    }

    for item in supporting_context {
        if item.room_id != room_id {
            return Err(ContextError::SupportingContextFromAnotherRoom(item.room_id));
        }
    }

    Ok(RoomContextSnapshot {
        room_id,
        active_specs,
        supporting_context: supporting_context.to_vec(),
        generated_at,
        expires_at: generated_at + ttl,
    })
}
