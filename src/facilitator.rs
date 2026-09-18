use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{events::CommittedRoomEvent, graph::DerivedFact};

/// A memory-derived candidate. It is intentionally only a draft shape: it has
/// no decision status and no transition capability.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DraftProposal {
    pub room_id: Uuid,
    pub title: String,
    pub summary: String,
    pub source_event_ids: Vec<Uuid>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FacilitatorError {
    EmptyRoom,
    EmptyTitle,
    EmptySummary,
    MissingProvenance,
    MissingSourceEvent(Uuid),
    SourceEventFromAnotherRoom,
    DuplicateSourceEvent(Uuid),
}

impl Display for FacilitatorError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for FacilitatorError {}

/// Builds a draft candidate from supporting graph context without making it
/// authoritative or activating a gateway decision.
pub fn draft_from_fact(
    fact: &DerivedFact,
    title: impl Into<String>,
    summary: impl Into<String>,
) -> Result<DraftProposal, FacilitatorError> {
    if fact.room_id.is_nil() {
        return Err(FacilitatorError::EmptyRoom);
    }
    if !fact.provenance.is_complete() {
        return Err(FacilitatorError::MissingProvenance);
    }
    let title = title.into();
    let summary = summary.into();
    if title.trim().is_empty() {
        return Err(FacilitatorError::EmptyTitle);
    }
    if summary.trim().is_empty() {
        return Err(FacilitatorError::EmptySummary);
    }
    Ok(DraftProposal {
        room_id: fact.room_id,
        title,
        summary,
        source_event_ids: fact.provenance.source_event_ids.clone(),
    })
}

/// Validates a candidate against the committed events supplied by the gateway
/// adapter. This is the last memory-side check before a candidate crosses the
/// facilitator port.
pub fn validate_draft(
    candidate: DraftProposal,
    source_events: &[CommittedRoomEvent],
) -> Result<DraftProposal, FacilitatorError> {
    if candidate.room_id.is_nil() {
        return Err(FacilitatorError::EmptyRoom);
    }
    if candidate.title.trim().is_empty() {
        return Err(FacilitatorError::EmptyTitle);
    }
    if candidate.summary.trim().is_empty() {
        return Err(FacilitatorError::EmptySummary);
    }
    if candidate.source_event_ids.is_empty() {
        return Err(FacilitatorError::MissingProvenance);
    }

    let events = source_events
        .iter()
        .map(|event| (event.event_id, event))
        .collect::<HashMap<_, _>>();
    let mut seen = std::collections::HashSet::new();
    for event_id in &candidate.source_event_ids {
        if !seen.insert(*event_id) {
            return Err(FacilitatorError::DuplicateSourceEvent(*event_id));
        }
        let event = events
            .get(event_id)
            .ok_or(FacilitatorError::MissingSourceEvent(*event_id))?;
        if event.room_id != candidate.room_id {
            return Err(FacilitatorError::SourceEventFromAnotherRoom);
        }
    }
    Ok(candidate)
}
