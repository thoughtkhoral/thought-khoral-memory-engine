use std::{env, net::SocketAddr, sync::Arc};

use thought_khoral_memory_engine::{
    graph::GraphStore,
    ingestion::{IngestionService, app},
};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let address = env::var("THOUGHT_KHORAL_MEMORY_LISTEN_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:43121".to_owned())
        .parse::<SocketAddr>()
        .expect("THOUGHT_KHORAL_MEMORY_LISTEN_ADDRESS must be a socket address");
    let secret = env::var("THOUGHT_KHORAL_MEMORY_SHARED_SECRET")
        .expect("THOUGHT_KHORAL_MEMORY_SHARED_SECRET is required");
    let service = Arc::new(Mutex::new(IngestionService::new(GraphStore::new(), secret)));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("memory-engine listener must bind");
    axum::serve(listener, app(service))
        .await
        .expect("memory-engine server must remain available");
}
