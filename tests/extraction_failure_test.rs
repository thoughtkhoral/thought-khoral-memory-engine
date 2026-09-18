use chrono::{TimeZone, Utc};
use serde::Deserialize;
use serde_json::json;
use thought_khoral_memory_engine::events::CommittedRoomEvent;
use thought_khoral_memory_engine::extractor::{
    DeterministicFixtureExtractor, ExtractionError, RoomScope, ingest_graph_only,
};
use thought_khoral_memory_engine::graph::GraphStore;
use uuid::Uuid;

#[derive(Deserialize)]
struct FixtureEnvelope {
    event: CommittedRoomEvent,
}

fn fixture(path: &str) -> CommittedRoomEvent {
    serde_json::from_str::<FixtureEnvelope>(path)
        .expect("fixture JSON should describe a committed room event")
        .event
}

fn event_with_extraction(
    room_id: Uuid,
    event_id: Uuid,
    extraction: serde_json::Value,
) -> CommittedRoomEvent {
    CommittedRoomEvent {
        event_id,
        room_id,
        event_type: "room.message".to_owned(),
        occurred_at: Utc.with_ymd_and_hms(2026, 9, 16, 12, 0, 0).unwrap(),
        payload: json!({"extraction": extraction}),
    }
}

#[test]
fn fixture_extraction_stays_partitioned_and_non_authoritative() {
    let event_a = fixture(include_str!("../fixtures/room-scoped/room-a.json"));
    let event_b = fixture(include_str!("../fixtures/room-scoped/room-b.json"));
    let room_a = event_a.room_id;
    let room_b = event_b.room_id;
    let backend = DeterministicFixtureExtractor;
    let mut graph = GraphStore::new();

    ingest_graph_only(&mut graph, RoomScope::new(room_a), &event_a, &backend).unwrap();
    ingest_graph_only(&mut graph, RoomScope::new(room_b), &event_b, &backend).unwrap();

    assert_eq!(graph.facts_for_room(room_a).len(), 1);
    assert_eq!(graph.nodes_for_room(room_a).len(), 2);
    assert_eq!(graph.edges_for_room(room_a).len(), 1);
    assert_eq!(graph.supporting_context_for_room(room_a).len(), 1);
    assert!(graph.query(room_a, "green").is_empty());
    assert!(graph.query(room_b, "blue").is_empty());
}

#[test]
fn malformed_extraction_is_a_structured_failure_without_graph_writes() {
    let event = fixture(include_str!("../fixtures/room-scoped/malformed.json"));
    let backend = DeterministicFixtureExtractor;
    let mut graph = GraphStore::new();

    let result = ingest_graph_only(&mut graph, RoomScope::new(event.room_id), &event, &backend);

    assert!(matches!(result, Err(ExtractionError::MalformedOutput(_))));
    assert!(graph.facts_for_room(event.room_id).is_empty());
    assert!(graph.nodes_for_room(event.room_id).is_empty());
}

#[test]
fn missing_provenance_is_rejected_without_partial_writes() {
    let room_id = Uuid::from_u128(1);
    let event_id = Uuid::from_u128(10);
    let event = event_with_extraction(
        room_id,
        event_id,
        json!({
            "facts": [{
                "room_id": room_id,
                "content": "missing provenance",
                "provenance": {"source_event_ids": [], "source_timestamps": []}
            }],
            "nodes": [],
            "edges": [],
            "supporting_context": []
        }),
    );
    let backend = DeterministicFixtureExtractor;
    let mut graph = GraphStore::new();

    let result = ingest_graph_only(&mut graph, RoomScope::new(room_id), &event, &backend);

    assert!(matches!(result, Err(ExtractionError::Graph(_))));
    assert!(graph.facts_for_room(room_id).is_empty());
}

#[test]
fn cross_room_source_ids_are_rejected_without_partial_writes() {
    let room_a = Uuid::from_u128(1);
    let room_b = Uuid::from_u128(2);
    let event_a = event_with_extraction(
        room_a,
        Uuid::from_u128(10),
        json!({"facts": [], "nodes": [], "edges": [], "supporting_context": []}),
    );
    let event_b = event_with_extraction(
        room_b,
        Uuid::from_u128(11),
        json!({"facts": [], "nodes": [], "edges": [], "supporting_context": []}),
    );
    let backend = DeterministicFixtureExtractor;
    let mut graph = GraphStore::new();
    graph.record_event(event_b.clone()).unwrap();
    let extraction = json!({
        "facts": [{
            "room_id": room_a,
            "content": "cross room source",
            "provenance": {
                "source_event_ids": [event_b.event_id],
                "source_timestamps": [event_b.occurred_at]
            }
        }],
        "nodes": [],
        "edges": [],
        "supporting_context": []
    });
    let event_a = CommittedRoomEvent {
        payload: json!({"extraction": extraction}),
        ..event_a
    };

    let result = ingest_graph_only(&mut graph, RoomScope::new(room_a), &event_a, &backend);

    assert!(matches!(result, Err(ExtractionError::Graph(_))));
    assert!(graph.facts_for_room(room_a).is_empty());
}

#[test]
fn backend_timeout_and_failure_are_nonblocking_structured_errors() {
    let room_id = Uuid::from_u128(1);
    let backend = DeterministicFixtureExtractor;
    for marker in ["timeout", "backend-error"] {
        let event = event_with_extraction(
            room_id,
            Uuid::from_u128(if marker == "timeout" { 20 } else { 21 }),
            json!(marker),
        );
        let mut graph = GraphStore::new();

        let result = ingest_graph_only(&mut graph, RoomScope::new(room_id), &event, &backend);

        assert!(matches!(
            result,
            Err(ExtractionError::BackendTimeout | ExtractionError::BackendFailure(_))
        ));
        assert!(graph.facts_for_room(room_id).is_empty());
    }
}
