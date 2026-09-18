use chrono::{TimeZone, Utc};
use serde_json::json;
use thought_khoral_memory_engine::{
    events::CommittedRoomEvent,
    graph::GraphStore,
    ingestion::{IngestionError, IngestionRequest, IngestionService},
};
use uuid::Uuid;

fn event(room_id: Uuid, event_id: Uuid, extraction: serde_json::Value) -> CommittedRoomEvent {
    CommittedRoomEvent {
        event_id,
        room_id,
        event_type: "room.message".to_owned(),
        occurred_at: Utc.with_ymd_and_hms(2026, 9, 18, 12, 0, 0).unwrap(),
        payload: json!({"extraction": extraction}),
    }
}

#[test]
fn authenticated_ingestion_returns_a_receipt_without_active_decision_state() {
    let room_id = Uuid::from_u128(1);
    let event_id = Uuid::from_u128(10);
    let mut service = IngestionService::new(GraphStore::new(), "test-secret");
    let request = IngestionRequest::from(event(
        room_id,
        event_id,
        json!({"facts": [], "nodes": [], "edges": [], "supporting_context": []}),
    ));

    let receipt = service
        .ingest("test-secret", request)
        .expect("authenticated committed events should be accepted");

    assert_eq!(receipt.room_id, room_id);
    assert_eq!(receipt.event_id, event_id);
    assert!(service.active_decisions().is_empty());
}

#[test]
fn unauthenticated_or_cross_room_ingestion_is_rejected_before_graph_writes() {
    let room_a = Uuid::from_u128(1);
    let room_b = Uuid::from_u128(2);
    let event = event(
        room_a,
        Uuid::from_u128(10),
        json!({"facts": [], "nodes": [], "edges": [], "supporting_context": []}),
    );
    let mut service = IngestionService::new(GraphStore::new(), "test-secret");

    assert!(matches!(
        service.ingest("wrong-secret", IngestionRequest::from(event.clone())),
        Err(IngestionError::Unauthorized)
    ));
    assert!(matches!(
        service.ingest("test-secret", IngestionRequest::with_room_id(room_b, event),),
        Err(IngestionError::ScopeMismatch { .. })
    ));
    assert!(service.graph().facts_for_room(room_a).is_empty());
    assert!(service.graph().facts_for_room(room_b).is_empty());
}
