use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
    routing::get,
    Router,
};
use futures_util::{stream::StreamExt, SinkExt};
use shared::{ClientMessage, ServerMessage};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::game_state::SharedGameState;

pub fn create_router(state: SharedGameState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .route("/health", get(health_check))
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<SharedGameState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: SharedGameState) {
    let (mut sender, mut receiver) = socket.split();

    let player_id = Arc::new(RwLock::new(None::<Uuid>));
    let player_id_clone = player_id.clone();

    // Spawn task to send game state updates
    let state_clone = state.clone();
    let mut send_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(50)); // 20 updates/sec
        let mut welcome_sent = false;

        loop {
            interval.tick().await;

            let pid = *player_id_clone.read().await;
            if pid.is_none() {
                continue;
            }
            let pid_unwrapped = pid.unwrap();

            // Send Welcome once per connection
            if !welcome_sent {
                let welcome = ServerMessage::Welcome { player_id: pid_unwrapped };
                if let Ok(json) = serde_json::to_string(&welcome) {
                    if sender.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                    welcome_sent = true;
                }
            }

            let game = state_clone.read().await;

            // Send game state
            let msg = ServerMessage::GameState {
                players: game.players.values().cloned().collect(),
                enemies: game.enemies.values().cloned().collect(),
                game_time: game.game_time,
            };

            let json = match serde_json::to_string(&msg) {
                Ok(j) => j,
                Err(e) => {
                    tracing::error!("Failed to serialize message: {}", e);
                    continue;
                }
            };

            if sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    let state_clone = state.clone();
    let player_id_recv = player_id.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(client_msg) => {
                        handle_client_message(client_msg, &state_clone, &player_id_recv).await;
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse client message: {}", e);
                    }
                }
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = &mut send_task => {
            recv_task.abort();
        }
        _ = &mut recv_task => {
            send_task.abort();
        }
    }

    // Cleanup: remove player on disconnect
    let pid = *player_id.read().await;
    if let Some(pid) = pid {
        let mut game = state.write().await;
        game.remove_player(pid);
        tracing::info!("Player {} disconnected", pid);
    }
}

async fn handle_client_message(
    msg: ClientMessage,
    state: &SharedGameState,
    player_id: &Arc<RwLock<Option<Uuid>>>,
) {
    match msg {
        ClientMessage::Join => {
            let new_id = Uuid::new_v4();
            let mut game = state.write().await;
            let _player = game.add_player(new_id);

            *player_id.write().await = Some(new_id);

            tracing::info!("Player {} joined", new_id);

            // Note: Welcome message could be sent here if we had access to sender
            // For simplicity, client will receive game state updates immediately
        }
        ClientMessage::Move { target } => {
            if let Some(pid) = *player_id.read().await {
                let mut game = state.write().await;
                let delta_time = 1.0 / game.config.tick_rate as f32;
                game.move_player(pid, target, delta_time);
            }
        }
    }
}
