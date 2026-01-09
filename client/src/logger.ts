// Console logger with on-screen display
export class GameLogger {
  private maxLogs = 8;
  private logs: { message: string; type: string }[] = [];
  private consoleElement: HTMLElement | null = null;

  constructor() {
    this.consoleElement = document.getElementById("console-overlay");
  }

  private addLog(message: string, type: "input" | "engine" | "network" | "update") {
    this.logs.push({ message, type });

    // Keep only the last N logs
    if (this.logs.length > this.maxLogs) {
      this.logs.shift();
    }

    // Write to browser console
    console.log(message);

    // Update on-screen console
    this.updateDisplay();
  }

  private updateDisplay() {
    if (!this.consoleElement) return;

    this.consoleElement.innerHTML = this.logs
      .map((log) => {
        const className = `console-log log-${log.type}`;
        return `<div class="${className}">${this.escapeHtml(log.message)}</div>`;
      })
      .join("");

    // Auto-scroll to bottom
    this.consoleElement.scrollTop = this.consoleElement.scrollHeight;
  }

  private escapeHtml(text: string): string {
    const map: { [key: string]: string } = {
      "&": "&amp;",
      "<": "&lt;",
      ">": "&gt;",
      '"': "&quot;",
      "'": "&#039;",
    };
    return text.replace(/[&<>"']/g, (m) => map[m]);
  }

  input(message: string) {
    this.addLog(`[INPUT] ${message}`, "input");
  }

  engine(message: string) {
    this.addLog(`[ENGINE] ${message}`, "engine");
  }

  network(message: string) {
    this.addLog(`[NETWORK] ${message}`, "network");
  }

  update(message: string) {
    this.addLog(`[SERVER UPDATE] ${message}`, "update");
  }
}

// Global logger instance
export const logger = new GameLogger();
