import { Position } from "./protocol";
import { logger } from "./logger";

export class InputHandler {
  private keys: Set<string> = new Set();
  private mousePos: Position = { x: 0, y: 0 };
  private mouseClicked = false;

  constructor() {
    this.setupEventListeners();
  }

  private setupEventListeners() {
    window.addEventListener("keydown", (e) => {
      const key = e.key.toUpperCase();
      if (["W", "A", "S", "D"].includes(key)) {
        this.keys.add(key);
        logger.input(`Key down: ${key}, held keys: [${Array.from(this.keys).join(", ")}]`);
      }
    });

    window.addEventListener("keyup", (e) => {
      const key = e.key.toUpperCase();
      this.keys.delete(key);
      if (["W", "A", "S", "D"].includes(key)) {
        logger.input(`Key up: ${key}, held keys: [${Array.from(this.keys).join(", ")}]`);
      }
    });

    window.addEventListener("mousemove", (e) => {
      this.mousePos = { x: e.clientX, y: e.clientY };
    });

    window.addEventListener("click", () => {
      this.mouseClicked = true;
      logger.input(`Mouse Click detected at (${this.mousePos.x}, ${this.mousePos.y})`);
    });
  }

  getMovementDirection(): { x: number; y: number } {
    let x = 0;
    let y = 0;

    if (this.keys.has("W")) y -= 1;
    if (this.keys.has("S")) y += 1;
    if (this.keys.has("A")) x -= 1;
    if (this.keys.has("D")) x += 1;

    // Normalize diagonal movement
    if (x !== 0 && y !== 0) {
      x *= 0.707;
      y *= 0.707;
    }

    if (x !== 0 || y !== 0) {
      logger.input(`WASD Movement: direction = (${x.toFixed(2)}, ${y.toFixed(2)}), keys = [${Array.from(this.keys).join(", ")}]`);
    }

    return { x, y };
  }

  getMousePosition(): Position {
    return this.mousePos;
  }

  consumeMouseClick(): boolean {
    const clicked = this.mouseClicked;
    if (clicked) {
      logger.input(`Mouse Click at (${this.mousePos.x}, ${this.mousePos.y})`);
    }
    this.mouseClicked = false;
    return clicked;
  }

  isMoving(): boolean {
    return this.keys.size > 0;
  }
}
