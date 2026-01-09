use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 2D position in game world
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn distance_from_center(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns which ring (1-10+) this position is in
    pub fn ring(&self, ring_radius: f32) -> u32 {
        let distance = self.distance_from_center();
        ((distance / ring_radius).floor() as u32).max(1)
    }

    pub fn move_towards(&mut self, target: &Position, speed: f32, delta_time: f32) {
        let distance = self.distance_to(target);
        if distance > 0.01 {
            let ratio = (speed * delta_time / distance).min(1.0);
            self.x += (target.x - self.x) * ratio;
            self.y += (target.y - self.y) * ratio;
        }
    }
}

/// Player entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub position: Position,
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
    pub attack_speed: f32, // attacks per second
    pub movement_speed: f32,
    pub last_attack_time: f64, // game time
    pub max_ring_reached: u32,
    pub enemies_defeated: u32,
    pub spawn_time: chrono::DateTime<chrono::Utc>,
}

impl Player {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            position: Position::new(0.0, 0.0), // spawn at center
            health: 100.0,
            max_health: 100.0,
            damage: 10.0,
            attack_speed: 1.0,
            movement_speed: 5.0,
            last_attack_time: 0.0,
            max_ring_reached: 1,
            enemies_defeated: 0,
            spawn_time: chrono::Utc::now(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn is_in_safe_zone(&self, safe_zone_radius: f32) -> bool {
        self.position.distance_from_center() <= safe_zone_radius
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
    }

    pub fn can_attack(&self, current_time: f64) -> bool {
        current_time - self.last_attack_time >= 1.0 / self.attack_speed as f64
    }
}

/// Enemy type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyType {
    Goblin,
    Orc,
    Wolf,
    Skeleton,
    Zombie,
    Demon,
    Wraith,
    Troll,
    Dragon,
    Lich,
}

impl EnemyType {
    pub fn all() -> Vec<EnemyType> {
        vec![
            EnemyType::Goblin,
            EnemyType::Orc,
            EnemyType::Wolf,
            EnemyType::Skeleton,
            EnemyType::Zombie,
            EnemyType::Demon,
            EnemyType::Wraith,
            EnemyType::Troll,
            EnemyType::Dragon,
            EnemyType::Lich,
        ]
    }

    /// Get base stats for this enemy type (ring 1 stats)
    pub fn base_stats(&self) -> EnemyStats {
        match self {
            EnemyType::Goblin => EnemyStats {
                max_health: 20.0,
                damage: 5.0,
                movement_speed: 4.0,
                attack_speed: 0.8,
            },
            EnemyType::Orc => EnemyStats {
                max_health: 40.0,
                damage: 8.0,
                movement_speed: 3.0,
                attack_speed: 0.6,
            },
            EnemyType::Wolf => EnemyStats {
                max_health: 15.0,
                damage: 7.0,
                movement_speed: 6.0,
                attack_speed: 1.2,
            },
            EnemyType::Skeleton => EnemyStats {
                max_health: 25.0,
                damage: 6.0,
                movement_speed: 3.5,
                attack_speed: 0.9,
            },
            EnemyType::Zombie => EnemyStats {
                max_health: 50.0,
                damage: 10.0,
                movement_speed: 2.0,
                attack_speed: 0.5,
            },
            EnemyType::Demon => EnemyStats {
                max_health: 60.0,
                damage: 15.0,
                movement_speed: 4.5,
                attack_speed: 0.7,
            },
            EnemyType::Wraith => EnemyStats {
                max_health: 30.0,
                damage: 12.0,
                movement_speed: 5.0,
                attack_speed: 1.0,
            },
            EnemyType::Troll => EnemyStats {
                max_health: 100.0,
                damage: 20.0,
                movement_speed: 2.5,
                attack_speed: 0.4,
            },
            EnemyType::Dragon => EnemyStats {
                max_health: 150.0,
                damage: 30.0,
                movement_speed: 3.0,
                attack_speed: 0.5,
            },
            EnemyType::Lich => EnemyStats {
                max_health: 120.0,
                damage: 25.0,
                movement_speed: 3.5,
                attack_speed: 0.8,
            },
        }
    }

    /// Get scaled stats for a given ring
    pub fn stats_for_ring(&self, ring: u32) -> EnemyStats {
        let base = self.base_stats();
        let scale = 1.0 + (ring as f32 - 1.0) * 0.3; // 30% increase per ring
        EnemyStats {
            max_health: base.max_health * scale,
            damage: base.damage * scale,
            movement_speed: base.movement_speed * (1.0 + (ring as f32 - 1.0) * 0.1), // 10% speed increase per ring
            attack_speed: base.attack_speed,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EnemyStats {
    pub max_health: f32,
    pub damage: f32,
    pub movement_speed: f32,
    pub attack_speed: f32,
}

/// Enemy entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub id: Uuid,
    pub enemy_type: EnemyType,
    pub position: Position,
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
    pub movement_speed: f32,
    pub attack_speed: f32,
    pub spawn_ring: u32,
    pub last_attack_time: f64,
    pub target_player_id: Option<Uuid>,
}

impl Enemy {
    pub fn new(id: Uuid, enemy_type: EnemyType, position: Position, ring: u32) -> Self {
        let stats = enemy_type.stats_for_ring(ring);
        Self {
            id,
            enemy_type,
            position,
            health: stats.max_health,
            max_health: stats.max_health,
            damage: stats.damage,
            movement_speed: stats.movement_speed,
            attack_speed: stats.attack_speed,
            spawn_ring: ring,
            last_attack_time: 0.0,
            target_player_id: None,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
    }

    pub fn can_attack(&self, current_time: f64) -> bool {
        current_time - self.last_attack_time >= 1.0 / self.attack_speed as f64
    }
}

/// Score entry for the leaderboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreEntry {
    pub player_id: Uuid,
    pub max_ring_reached: u32,
    pub survival_time_seconds: f32,
    pub enemies_defeated: u32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ScoreEntry {
    /// Calculate a composite score for sorting
    pub fn total_score(&self) -> u32 {
        // Primary: max ring, Secondary: survival time, Tertiary: enemies defeated
        self.max_ring_reached * 10000
            + (self.survival_time_seconds as u32) * 10
            + self.enemies_defeated
    }
}
