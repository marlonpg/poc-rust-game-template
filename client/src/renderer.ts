import { Position, Enemy, EnemyType, Projectile } from "./protocol";

interface DrawContext {
  ctx: CanvasRenderingContext2D;
  cameraX: number;
  cameraY: number;
  scale: number;
}

export class GameRenderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private width: number;
  private height: number;

  constructor(canvasId: string) {
    const canvas = document.getElementById(canvasId) as HTMLCanvasElement;
    if (!canvas) {
      throw new Error(`Canvas element with id '${canvasId}' not found`);
    }

    this.canvas = canvas;
    const ctx = this.canvas.getContext("2d");
    if (!ctx) {
      throw new Error("Failed to get 2D context from canvas");
    }
    this.ctx = ctx;

    // Set canvas size to window size
    this.resizeCanvas();
    window.addEventListener("resize", () => this.resizeCanvas());

    this.width = this.canvas.width;
    this.height = this.canvas.height;
  }

  private resizeCanvas() {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }

  clear(color: string = "#000000") {
    this.ctx.fillStyle = color;
    this.ctx.fillRect(0, 0, this.width, this.height);
  }

  drawGameState(
    playerPos: Position,
    players: { id: string; position: Position; health: number; max_health: number }[],
    enemies: Enemy[],
    projectiles: Projectile[],
    safeZoneRadius: number,
    ringRadius: number,
    maxRings: number
  ) {
    const drawCtx: DrawContext = {
      ctx: this.ctx,
      cameraX: playerPos.x,
      cameraY: playerPos.y,
      scale: 1.0, // Zoom level (closer camera)
    };

    // Draw world
    this.drawWorldBackground(drawCtx);
    this.drawGrid(drawCtx);
    this.drawSafeZone(drawCtx, safeZoneRadius);
    this.drawRings(drawCtx, ringRadius, maxRings);

    // Draw entities
    this.drawEnemies(drawCtx, enemies);
    this.drawProjectiles(drawCtx, projectiles);
    this.drawPlayers(drawCtx, players, playerPos);
  }

  private drawWorldBackground(ctx: DrawContext) {
    ctx.ctx.fillStyle = "#0a0a0a";
    ctx.ctx.fillRect(0, 0, this.width, this.height);
  }

  private drawGrid(ctx: DrawContext) {
    const gridSize = 100;
    const gridColor = "#1a1a1a";
    const startX = Math.floor(-ctx.cameraX / gridSize) * gridSize;
    const startY = Math.floor(-ctx.cameraY / gridSize) * gridSize;
    const endX = startX + (this.width / ctx.scale + gridSize);
    const endY = startY + (this.height / ctx.scale + gridSize);

    ctx.ctx.strokeStyle = gridColor;
    ctx.ctx.lineWidth = 1;

    for (let x = startX; x < endX; x += gridSize) {
      const screenX = (x - ctx.cameraX) * ctx.scale + this.width / 2;
      ctx.ctx.beginPath();
      ctx.ctx.moveTo(screenX, 0);
      ctx.ctx.lineTo(screenX, this.height);
      ctx.ctx.stroke();
    }

    for (let y = startY; y < endY; y += gridSize) {
      const screenY = (y - ctx.cameraY) * ctx.scale + this.height / 2;
      ctx.ctx.beginPath();
      ctx.ctx.moveTo(0, screenY);
      ctx.ctx.lineTo(this.width, screenY);
      ctx.ctx.stroke();
    }
  }

  private drawSafeZone(ctx: DrawContext, radius: number) {
    const [screenX, screenY] = this.screenCoords(0, 0, ctx);
    const screenRadius = radius * ctx.scale;

    ctx.ctx.fillStyle = "rgba(0, 200, 0, 0.05)";
    ctx.ctx.beginPath();
    ctx.ctx.arc(screenX, screenY, screenRadius, 0, Math.PI * 2);
    ctx.ctx.fill();

    ctx.ctx.strokeStyle = "rgba(0, 200, 0, 0.3)";
    ctx.ctx.lineWidth = 2;
    ctx.ctx.stroke();

    ctx.ctx.fillStyle = "rgba(0, 200, 0, 0.5)";
    ctx.ctx.font = "12px Arial";
    ctx.ctx.textAlign = "center";
    ctx.ctx.fillText("SAFE ZONE", screenX, screenY - 10);
  }

  private drawRings(ctx: DrawContext, ringRadius: number, maxRings: number) {
    const [centerX, centerY] = this.screenCoords(0, 0, ctx);

    ctx.ctx.strokeStyle = "rgba(150, 100, 255, 0.2)";
    ctx.ctx.lineWidth = 1;

    for (let i = 1; i <= maxRings; i++) {
      const radius = ringRadius * i * ctx.scale;
      ctx.ctx.beginPath();
      ctx.ctx.arc(centerX, centerY, radius, 0, Math.PI * 2);
      ctx.ctx.stroke();

      // Draw ring number
      const angle = 0;
      const x = centerX + radius * Math.cos(angle);
      const y = centerY + radius * Math.sin(angle);
      ctx.ctx.fillStyle = "rgba(150, 100, 255, 0.3)";
      ctx.ctx.font = "10px Arial";
      ctx.ctx.textAlign = "center";
      ctx.ctx.fillText(`Ring ${i}`, x, y);
    }
  }

  private screenCoords(
    worldX: number,
    worldY: number,
    ctx: DrawContext
  ): [number, number] {
    const screenX = (worldX - ctx.cameraX) * ctx.scale + this.width / 2;
    const screenY = (worldY - ctx.cameraY) * ctx.scale + this.height / 2;
    return [screenX, screenY];
  }

  private drawPlayers(
    ctx: DrawContext,
    players: { id: string; position: Position; health: number; max_health: number }[],
    playerPos: Position
  ) {
    for (const player of players) {
      const [screenX, screenY] = this.screenCoords(
        player.position.x,
        player.position.y,
        ctx
      );

      // Draw player circle
      const radius = 8 * ctx.scale;
      const isCurrentPlayer =
        player.position.x === playerPos.x && player.position.y === playerPos.y;
      ctx.ctx.fillStyle = isCurrentPlayer ? "#00ff00" : "#0099ff";
      ctx.ctx.beginPath();
      ctx.ctx.arc(screenX, screenY, radius, 0, Math.PI * 2);
      ctx.ctx.fill();

      // Draw health bar above player
      this.drawHealthBar(ctx, screenX, screenY - radius - 15, 20, 4, player.health, player.max_health);
    }
  }

  private drawEnemies(ctx: DrawContext, enemies: Enemy[]) {
    for (const enemy of enemies) {
      const [screenX, screenY] = this.screenCoords(
        enemy.position.x,
        enemy.position.y,
        ctx
      );

      const radius = this.getEnemyRadius(enemy.enemy_type);
      const color = this.getEnemyColor(enemy.enemy_type);

      // Draw enemy
      ctx.ctx.fillStyle = color;
      ctx.ctx.beginPath();
      ctx.ctx.arc(screenX, screenY, radius * ctx.scale, 0, Math.PI * 2);
      ctx.ctx.fill();

      // Draw health bar
      this.drawHealthBar(ctx, screenX, screenY - radius * ctx.scale - 12, 18, 3, enemy.health, enemy.max_health);
    }
  }

  private drawHealthBar(
    ctx: DrawContext,
    x: number,
    y: number,
    width: number,
    height: number,
    health: number,
    maxHealth: number
  ) {
    const healthPercent = Math.max(0, health / maxHealth);

    // Background
    ctx.ctx.fillStyle = "#333";
    ctx.ctx.fillRect(x - width / 2, y, width, height);

    // Health fill
    const healthColor = healthPercent > 0.5 ? "#00ff00" : healthPercent > 0.25 ? "#ffff00" : "#ff0000";
    ctx.ctx.fillStyle = healthColor;
    ctx.ctx.fillRect(x - width / 2, y, width * healthPercent, height);

    // Border
    ctx.ctx.strokeStyle = "#fff";
    ctx.ctx.lineWidth = 1;
    ctx.ctx.strokeRect(x - width / 2, y, width, height);
  }

  private drawProjectiles(ctx: DrawContext, projectiles: Projectile[]) {
    for (const proj of projectiles) {
      const [screenX, screenY] = this.screenCoords(
        proj.position.x,
        proj.position.y,
        ctx
      );

      // Draw projectile as bright yellow circle
      const radius = 4 * ctx.scale;
      ctx.ctx.fillStyle = "#ffff00";
      ctx.ctx.beginPath();
      ctx.ctx.arc(screenX, screenY, radius, 0, Math.PI * 2);
      ctx.ctx.fill();

      // Add a glow effect
      ctx.ctx.strokeStyle = "#ffff00";
      ctx.ctx.lineWidth = 2;
      ctx.ctx.stroke();
    }
  }

  private getEnemyRadius(type: EnemyType): number {
    const radiusMap: Record<EnemyType, number> = {
      [EnemyType.Goblin]: 5,
      [EnemyType.Orc]: 7,
      [EnemyType.Wolf]: 6,
      [EnemyType.Skeleton]: 6,
      [EnemyType.Zombie]: 8,
      [EnemyType.Demon]: 9,
      [EnemyType.Wraith]: 5,
      [EnemyType.Troll]: 12,
      [EnemyType.Dragon]: 15,
      [EnemyType.Lich]: 10,
    };
    return radiusMap[type] || 6;
  }

  private getEnemyColor(type: EnemyType): string {
    const colorMap: Record<EnemyType, string> = {
      [EnemyType.Goblin]: "#ffff00",
      [EnemyType.Orc]: "#ff6600",
      [EnemyType.Wolf]: "#ff3333",
      [EnemyType.Skeleton]: "#cccccc",
      [EnemyType.Zombie]: "#00ff00",
      [EnemyType.Wraith]: "#cc99ff",
      [EnemyType.Demon]: "#ff0000",
      [EnemyType.Troll]: "#00ccff",
      [EnemyType.Dragon]: "#ff00ff",
      [EnemyType.Lich]: "#00ffff",
    };
    return colorMap[type] || "#ffffff";
  }
}
