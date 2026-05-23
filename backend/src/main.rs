mod storage;
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::storage::StorageEngine;
use axum::{
    routing::{get, post},
    Router,
};



type Tx = broadcast::Sender<String>

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<StorageEngine>,
    pub broadcaster: Tx,
}

#[tokio::main]
async fn main() {
    let storage_engine = StorageEngine::new("flags.log");
    let (tx, _rx) = broadcast::channel::<String>(100);


    let state = AppState {
        storage: Arc::new(storage_engine),
        broadcaster: tx,
    }

    let app = Router::new()
        .route("/api/flags", get(get_flags_handler).post(set_flags_handler))
        .route("/ws", get(websocket_handler))
        .with_state(state);


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Live Config Engine running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();

}