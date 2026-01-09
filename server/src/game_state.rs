use rand::Rng;
use shared::{Enemy, EnemyType, Player, Position, ScoreEntry};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::config::GameConfig;

pub type SharedGameState = Arc<RwLock<GameState>>;

#[derive(Debug)]
pub struct GameState {
    pub config: GameConfig,
    pub players: HashMap<Uuid, Player>,
    pub enemies: HashMap<Uuid, Enemy>,
    pub scores: Vec<ScoreEntry>,
    pub game_time: f64,
    pub last_spawn_time: f64,
}

impl GameState {
    pub fn new(config: GameConfig) -> Self {
        Self {
            config,
            players: HashMap::new(),
            enemies: HashMap::new(),
            scores: Vec::new(),
            game_time: 0.0,
            last_spawn_time: 0.0,
        }
    }

    /// Add a new player to the game
    pub fn add_player(&mut self, player_id: Uuid) -> Player {
        let player = Player::new(player_id);
        self.players.insert(player_id, player.clone());
        tracing::info!("Player {} joined the game", player_id);
        player
    }

    /// Remove a player (death or disconnect)
    pub fn remove_player(&mut self, player_id: Uuid) -> Option<Player> {
        let player = self.players.remove(&player_id)?;

        // Check if eligible for scoreboard (reached ring 10+)
        if player.max_ring_reached >= self.config.score_min_ring {
            let survival_time = (chrono::Utc::now() - player.spawn_time).num_seconds() as f32;

            let score = ScoreEntry {
                player_id,
                max_ring_reached: player.max_ring_reached,
                survival_time_seconds: survival_time,
                enemies_defeated: player.enemies_defeated,
                timestamp: chrono::Utc::now(),
            };

            self.add_score(score);
            tracing::info!(
                "Player {} qualified for scoreboard: Ring {}, Time: {:.1}s, Kills: {}",
                player_id,
                player.max_ring_reached,
                survival_time,
                player.enemies_defeated
            );
        }

        Some(player)
    }

    /// Update player movement
    pub fn move_player(&mut self, player_id: Uuid, target: Position, delta_time: f32) {
        if let Some(player) = self.players.get_mut(&player_id) {
            player
                .position
                .move_towards(&target, player.movement_speed, delta_time);

            // Update max ring reached
            let current_ring = player.position.ring(self.config.ring_radius);
            if current_ring > player.max_ring_reached {
                player.max_ring_reached = current_ring;
            }
        }
    }

    /// Spawn enemies based on active rings
    pub fn spawn_enemies(&mut self, _delta_time: f32) {
        let spawn_interval = 1.0 / self.config.enemy_spawn_rate as f64;

        if self.game_time - self.last_spawn_time < spawn_interval {
            return;
        }

        self.last_spawn_time = self.game_time;

        // Determine active rings based on player positions
        let active_rings = self.get_active_rings();

        for ring in active_rings {
            self.spawn_enemy_in_ring(ring);
        }
    }

    fn get_active_rings(&self) -> Vec<u32> {
        let mut rings = std::collections::HashSet::new();

        for player in self.players.values() {
            let player_ring = player.position.ring(self.config.ring_radius);
            // Spawn in player's ring and adjacent rings
            for offset in 0..=1 {
                let ring = (player_ring + offset).min(self.config.max_rings);
                rings.insert(ring);
            }
        }

        rings.into_iter().collect()
    }

    fn spawn_enemy_in_ring(&mut self, ring: u32) {
        let mut rng = rand::thread_rng();

        // Choose random enemy type
        let enemy_types = EnemyType::all();
        let enemy_type = enemy_types[rng.gen_range(0..enemy_types.len())];

        // Generate random position in the ring
        let inner_radius =
            (ring - 1) as f32 * self.config.ring_radius + self.config.safe_zone_radius;
        let outer_radius = ring as f32 * self.config.ring_radius + self.config.safe_zone_radius;
        let radius = rng.gen_range(inner_radius..outer_radius);
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);

        let position = Position::new(radius * angle.cos(), radius * angle.sin());

        let enemy_id = Uuid::new_v4();
        let enemy = Enemy::new(enemy_id, enemy_type, position, ring);

        self.enemies.insert(enemy_id, enemy);
        tracing::debug!(
            "Spawned {:?} in ring {} at ({:.1}, {:.1})",
            enemy_type,
            ring,
            position.x,
            position.y
        );
    }

    /// Update enemy AI and movement
    pub fn update_enemies(&mut self, delta_time: f32) {
        let players: Vec<_> = self.players.values().cloned().collect();

        for enemy in self.enemies.values_mut() {
            // Find closest player
            let closest_player = players.iter().filter(|p| p.is_alive()).min_by(|a, b| {
                let dist_a = enemy.position.distance_to(&a.position);
                let dist_b = enemy.position.distance_to(&b.position);
                dist_a.partial_cmp(&dist_b).unwrap()
            });

            if let Some(target_player) = closest_player {
                enemy.target_player_id = Some(target_player.id);
                enemy.position.move_towards(
                    &target_player.position,
                    enemy.movement_speed,
                    delta_time,
                );
            }
        }
    }

    /// Process combat between players and enemies
    pub fn process_combat(&mut self) {
        let combat_range = 50.0; // Attack range

        // Players attack enemies
        let player_ids: Vec<_> = self.players.keys().cloned().collect();
        for player_id in player_ids {
            let player = match self.players.get(&player_id) {
                Some(p) if p.is_alive() && p.can_attack(self.game_time) => p.clone(),
                _ => continue,
            };

            // Can't attack in safe zone
            if player.is_in_safe_zone(self.config.safe_zone_radius) {
                continue;
            }

            // Find closest enemy
            if let Some((enemy_id, _)) = self
                .enemies
                .iter()
                .filter(|(_, e)| e.is_alive())
                .map(|(id, e)| (id, e.position.distance_to(&player.position)))
                .filter(|(_, dist)| *dist <= combat_range)
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            {
                let enemy_id = *enemy_id;

                // Apply damage
                if let Some(enemy) = self.enemies.get_mut(&enemy_id) {
                    enemy.take_damage(player.damage);

                    if !enemy.is_alive() {
                        tracing::debug!("Player {} killed enemy {}", player_id, enemy_id);
                        if let Some(p) = self.players.get_mut(&player_id) {
                            p.enemies_defeated += 1;
                        }
                    }
                }

                // Update attack cooldown
                if let Some(p) = self.players.get_mut(&player_id) {
                    p.last_attack_time = self.game_time;
                }
            }
        }

        // Enemies attack players
        let enemy_ids: Vec<_> = self.enemies.keys().cloned().collect();
        for enemy_id in enemy_ids {
            let enemy = match self.enemies.get(&enemy_id) {
                Some(e) if e.is_alive() && e.can_attack(self.game_time) => e.clone(),
                _ => continue,
            };

            if let Some(target_id) = enemy.target_player_id {
                if let Some(target_player) = self.players.get(&target_id) {
                    // Can't attack players in safe zone
                    if target_player.is_in_safe_zone(self.config.safe_zone_radius) {
                        continue;
                    }

                    let distance = enemy.position.distance_to(&target_player.position);
                    if distance <= combat_range {
                        // Apply damage
                        if let Some(player) = self.players.get_mut(&target_id) {
                            player.take_damage(enemy.damage);

                            if !player.is_alive() {
                                tracing::info!("Player {} died", target_id);
                            }
                        }

                        // Update attack cooldown
                        if let Some(e) = self.enemies.get_mut(&enemy_id) {
                            e.last_attack_time = self.game_time;
                        }
                    }
                }
            }
        }

        // Clean up dead enemies
        self.enemies.retain(|_, e| e.is_alive());

        // Dead players will be removed when connection drops
    }

    /// Add a score entry to the leaderboard
    fn add_score(&mut self, score: ScoreEntry) {
        self.scores.push(score);

        // Sort by score descending
        self.scores
            .sort_by_key(|s| std::cmp::Reverse(s.total_score()));

        // Keep only top N
        self.scores.truncate(self.config.max_scoreboard_entries);
    }

    /// Get top scores
    #[allow(dead_code)]
    pub fn get_top_scores(&self, limit: usize) -> Vec<ScoreEntry> {
        self.scores.iter().take(limit).cloned().collect()
    }
}
