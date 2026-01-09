import {
  ClientMessage,
  ServerMessage,
  Position,
} from "./protocol";
import { logger } from "./logger";
import { logger } from "./logger";

export type ServerCallback = (message: ServerMessage) => void;

export class GameClient {
  private ws: WebSocket | null = null;
  private url: string;
  private callbacks: ServerCallback[] = [];
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 2000;

  constructor(url: string = "ws://localhost:3000/ws") {
    this.url = url;
  }

  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        logger.network(`Attempting to connect to ${this.url}`);
        this.ws = new WebSocket(this.url);

        this.ws.onopen = () => {
          logger.network("WebSocket connected!");
          console.log("Connected to game server");
          this.reconnectAttempts = 0;
          resolve();
        };

        this.ws.onmessage = (event) => {
          try {
            const message = JSON.parse(event.data) as ServerMessage;
                        logger.network(`Received message: ${message.type}`);
            this.callbacks.forEach((cb) => cb(message));
          } catch (e) {
            logger.network(`Failed to parse server message: ${e}`);
            console.error("Failed to parse server message:", e);
          }
        };

        this.ws.onerror = (error) => {
          logger.network(`WebSocket error: ${error}`);
          console.error("WebSocket error:", error);
          reject(error);
        };

        this.ws.onclose = () => {
          logger.network("WebSocket closed");
          console.log("Disconnected from server");
          this.attemptReconnect();
        };
      } catch (e) {
        reject(e);
      }
    });
  }

  private attemptReconnect() {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      console.log(
        `Attempting reconnection (${this.reconnectAttempts}/${this.maxReconnectAttempts}) in ${this.reconnectDelay}ms`
      );
      setTimeout(() => {
        this.connect().catch(() => {
          // Reconnection failed, will try again
        });
      }, this.reconnectDelay);
    } else {
      console.error("Max reconnection attempts reached");
    }
  }

  isConnected(): boolean {
    return this.ws !== null && this.ws.readyState === WebSocket.OPEN;
  }

  subscribe(callback: ServerCallback): () => void {
    this.callbacks.push(callback);
    return () => {
      this.callbacks = this.callbacks.filter((cb) => cb !== callback);
    };
  }

  send(message: ClientMessage) {
    if (this.isConnected()) {
      this.ws!.send(JSON.stringify(message));
    } else {
      logger.network(`Cannot send message (${message.type}): not connected to server`);
      console.warn("Cannot send message: not connected to server");
    }
  }

  join() {
      logger.network("Sending Join message");
    this.send({ type: "Join" });
  }

  move(target: Position) {
      logger.network(`Sending Move to (${target.x.toFixed(2)}, ${target.y.toFixed(2)})`);
    this.send({ type: "Move", target });
  }

  disconnect() {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }
}
