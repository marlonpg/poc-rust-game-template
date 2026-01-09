use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub tick_rate: f64,        // ticks per second
    pub safe_zone_radius: f32, // radius of safe zone
    pub ring_radius: f32,      // radius of each ring
    pub max_rings: u32,        // number of rings
    pub enemy_spawn_rate: f32, // enemies per second per ring
    pub map_size: f32,         // total map radius
    pub score_min_ring: u32,   // minimum ring to qualify for scoreboard
    pub max_scoreboard_entries: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            tick_rate: 20.0,         // 20 ticks per second (50ms per tick)
            safe_zone_radius: 100.0, // 100 units
            ring_radius: 200.0,      // 200 units per ring
            max_rings: 10,
            enemy_spawn_rate: 0.5, // 0.5 enemies per second per ring
            map_size: 2500.0,      // 2500 units total (beyond ring 10)
            score_min_ring: 10,
            max_scoreboard_entries: 100,
        }
    }
}
