use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{Enemy, Player, Position, Projectile, ScoreEntry};
use crate::upgrades::UpgradeType;

/// Client → Server messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    /// Join the game
    Join,
    /// Move player to a target position
    Move { target: Position },
    /// Choose an upgrade after leveling up
    ChooseUpgrade { upgrade: UpgradeType },
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
        projectiles: Vec<Projectile>,
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
    /// Player leveled up - present upgrade choices
    LevelUp {
        player_id: Uuid,
        new_level: u32,
        upgrade_choices: Vec<UpgradeType>,
    },
    /// Error message
    Error { message: String },
}
