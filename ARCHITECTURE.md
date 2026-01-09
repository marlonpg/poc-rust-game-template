# Game Backend — Server Architecture

This document describes the architecture and implementation of the Rust-based game server.

## Overview
- **Game Type:** Action Roguelike / Survival (Vampire Survivors-style)
- **Architecture:** Server-authoritative multiplayer
- **Tech Stack:** Rust, Tokio, Axum, WebSocket

## Components

### 1. Workspace Structure
```
├── server/          # Game server binary
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── config.rs       # Game configuration
│   │   ├── game_state.rs   # Core game state and logic
│   │   ├── game_loop.rs    # Main game tick loop
│   │   └── network.rs      # WebSocket handling
│   └── Cargo.toml
├── shared/          # Shared types between server and client
│   ├── src/
│   │   ├── lib.rs
│   │   ├── types.rs        # Game entities (Player, Enemy, etc.)
│   │   ├── messages.rs     # Client/Server message protocol
│   │   └── tests.rs        # Unit tests
│   └── Cargo.toml
└── Cargo.toml       # Workspace root
```

### 2. Game State (`game_state.rs`)
Central state manager:
- **Players:** HashMap of active players
- **Enemies:** HashMap of spawned enemies
- **Scores:** Top scores leaderboard
- **Game Time:** Running time counter

Key operations:
- `add_player()` / `remove_player()` — player lifecycle
- `spawn_enemies()` — procedural enemy spawning per ring
- `update_enemies()` — AI targeting and movement
- `process_combat()` — server-authoritative combat resolution

### 3. Game Loop (`game_loop.rs`)
Runs at 20 ticks/second (50ms per tick):
1. Update game time
2. Spawn enemies in active rings
3. Update enemy AI (target closest player, move towards)
4. Process combat (players attack closest enemy, enemies attack target player)
5. Clean up dead entities

### 4. Network Layer (`network.rs`)
WebSocket server on port 3000:
- **Endpoints:**
  - `/ws` — WebSocket connection
  - `/health` — Health check
- **Protocol:**
  - Client → Server: `Join`, `Move { target }`
  - Server → Client: `Welcome`, `GameState`, `PlayerDied`, `Scoreboard`, `Error`
- **State Broadcasting:** 20 updates/sec (50ms interval)

### 5. Game Mechanics

#### Map Design
- **Safe Zone:** 100-unit radius at center (no combat)
- **Rings:** 10 concentric rings, 200 units each
- **Total Map:** 2500-unit radius

#### Player System
- Spawn at center (0, 0)
- Stats: HP (100), Damage (10), Attack Speed (1/sec), Move Speed (5 units/sec)
- Auto-attack closest enemy outside safe zone
- Track max ring reached and enemies defeated

#### Enemy System
10 enemy types with unique stats:
1. **Goblin** — Fast, weak
2. **Orc** — Balanced melee
3. **Wolf** — Very fast, low HP
4. **Skeleton** — Standard undead
5. **Zombie** — Slow, tanky
6. **Demon** — High damage
7. **Wraith** — Fast assassin
8. **Troll** — Very tanky
9. **Dragon** — Boss-level stats
10. **Lich** — Magic damage dealer

Stats scale per ring:
- HP: +30% per ring
- Damage: +30% per ring
- Speed: +10% per ring

#### Combat
- **Range:** 50 units
- **Player:** Attacks closest enemy (if outside safe zone)
- **Enemy:** Attacks closest player (if outside safe zone)
- **Resolution:** Server-authoritative damage calculation
- **Death:** Immediate removal from game

#### Scoreboard
- **Eligibility:** Must reach Ring 10+
- **Metrics:** Max ring, survival time, enemies defeated
- **Score Formula:** `ring * 10000 + time * 10 + kills`
- **Persistence:** In-memory (top 100)

## Configuration

Default settings (`config.rs`):
```rust
tick_rate: 20.0              // 20 Hz game loop
safe_zone_radius: 100.0      // 100 units
ring_radius: 200.0           // 200 units per ring
max_rings: 10
enemy_spawn_rate: 0.5        // 0.5 enemies/sec/ring
map_size: 2500.0             // Total radius
score_min_ring: 10           // Min ring for scoreboard
max_scoreboard_entries: 100
```

## Running the Server

### Development
```bash
# Run with debug logs
RUST_LOG=server=debug cargo run --bin server

# Run on custom port
SERVER_ADDR=0.0.0.0:8080 cargo run --bin server
```

### Production
```bash
# Build optimized binary
cargo build --release

# Run release build
RUST_LOG=info ./target/release/server
```

### Testing
```bash
# Run all tests
cargo test --all

# Run specific test module
cargo test --package shared

# With verbose output
cargo test -- --nocapture
```

## Client Integration

### WebSocket Connection
```javascript
const ws = new WebSocket('ws://localhost:3000/ws');

// Join game
ws.send(JSON.stringify({ type: 'Join' }));

// Move player
ws.send(JSON.stringify({ 
  type: 'Move', 
  target: { x: 100.0, y: 50.0 } 
}));

// Receive game state
ws.onmessage = (event) => {
  const msg = JSON.parse(event.data);
  if (msg.type === 'GameState') {
    // Update rendering
    renderPlayers(msg.players);
    renderEnemies(msg.enemies);
  }
};
```

### Message Protocol

**Client → Server:**
```typescript
type ClientMessage = 
  | { type: 'Join' }
  | { type: 'Move', target: { x: number, y: number } }
```

**Server → Client:**
```typescript
type ServerMessage = 
  | { type: 'Welcome', player_id: string }
  | { type: 'GameState', players: Player[], enemies: Enemy[], game_time: number }
  | { type: 'PlayerDied', player_id: string, max_ring: number, survival_time: number, enemies_defeated: number, score_recorded: boolean }
  | { type: 'Scoreboard', scores: ScoreEntry[] }
  | { type: 'Error', message: string }
```

## Performance Considerations

- **Game Loop:** 20 Hz (50ms ticks) provides smooth gameplay
- **State Broadcasting:** 20 Hz to all clients
- **Enemy Spawning:** Rate-limited to 0.5/sec per active ring
- **Combat Range:** 50 units reduces collision checks
- **Dead Entity Cleanup:** Immediate removal on death

## Anti-Cheat

Server validates:
- Movement bounds and speed
- Attack range and cooldowns
- Damage calculations
- Safe zone boundaries
- Enemy spawning locations

Client only sends:
- Join requests
- Movement targets (not positions)

## Next Steps

### Client Development
- [ ] Implement WebSocket client
- [ ] Render players and enemies (2D sprites)
- [ ] Handle input (WASD movement)
- [ ] Display UI (health, ring, score)

### Server Enhancements
- [ ] Persistent scoreboard (database)
- [ ] Player abilities/upgrades
- [ ] Boss enemies at ring boundaries
- [ ] Loot drops
- [ ] Configuration file support
- [ ] Metrics and monitoring

### DevOps
- [ ] Docker containerization
- [ ] CI/CD pipeline
- [ ] Load testing
- [ ] Horizontal scaling (multiple instances)
