# Game Client — Development Guide

## Project Setup

### Dependencies
- **Vite** - Fast build tool and dev server
- **TypeScript** - Type-safe JavaScript
- **Node.js** - JavaScript runtime

### Initial Setup
```bash
cd client
npm install
npm run dev
```

## Development Workflow

### File Structure
```
client/
├── index.html          # HTML entry point with canvas & UI
├── src/
│   ├── main.ts         # Bootstraps GameEngine
│   ├── engine.ts       # Core game loop & state
│   ├── network.ts      # WebSocket connection
│   ├── renderer.ts     # Canvas rendering
│   ├── input.ts        # Input handling
│   └── protocol.ts     # Type definitions
├── package.json
├── tsconfig.json
├── vite.config.ts
└── README.md
```

## Key Concepts

### Game Loop
The game runs on `requestAnimationFrame`:
1. **Update** - Handle input, send movement
2. **Render** - Draw game state to canvas

### Network Flow
```
User Input → GameEngine
    ↓
GameClient (WebSocket)
    ↓
Rust Server
    ↓
GameState Message
    ↓
GameEngine (subscribers)
    ↓
Renderer
    ↓
Canvas
```

### Coordinate System
- **World Space** - Game coordinates (-2500 to +2500)
- **Screen Space** - Canvas pixels (0 to width/height)
- **Camera** - Follows player, scaled 0.5x

### Rendering
- Grid background (100-unit cells)
- Safe zone (100-unit radius, green tint)
- Rings (concentric circles, purple)
- Enemies (colored circles, type-dependent)
- Players (green for you, blue for others)
- Health bars (above all entities)

## Common Tasks

### Add New Enemy Type Rendering
Edit `renderer.ts`:
```typescript
private getEnemyColor(type: EnemyType): string {
  const colorMap: Record<EnemyType, string> = {
    [EnemyType.NewType]: "#hexcolor",
    // ...
  };
}
```

### Change Rendering Scale
In `engine.ts` or `renderer.ts`, modify:
```typescript
scale: 0.5  // Change to zoom in/out
```

### Adjust Movement Smoothness
In `engine.ts`:
```typescript
moveInterval = 50  // Milliseconds between movement updates
```

### Add UI Element
Edit `index.html` in the `#ui` or appropriate section, then update `engine.ts` `updateUI()` method.

## Performance Tips

1. **Reduce draw calls**: Batch entity rendering
2. **Optimize collision**: Use spatial partitioning
3. **Cache computed values**: Pre-calculate screen coordinates
4. **Profile**: Use browser DevTools Performance tab

## Debugging

### Browser Console
```javascript
// Check connection
console.log(client.isConnected());

// View game state
console.log(players, enemies, scores);
```

### Network Tab
Monitor WebSocket frames:
- Open DevTools → Network → WS
- Watch message flow in real-time

### Canvas Debug
Add debug rectangles:
```typescript
ctx.ctx.strokeStyle = "red";
ctx.ctx.strokeRect(x, y, width, height);
```

## Building

### Development Build
```bash
npm run dev
```

### Production Build
```bash
npm run build  # Creates dist/ folder
npm run preview  # Test production build locally
```

Deploy `dist/` folder to web server.

## Testing

### Unit Testing
Currently not set up. To add:
```bash
npm install --save-dev vitest @testing-library/dom
```

### Manual Testing Checklist
- [ ] Connect to server
- [ ] Join game
- [ ] Move with WASD
- [ ] Move with mouse click
- [ ] See other players
- [ ] See enemies
- [ ] See health bars
- [ ] See scoreboard
- [ ] Handle disconnection
- [ ] Auto-reconnect works

## Troubleshooting

### "Cannot connect to server"
1. Check server is running: `cargo run --bin server`
2. Check server port: defaults to 3000
3. Check firewall blocking

### "Movement not working"
1. Check console for errors
2. Verify `moveInterval` is short enough
3. Check server is receiving messages

### "Rendering issues"
1. Check browser DevTools for errors
2. Verify canvas size: `canvas.width` × `canvas.height`
3. Try zooming browser window

### "Performance lag"
1. Check FPS in DevTools
2. Reduce enemy/player count in server config
3. Check for memory leaks (DevTools → Memory)

## Resources

- [Vite Docs](https://vitejs.dev/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Canvas API](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API)
- [WebSocket API](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)

## Next Steps

1. **Particle Effects** - Add explosions, hit feedback
2. **Sound** - Web Audio API for effects and music
3. **UI Improvements** - Main menu, pause screen, settings
4. **Mobile Support** - Touch input for mobile devices
5. **Performance** - WebGL renderer for large player counts
