mod storage;
use shared::FeatureFlag;
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::storage::StorageEngine;
use axum::{
    routing::{get, post},
    Router,
    extract::State,
    extract::ws::{
        Message, WebSocket, WebSocketUpgrade
    },
    Json,
    response::IntoResponse,
};
use tower_http::services::ServeDir;



type Tx = broadcast::Sender<FeatureFlag>;

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<StorageEngine>,
    pub broadcaster: Tx,
}




#[tokio::main]
async fn main() {
    let storage_engine = StorageEngine::new("flags.log");
    let (tx, _rx) = broadcast::channel::<FeatureFlag>(100);


    let state = AppState {
        storage: Arc::new(storage_engine),
        broadcaster: tx
    };

    let app = Router::new()
        .route("/api/flags", get(get_flags_handler).post(set_flag_handler))
        .route("/ws", get(websocket_handler))
        .fallback_service(ServeDir::new("static"))
        .with_state(state);


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Live Config Engine running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();

}

async fn get_flags_handler(
    State(state): State<AppState>, 
) -> Json<Vec<FeatureFlag>> {
    let flags = state.storage.get_all_flags();
    Json(flags)
}



async fn set_flag_handler(
    State(state): State<AppState>,
    Json(incoming_flag): Json<FeatureFlag>,
) -> &'static str {
    state.storage.set_flag(incoming_flag.clone());
    
    let _ = state.broadcaster.send(incoming_flag);

    "Flag updated successfully!"
}


//Gatekeeper: Catches the intitial HTTP request and upgrades it to ws. 
async fn websocket_handler(
    ws:WebSocketUpgrade,
    State(state):State<AppState>,
) -> impl IntoResponse {
    //Asxum handles the protocol upgradew, then hands live stream to our background loop
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

//Worker: manages the open pipeline for a specific client
async fn handle_socket(mut socket: WebSocket, state: AppState){
    //Tune private receiver into the server's central broadaast tower
    let mut rx = state.broadcaster.subscribe();

    let initial_flags = state.storage.get_all_flags();
    if let Ok(json_text) = serde_json::to_string(&initial_flags){
       if socket.send(Message::Text(json_text)).await.is_err() {
            return;
        }
    }

    while let Ok(flag) = rx.recv().await {
            if let Ok(json_text) = serde_json::to_string(&flag){
                if socket.send(Message::Text(json_text)).await.is_err() { 
                    break;
                }
            }
        }
    }
