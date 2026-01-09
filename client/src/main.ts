import { GameEngine } from "./engine";
import { logger } from "./logger";

// Initialize and start the game
logger.engine("Game starting up...");
const engine = new GameEngine();
engine.start();
logger.engine("Game engine started");

