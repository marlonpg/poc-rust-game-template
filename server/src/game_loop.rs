use crate::game_state::SharedGameState;
use std::time::Duration;
use tokio::time;

pub async fn run_game_loop(state: SharedGameState) {
    let tick_rate = {
        let s = state.read().await;
        s.config.tick_rate
    };

    let tick_duration = Duration::from_secs_f64(1.0 / tick_rate);
    let mut interval = time::interval(tick_duration);

    tracing::info!("Game loop started at {} ticks/sec", tick_rate);

    loop {
        interval.tick().await;

        let delta_time = 1.0 / tick_rate as f32;

        let mut game = state.write().await;

        // Update game time
        game.game_time += delta_time as f64;

        // Spawn enemies
        game.spawn_enemies(delta_time);

        // Update enemy AI
        game.update_enemies(delta_time);

        // Update projectiles and collisions
        game.update_projectiles(delta_time);

        // Process combat (spawn projectiles)
        game.process_combat();

        // Could add state broadcasting here if needed
        // For now, clients request state via WebSocket
    }
}
