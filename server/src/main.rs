mod config;
mod game_loop;
mod game_state;
mod network;

use config::GameConfig;
use game_state::GameState;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting game server...");

    // Load configuration
    let config = GameConfig::default();
    tracing::info!("Game configuration: {:?}", config);

    // Initialize game state
    let game_state = Arc::new(RwLock::new(GameState::new(config)));

    // Start game loop
    let game_loop_handle = {
        let state = game_state.clone();
        tokio::spawn(async move {
            game_loop::run_game_loop(state).await;
        })
    };

    // Create router
    let app = network::create_router(game_state);

    // Configure server address
    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    // Run server
    let server_handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("Server failed");
    });

    // Wait for either task to complete (they shouldn't)
    tokio::select! {
        _ = game_loop_handle => {
            tracing::error!("Game loop terminated unexpectedly");
        }
        _ = server_handle => {
            tracing::error!("Server terminated unexpectedly");
        }
    }

    Ok(())
}
