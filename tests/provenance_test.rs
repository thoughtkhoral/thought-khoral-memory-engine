use chrono::{TimeZone, Utc};
use thought_khoral_memory_engine::events::CommittedRoomEvent;
use thought_khoral_memory_engine::graph::{DerivedFact, GraphStore};
use thought_khoral_memory_engine::provenance::Provenance;
use uuid::Uuid;

fn event(event_id: u128, room_id: u128) -> CommittedRoomEvent {
    CommittedRoomEvent {
        event_id: Uuid::from_u128(event_id),
        room_id: Uuid::from_u128(room_id),
        event_type: "room.message".to_owned(),
        occurred_at: Utc.with_ymd_and_hms(2026, 9, 16, 12, 0, 0).unwrap(),
        payload: serde_json::json!({"text": "source"}),
    }
}

#[test]
fn accepted_facts_retain_source_ids_and_timestamps() {
    let source = event(10, 1);
    let mut graph = GraphStore::new();
    graph.record_event(source.clone()).unwrap();

    graph
        .insert_fact(DerivedFact {
            room_id: source.room_id,
            content: "derived".to_owned(),
            provenance: Provenance {
                source_event_ids: vec![source.event_id],
                source_timestamps: vec![source.occurred_at],
            },
        })
        .unwrap();

    let fact = &graph.facts_for_room(source.room_id)[0];
    assert!(!fact.provenance.source_event_ids.is_empty());
    assert_eq!(fact.provenance.source_timestamps, vec![source.occurred_at]);
}

#[test]
fn a_source_event_from_another_room_is_rejected() {
    let source = event(10, 1);
    let other_room = event(11, 2);
    let mut graph = GraphStore::new();
    graph.record_event(source.clone()).unwrap();
    graph.record_event(other_room.clone()).unwrap();

    let result = graph.insert_fact(DerivedFact {
        room_id: source.room_id,
        content: "cross-room".to_owned(),
        provenance: Provenance {
            source_event_ids: vec![other_room.event_id],
            source_timestamps: vec![other_room.occurred_at],
        },
    });

    assert!(result.is_err());
    assert!(graph.facts_for_room(source.room_id).is_empty());
}
