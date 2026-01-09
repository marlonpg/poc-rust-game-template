# Game Client

A web-based client for the Rust action roguelike game server.

## Quick Start

### Prerequisites
- Node.js 18+ and npm

### Installation
```bash
cd client
npm install
```

### Development
```bash
npm run dev
```

Opens on `http://localhost:5173`

### Build for Production
```bash
npm run build
npm run preview
```

## Architecture

### Structure
```
src/
├── main.ts          # Entry point
├── engine.ts        # Main game engine & state management
├── network.ts       # WebSocket client & server communication
├── renderer.ts      # Canvas 2D rendering
├── input.ts         # Keyboard & mouse input handling
└── protocol.ts      # Shared types from server
```

### Key Components

#### GameEngine (`engine.ts`)
- Manages game state (players, enemies, scores)
- Coordinates network, rendering, and input
- Implements game loop with requestAnimationFrame
- Updates UI with player stats

#### GameClient (`network.ts`)
- WebSocket connection management
- Auto-reconnect with exponential backoff
- Message serialization/deserialization
- Callback-based architecture

#### GameRenderer (`renderer.ts`)
- Canvas 2D rendering
- Camera following player
- World grid with zoom
- Safe zone and ring visualization
- Enemy type-specific colors and sizes
- Health bars for players and enemies

#### InputHandler (`input.ts`)
- WASD keyboard input
- Mouse click movement
- Normalized diagonal movement

### Protocol

See [../server/](../server/) for server implementation.

**Client → Server:**
- `Join` - Join the game
- `Move { target }` - Move to position

**Server → Client:**
- `Welcome { player_id }` - Connection established
- `GameState { players, enemies, game_time }` - World state update (20 Hz)
- `PlayerDied { ... }` - Death notification
- `Scoreboard { scores }` - Top scores
- `Error { message }` - Error message

## Controls

- **WASD** - Move character
- **Mouse Click** - Move to cursor location
- Auto-attack triggers automatically outside safe zone

## Features

- Real-time multiplayer
- Server-authoritative gameplay
- Smooth camera following
- Visual ring difficulty indicators
- Enemy type diversity with colors
- Health bars and UI stats
- Scoreboard tracking
- Auto-reconnection support

## Development

### Type Checking
```bash
npm run type-check
```

### Linting
```bash
npm run lint
```

## Troubleshooting

### Connection Failed
- Ensure the Rust server is running on `localhost:3000`
- Check browser console for detailed error messages
- Try refreshing the page

### Performance Issues
- Reduce number of players/enemies in the server config
- Check canvas size (press F12 for DevTools)
- Monitor FPS with browser DevTools Performance tab

## Future Enhancements

- [ ] Particle effects
- [ ] Sound effects and music
- [ ] Multiple render layers (depth sorting)
- [ ] Weapon/ability visual feedback
- [ ] Minimap
- [ ] Player names above heads
- [ ] Damage numbers floating text
- [ ] Screen shake on hit
- [ ] Pause menu
- [ ] Settings/options
- [ ] Mobile touch support
