use chrono::{Duration, TimeZone, Utc};
use thought_khoral_memory_engine::context::{
    DecisionSpec, DecisionSpecStatus, SupportingContextItem, project_room_context,
};
use thought_khoral_memory_engine::provenance::Provenance;
use uuid::Uuid;

fn id(value: u128) -> Uuid {
    Uuid::from_u128(value)
}

fn provenance(event_id: u128) -> Provenance {
    Provenance {
        source_event_ids: vec![id(event_id)],
        source_timestamps: vec![Utc.with_ymd_and_hms(2026, 9, 16, 12, 0, 0).unwrap()],
    }
}

#[test]
fn active_specs_and_supporting_context_remain_separate() {
    let room_id = id(1);
    let generated_at = Utc.with_ymd_and_hms(2026, 9, 16, 13, 0, 0).unwrap();
    let specs = vec![
        DecisionSpec {
            room_id,
            decision_id: id(2),
            status: DecisionSpecStatus::Active,
            title: "Deployment target".to_owned(),
            summary: "Use blue".to_owned(),
            provenance: provenance(10),
        },
        DecisionSpec {
            room_id,
            decision_id: id(3),
            status: DecisionSpecStatus::Superseded,
            title: "Old target".to_owned(),
            summary: "Use green".to_owned(),
            provenance: provenance(11),
        },
        DecisionSpec {
            room_id,
            decision_id: id(4),
            status: DecisionSpecStatus::Dismissed,
            title: "Dismissed target".to_owned(),
            summary: "Do not use".to_owned(),
            provenance: provenance(12),
        },
    ];
    let support = vec![SupportingContextItem {
        room_id,
        content: serde_json::json!({"fact": "blue is available"}),
        provenance: provenance(13),
    }];

    let snapshot = project_room_context(
        room_id,
        &specs,
        &support,
        generated_at,
        Duration::minutes(5),
    )
    .unwrap();

    assert_eq!(snapshot.active_specs.len(), 1);
    assert_eq!(snapshot.active_specs[0].decision_id, id(2));
    assert_eq!(snapshot.supporting_context.len(), 1);
    assert_eq!(snapshot.supporting_context[0].room_id, room_id);
    assert_eq!(snapshot.expires_at, generated_at + Duration::minutes(5));
    assert_eq!(specs[1].status, DecisionSpecStatus::Superseded);
}

#[test]
fn cross_room_context_cannot_enter_a_snapshot() {
    let room_id = id(1);
    let other_room = id(2);
    let generated_at = Utc.with_ymd_and_hms(2026, 9, 16, 13, 0, 0).unwrap();
    let support = vec![SupportingContextItem {
        room_id: other_room,
        content: serde_json::json!({"fact": "wrong room"}),
        provenance: provenance(20),
    }];

    let result = project_room_context(room_id, &[], &support, generated_at, Duration::minutes(5));

    assert!(result.is_err());
}
