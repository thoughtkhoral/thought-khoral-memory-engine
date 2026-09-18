use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::events::CommittedRoomEvent;
use crate::provenance::Provenance;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DerivedFact {
    pub room_id: Uuid,
    pub content: String,
    pub provenance: Provenance,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct GraphNode {
    pub node_id: Uuid,
    pub room_id: Uuid,
    pub label: String,
    pub provenance: Provenance,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct GraphEdge {
    pub edge_id: Uuid,
    pub room_id: Uuid,
    pub from_node: Uuid,
    pub to_node: Uuid,
    pub relation: String,
    pub provenance: Provenance,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GraphError {
    DuplicateEvent(Uuid),
    DuplicateNode(Uuid),
    DuplicateEdge(Uuid),
    EmptyProvenance,
    SourceEventNotFound(Uuid),
    SourceEventFromAnotherRoom(Uuid),
    SourceTimestampMismatch(Uuid),
    NodeNotFound(Uuid),
    NodeFromAnotherRoom(Uuid),
    EdgeFromAnotherRoom(Uuid),
}

impl Display for GraphError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for GraphError {}

#[derive(Default)]
pub struct GraphStore {
    events: HashMap<Uuid, CommittedRoomEvent>,
    facts: HashMap<Uuid, Vec<DerivedFact>>,
    nodes: HashMap<Uuid, GraphNode>,
    edges: HashMap<Uuid, GraphEdge>,
}

impl GraphStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_event(&mut self, event: CommittedRoomEvent) -> Result<(), GraphError> {
        if self.events.contains_key(&event.event_id) {
            return Err(GraphError::DuplicateEvent(event.event_id));
        }
        self.events.insert(event.event_id, event);
        Ok(())
    }

    pub fn insert_fact(&mut self, fact: DerivedFact) -> Result<(), GraphError> {
        self.validate_provenance(fact.room_id, &fact.provenance)?;
        self.facts.entry(fact.room_id).or_default().push(fact);
        Ok(())
    }

    pub fn facts_for_room(&self, room_id: Uuid) -> Vec<DerivedFact> {
        self.facts.get(&room_id).cloned().unwrap_or_default()
    }

    pub fn query(&self, room_id: Uuid, term: &str) -> Vec<DerivedFact> {
        let term = term.to_lowercase();
        self.facts_for_room(room_id)
            .into_iter()
            .filter(|fact| fact.content.to_lowercase().contains(&term))
            .collect()
    }

    pub fn insert_node(&mut self, node: GraphNode) -> Result<(), GraphError> {
        self.validate_provenance(node.room_id, &node.provenance)?;
        if self.nodes.contains_key(&node.node_id) {
            return Err(GraphError::DuplicateNode(node.node_id));
        }
        self.nodes.insert(node.node_id, node);
        Ok(())
    }

    pub fn insert_edge(&mut self, edge: GraphEdge) -> Result<(), GraphError> {
        self.validate_provenance(edge.room_id, &edge.provenance)?;
        let from_node = self
            .nodes
            .get(&edge.from_node)
            .ok_or(GraphError::NodeNotFound(edge.from_node))?;
        let to_node = self
            .nodes
            .get(&edge.to_node)
            .ok_or(GraphError::NodeNotFound(edge.to_node))?;
        if from_node.room_id != edge.room_id {
            return Err(GraphError::NodeFromAnotherRoom(edge.from_node));
        }
        if to_node.room_id != edge.room_id {
            return Err(GraphError::NodeFromAnotherRoom(edge.to_node));
        }
        if self.edges.contains_key(&edge.edge_id) {
            return Err(GraphError::DuplicateEdge(edge.edge_id));
        }
        self.edges.insert(edge.edge_id, edge);
        Ok(())
    }

    fn validate_provenance(
        &self,
        room_id: Uuid,
        provenance: &Provenance,
    ) -> Result<(), GraphError> {
        if !provenance.is_complete() {
            return Err(GraphError::EmptyProvenance);
        }
        for (event_id, timestamp) in provenance
            .source_event_ids
            .iter()
            .zip(&provenance.source_timestamps)
        {
            let event = self
                .events
                .get(event_id)
                .ok_or(GraphError::SourceEventNotFound(*event_id))?;
            if event.room_id != room_id {
                return Err(GraphError::SourceEventFromAnotherRoom(*event_id));
            }
            if event.occurred_at != *timestamp {
                return Err(GraphError::SourceTimestampMismatch(*event_id));
            }
        }
        Ok(())
    }
}
