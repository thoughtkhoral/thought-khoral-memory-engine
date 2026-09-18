use chrono::Utc;
use thought_khoral_memory_engine::{
    events::CommittedRoomEvent,
    facilitator::{DraftProposal, FacilitatorError, draft_from_fact, validate_draft},
    graph::DerivedFact,
    provenance::Provenance,
};
use uuid::Uuid;

fn source_event(room_id: Uuid) -> CommittedRoomEvent {
    CommittedRoomEvent {
        event_id: Uuid::new_v4(),
        room_id,
        event_type: "message.created".to_owned(),
        occurred_at: Utc::now(),
        payload: serde_json::json!({"text": "deployment"}),
    }
}

#[test]
fn memory_candidate_preserves_room_and_source_provenance() {
    let room_id = Uuid::new_v4();
    let event = source_event(room_id);
    let fact = DerivedFact {
        room_id,
        content: "deployment uses blue-green rollout".to_owned(),
        provenance: Provenance {
            source_event_ids: vec![event.event_id],
            source_timestamps: vec![event.occurred_at],
        },
    };

    let candidate = draft_from_fact(
        &fact,
        "Use blue-green rollout",
        "The room discussed a blue-green rollout.",
    )
    .expect("a fact with provenance produces a draft candidate");
    let validated = validate_draft(candidate, &[event.clone()]).expect("candidate is valid");

    assert_eq!(validated.room_id, room_id);
    assert_eq!(validated.source_event_ids, vec![event.event_id]);
}

#[test]
fn cross_room_provenance_is_rejected_before_port_output() {
    let room_id = Uuid::new_v4();
    let other_room = Uuid::new_v4();
    let event = source_event(other_room);
    let candidate = DraftProposal {
        room_id,
        title: "wrong room".to_owned(),
        summary: "must not cross room boundaries".to_owned(),
        source_event_ids: vec![event.event_id],
    };

    assert_eq!(
        validate_draft(candidate, &[event]),
        Err(FacilitatorError::SourceEventFromAnotherRoom)
    );
}
