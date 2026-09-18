use chrono::{TimeZone, Utc};
use thought_khoral_memory_engine::events::CommittedRoomEvent;
use thought_khoral_memory_engine::graph::{DerivedFact, GraphEdge, GraphNode, GraphStore};
use thought_khoral_memory_engine::provenance::Provenance;
use uuid::Uuid;

fn id(value: u128) -> Uuid {
    Uuid::from_u128(value)
}

fn event(event_id: Uuid, room_id: Uuid, text: &str) -> CommittedRoomEvent {
    CommittedRoomEvent {
        event_id,
        room_id,
        event_type: "room.message".to_owned(),
        occurred_at: Utc.with_ymd_and_hms(2026, 9, 16, 12, 0, 0).unwrap(),
        payload: serde_json::json!({"text": text}),
    }
}

fn provenance(event: &CommittedRoomEvent) -> Provenance {
    Provenance {
        source_event_ids: vec![event.event_id],
        source_timestamps: vec![event.occurred_at],
    }
}

#[test]
fn facts_and_queries_are_partitioned_by_room() {
    let room_a = id(1);
    let room_b = id(2);
    let event_a = event(id(10), room_a, "deployment is blue");
    let event_b = event(id(11), room_b, "deployment is green");
    let mut graph = GraphStore::new();

    graph.record_event(event_a.clone()).unwrap();
    graph.record_event(event_b.clone()).unwrap();
    graph
        .insert_fact(DerivedFact {
            room_id: room_a,
            content: "deployment is blue".to_owned(),
            provenance: provenance(&event_a),
        })
        .unwrap();
    graph
        .insert_fact(DerivedFact {
            room_id: room_b,
            content: "deployment is green".to_owned(),
            provenance: provenance(&event_b),
        })
        .unwrap();

    assert_eq!(graph.facts_for_room(room_a).len(), 1);
    assert!(
        graph
            .facts_for_room(room_a)
            .iter()
            .all(|fact| fact.room_id == room_a)
    );
    assert!(
        graph
            .query(room_a, "deployment")
            .into_iter()
            .all(|item| item.room_id == room_a)
    );
    assert!(
        graph
            .query(room_a, "green")
            .into_iter()
            .all(|item| item.room_id == room_a)
    );
    assert!(graph.query(room_a, "green").is_empty());
}

#[test]
fn cross_room_edges_are_rejected_before_persistence() {
    let room_a = id(1);
    let room_b = id(2);
    let event_a = event(id(10), room_a, "A");
    let event_b = event(id(11), room_b, "B");
    let mut graph = GraphStore::new();

    graph.record_event(event_a.clone()).unwrap();
    graph.record_event(event_b.clone()).unwrap();
    graph
        .insert_node(GraphNode {
            node_id: id(20),
            room_id: room_a,
            label: "A".to_owned(),
            provenance: provenance(&event_a),
        })
        .unwrap();
    graph
        .insert_node(GraphNode {
            node_id: id(21),
            room_id: room_b,
            label: "B".to_owned(),
            provenance: provenance(&event_b),
        })
        .unwrap();

    let result = graph.insert_edge(GraphEdge {
        edge_id: id(30),
        room_id: room_a,
        from_node: id(20),
        to_node: id(21),
        relation: "related_to".to_owned(),
        provenance: provenance(&event_a),
    });

    assert!(result.is_err());
}
