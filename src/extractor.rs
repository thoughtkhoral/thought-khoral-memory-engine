use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::SupportingContextItem;
use crate::events::CommittedRoomEvent;
use crate::graph::{DerivedFact, GraphEdge, GraphError, GraphNode, GraphStore};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoomScope {
    pub room_id: Uuid,
}

impl RoomScope {
    pub fn new(room_id: Uuid) -> Self {
        Self { room_id }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ExtractionOutput {
    #[serde(default)]
    pub facts: Vec<DerivedFact>,
    #[serde(default)]
    pub nodes: Vec<GraphNode>,
    #[serde(default)]
    pub edges: Vec<GraphEdge>,
    #[serde(default)]
    pub supporting_context: Vec<SupportingContextItem>,
}

pub trait ExtractionBackend {
    fn extract(
        &self,
        event: &CommittedRoomEvent,
        scope: RoomScope,
    ) -> Result<ExtractionOutput, ExtractionError>;
}

pub struct DeterministicFixtureExtractor;

impl ExtractionBackend for DeterministicFixtureExtractor {
    fn extract(
        &self,
        event: &CommittedRoomEvent,
        _scope: RoomScope,
    ) -> Result<ExtractionOutput, ExtractionError> {
        let extraction = event
            .payload
            .get("extraction")
            .ok_or(ExtractionError::MissingOutput)?;

        match extraction.as_str() {
            Some("timeout") => Err(ExtractionError::BackendTimeout),
            Some("backend-error") => Err(ExtractionError::BackendFailure(
                "fixture backend failure".to_owned(),
            )),
            Some(marker) => Err(ExtractionError::MalformedOutput(format!(
                "unknown extraction marker: {marker}"
            ))),
            None => serde_json::from_value(extraction.clone())
                .map_err(|error| ExtractionError::MalformedOutput(error.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExtractionError {
    ScopeMismatch { event_room: Uuid, scope_room: Uuid },
    MissingOutput,
    MalformedOutput(String),
    BackendTimeout,
    BackendFailure(String),
    Graph(GraphError),
}

impl Display for ExtractionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for ExtractionError {}

impl From<GraphError> for ExtractionError {
    fn from(error: GraphError) -> Self {
        Self::Graph(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IngestReceipt {
    pub room_id: Uuid,
    pub event_id: Uuid,
    pub fact_count: usize,
    pub node_count: usize,
    pub edge_count: usize,
    pub supporting_context_count: usize,
}

pub fn ingest_graph_only<B: ExtractionBackend>(
    graph: &mut GraphStore,
    scope: RoomScope,
    event: &CommittedRoomEvent,
    backend: &B,
) -> Result<IngestReceipt, ExtractionError> {
    if event.room_id != scope.room_id {
        return Err(ExtractionError::ScopeMismatch {
            event_room: event.room_id,
            scope_room: scope.room_id,
        });
    }

    let output = backend.extract(event, scope)?;
    let mut candidate = graph.clone();
    candidate.record_event(event.clone())?;
    for node in &output.nodes {
        candidate.insert_node(node.clone())?;
    }
    for edge in &output.edges {
        candidate.insert_edge(edge.clone())?;
    }
    for fact in &output.facts {
        candidate.insert_fact(fact.clone())?;
    }
    for item in &output.supporting_context {
        candidate.insert_supporting_context(item.clone())?;
    }

    let receipt = IngestReceipt {
        room_id: event.room_id,
        event_id: event.event_id,
        fact_count: output.facts.len(),
        node_count: output.nodes.len(),
        edge_count: output.edges.len(),
        supporting_context_count: output.supporting_context.len(),
    };
    *graph = candidate;
    Ok(receipt)
}
