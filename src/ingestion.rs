use std::fmt::{Display, Formatter};
use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    events::CommittedRoomEvent,
    extractor::{DeterministicFixtureExtractor, ExtractionError, IngestReceipt, ingest_graph_only},
    graph::{GraphError, GraphStore},
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct IngestionRequest {
    pub room_id: Uuid,
    pub event: CommittedRoomEvent,
}

impl From<CommittedRoomEvent> for IngestionRequest {
    fn from(event: CommittedRoomEvent) -> Self {
        Self {
            room_id: event.room_id,
            event,
        }
    }
}

impl IngestionRequest {
    pub fn with_room_id(room_id: Uuid, event: CommittedRoomEvent) -> Self {
        Self { room_id, event }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IngestionError {
    Unauthorized,
    ScopeMismatch {
        request_room: Uuid,
        event_room: Uuid,
    },
    Extraction(ExtractionError),
    Graph(GraphError),
}

impl Display for IngestionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for IngestionError {}

impl From<ExtractionError> for IngestionError {
    fn from(error: ExtractionError) -> Self {
        Self::Extraction(error)
    }
}

impl From<GraphError> for IngestionError {
    fn from(error: GraphError) -> Self {
        Self::Graph(error)
    }
}

pub struct IngestionService {
    graph: GraphStore,
    shared_secret: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireIngestionRequest {
    pub room_id: Uuid,
    pub event_id: Uuid,
    pub event_type: String,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub payload: Value,
}

impl WireIngestionRequest {
    fn into_request(self) -> IngestionRequest {
        let event = CommittedRoomEvent {
            event_id: self.event_id,
            room_id: self.room_id,
            event_type: self.event_type,
            occurred_at: self.occurred_at,
            payload: self.payload,
        };
        IngestionRequest::from(event)
    }
}

impl IngestionService {
    pub fn new(graph: GraphStore, shared_secret: impl Into<String>) -> Self {
        Self {
            graph,
            shared_secret: shared_secret.into(),
        }
    }

    pub fn ingest(
        &mut self,
        authorization: &str,
        request: IngestionRequest,
    ) -> Result<IngestReceipt, IngestionError> {
        if authorization != self.shared_secret {
            return Err(IngestionError::Unauthorized);
        }
        if request.room_id != request.event.room_id {
            return Err(IngestionError::ScopeMismatch {
                request_room: request.room_id,
                event_room: request.event.room_id,
            });
        }

        let backend = DeterministicFixtureExtractor;
        ingest_graph_only(
            &mut self.graph,
            crate::extractor::RoomScope::new(request.room_id),
            &request.event,
            &backend,
        )
        .map_err(Into::into)
    }

    pub fn graph(&self) -> &GraphStore {
        &self.graph
    }

    pub fn active_decisions(&self) -> Vec<Uuid> {
        Vec::new()
    }
}

pub fn app(service: Arc<Mutex<IngestionService>>) -> Router {
    Router::new()
        .route("/healthz", get(|| async { StatusCode::OK }))
        .route("/internal/v1/ingest", post(ingest))
        .with_state(service)
}

async fn ingest(
    State(service): State<Arc<Mutex<IngestionService>>>,
    headers: HeaderMap,
    Json(request): Json<WireIngestionRequest>,
) -> (StatusCode, Json<Value>) {
    let Some(authorization) = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
    else {
        return error_response(StatusCode::UNAUTHORIZED, "unauthorized");
    };

    let request = request.into_request();
    let event_id = request.event.event_id;
    let room_id = request.room_id;
    match service.lock().await.ingest(authorization, request) {
        Ok(receipt) => (
            StatusCode::OK,
            Json(json!({
                "status": "accepted",
                "roomId": receipt.room_id,
                "eventId": receipt.event_id,
            })),
        ),
        Err(error) => {
            let (status, code) = match error {
                IngestionError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
                IngestionError::ScopeMismatch { .. } => (StatusCode::BAD_REQUEST, "scope_mismatch"),
                IngestionError::Extraction(_) => (StatusCode::BAD_REQUEST, "malformed_request"),
                IngestionError::Graph(_) => (StatusCode::BAD_REQUEST, "graph_rejected"),
            };
            let _ = (event_id, room_id);
            error_response(status, code)
        }
    }
}

fn error_response(status: StatusCode, code: &str) -> (StatusCode, Json<Value>) {
    (status, Json(json!({"status": "rejected", "code": code})))
}
