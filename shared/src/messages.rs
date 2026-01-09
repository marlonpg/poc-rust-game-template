use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{Enemy, Player, Position, ScoreEntry};

/// Client → Server messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    /// Join the game
    Join,
    /// Move player to a target position
    Move { target: Position },
}

/// Server → Client messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    /// Welcome message with assigned player ID
    Welcome { player_id: Uuid },
    /// Full game state update
    GameState {
        players: Vec<Player>,
        enemies: Vec<Enemy>,
        game_time: f64,
    },
    /// Player death notification
    PlayerDied {
        player_id: Uuid,
        max_ring: u32,
        survival_time: f32,
        enemies_defeated: u32,
        score_recorded: bool,
    },
    /// Top scores
    Scoreboard { scores: Vec<ScoreEntry> },
    /// Error message
    Error { message: String },
}
