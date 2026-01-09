import { GameClient } from "./network";
import { GameRenderer } from "./renderer";
import { InputHandler } from "./input";
import { ServerMessage, Player, Enemy, ScoreEntry, Position } from "./protocol";
import { logger } from "./logger";

export class GameEngine {
  private client: GameClient;
  private renderer: GameRenderer;
  private input: InputHandler;

  private playerId: string | null = null;
  private players: Map<string, Player> = new Map();
  private enemies: Map<string, Enemy> = new Map();
  private scores: ScoreEntry[] = [];

  private gameConfig = {
    safeZoneRadius: 100,
    ringRadius: 200,
    maxRings: 10,
  };

  private lastMoveTime = 0;
  private moveInterval = 50; // ms
  private frameCount = 0;

  constructor() {
    this.client = new GameClient();
    this.renderer = new GameRenderer("canvas");
    this.input = new InputHandler();
    logger.engine("GameEngine initialized");

    this.setupNetworkCallbacks();
  }

  private setupNetworkCallbacks() {
    this.client.subscribe((message: ServerMessage) => {
      switch (message.type) {
        case "Welcome":
          this.playerId = message.player_id;
          console.log("Joined game with ID:", this.playerId);
          break;

        case "GameState":
          this.players = new Map(message.players.map((p) => [p.id, p]));
          this.enemies = new Map(message.enemies.map((e) => [e.id, e]));
          if (this.playerId) {
            const myPlayer = this.players.get(this.playerId);
            if (myPlayer) {
              logger.update(`Player position: (${myPlayer.position.x.toFixed(2)}, ${myPlayer.position.y.toFixed(2)}), Health: ${myPlayer.health}/${myPlayer.max_health}, Ring: ${myPlayer.current_ring}`);
            }
          }
          break;

        case "PlayerDied":
          console.log("Player died:", message);
          if (message.player_id === this.playerId) {
            console.log("YOU DIED!");
            alert(`Game Over! Final Stats:\nRing: ${message.max_ring}\nTime: ${message.survival_time.toFixed(1)}s\nKills: ${message.enemies_defeated}`);
            this.reset();
          }
          break;

        case "Scoreboard":
          this.scores = message.scores;
          this.updateScoreboardUI();
          break;

        case "Error":
          console.error("Server error:", message.message);
          break;
      }

      this.updateUI();
    });
  }

  async start() {
    console.log("Connecting to game server...");
    try {
      await this.client.connect();
      this.client.join();
      this.gameLoop();
    } catch (e) {
      console.error("Failed to connect to server:", e);
      alert(
        "Failed to connect to server. Make sure the server is running on localhost:3000"
      );
    }
  }

  private gameLoop = () => {
    this.frameCount++;
    if (this.frameCount % 30 === 0) {
      // Log every 30 frames (~0.5 seconds at 60fps)
      logger.engine(`Frame ${this.frameCount}, playerId: ${this.playerId ? this.playerId.substring(0, 8) : "none"}`);
    }
    this.update();
    this.render();
    requestAnimationFrame(this.gameLoop);
  };

  private update() {
    if (!this.playerId) return;

    const currentPlayer = this.players.get(this.playerId);
    if (!currentPlayer) return;

    // Handle movement input
    const direction = this.input.getMovementDirection();
    const mouseClicked = this.input.consumeMouseClick();

    if (direction.x !== 0 || direction.y !== 0) {
      // Keyboard movement
      const speed = (currentPlayer.movement_speed || 5) * 2; // speed up client-directed moves
      const target = {
        x: currentPlayer.position.x + direction.x * speed,
        y: currentPlayer.position.y + direction.y * speed,
      };
      logger.engine(
        `Sending keyboard move to (${target.x.toFixed(2)}, ${target.y.toFixed(2)})`
      );
      this.sendMovement(target);
    } else if (mouseClicked) {
      // Mouse click movement
      const mousePos = this.input.getMousePosition();
      const canvasWidth = window.innerWidth;
      const canvasHeight = window.innerHeight;

      // Convert screen coordinates to world coordinates
      // Approximate conversion based on camera at player position
      const screenCenterX = canvasWidth / 2;
      const screenCenterY = canvasHeight / 2;
      const scale = 0.5;

      const worldX =
        currentPlayer.position.x + (mousePos.x - screenCenterX) * scale;
      const worldY =
        currentPlayer.position.y + (mousePos.y - screenCenterY) * scale;

      logger.engine(
        `Sending mouse click move to (${worldX.toFixed(2)}, ${worldY.toFixed(2)})`
      );
      this.sendMovement({ x: worldX, y: worldY });
    }
  }

  private sendMovement(target: Position) {
    const now = Date.now();
    if (now - this.lastMoveTime > this.moveInterval) {
      logger.network(`Sending Move command to server: (${target.x.toFixed(2)}, ${target.y.toFixed(2)})`);
      this.client.move(target);
      this.lastMoveTime = now;
    }
  }

  private render() {
    const currentPlayer = this.players.get(this.playerId || "");
    if (!currentPlayer) {
      this.renderer.clear();
      return;
    }

    const playersArray = Array.from(this.players.values());
    const enemiesArray = Array.from(this.enemies.values());

    this.renderer.drawGameState(
      currentPlayer.position,
      playersArray,
      enemiesArray,
      this.gameConfig.safeZoneRadius,
      this.gameConfig.ringRadius,
      this.gameConfig.maxRings
    );
  }

  private updateUI() {
    const currentPlayer = this.players.get(this.playerId || "");
    if (!currentPlayer) return;

    // Update stats
    const healthPercent = (currentPlayer.health / currentPlayer.max_health) * 100;
    const healthFill = document.getElementById("health-fill");
    if (healthFill) {
      healthFill.style.width = healthPercent + "%";
    }

    document.getElementById("ring-value")!.textContent =
      currentPlayer.max_ring_reached.toString();
    document.getElementById("enemies-value")!.textContent =
      currentPlayer.enemies_defeated.toString();

    const timeSeconds = Math.floor(
      (new Date().getTime() -
        new Date(currentPlayer.spawn_time).getTime()) /
        1000
    );
    document.getElementById("time-value")!.textContent = `${timeSeconds}s`;

    // Update connection status
    const statusEl = document.getElementById("connection-status");
    if (statusEl) {
      if (this.client.isConnected()) {
        statusEl.textContent = "Connected";
        statusEl.className = "connected";
      } else {
        statusEl.textContent = "Disconnected";
        statusEl.className = "disconnected";
      }
    }
  }

  private updateScoreboardUI() {
    const list = document.getElementById("scores-list");
    if (!list) return;

    list.innerHTML = "";
    for (let i = 0; i < Math.min(this.scores.length, 10); i++) {
      const score = this.scores[i];
      const entry = document.createElement("div");
      entry.className = "score-entry";
      entry.textContent = `${i + 1}. Ring ${score.max_ring_reached} - ${score.survival_time_seconds.toFixed(0)}s - ${score.enemies_defeated} kills`;
      list.appendChild(entry);
    }
  }

  private reset() {
    this.playerId = null;
    this.players.clear();
    this.enemies.clear();

    setTimeout(() => {
      location.reload();
    }, 3000);
  }
}
