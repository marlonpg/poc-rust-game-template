// ============================================================================
// Shared Protocol Types
// ============================================================================

export interface Position {
  x: number;
  y: number;
}

export enum EnemyType {
  Goblin = "Goblin",
  Orc = "Orc",
  Wolf = "Wolf",
  Skeleton = "Skeleton",
  Zombie = "Zombie",
  Demon = "Demon",
  Wraith = "Wraith",
  Troll = "Troll",
  Dragon = "Dragon",
  Lich = "Lich",
}

export interface Player {
  id: string;
  position: Position;
  health: number;
  max_health: number;
  damage: number;
  attack_speed: number;
  movement_speed: number;
  last_attack_time: number;
  max_ring_reached: number;
  enemies_defeated: number;
  spawn_time: string;
  level: number;
  current_xp: number;
  xp_to_next_level: number;
}

export interface Enemy {
  id: string;
  enemy_type: EnemyType;
  position: Position;
  health: number;
  max_health: number;
  damage: number;
  movement_speed: number;
  attack_speed: number;
  spawn_ring: number;
  xp_reward: number;
  last_attack_time: number;
  target_player_id: string | null;
}

export interface ScoreEntry {
  player_id: string;
  max_ring_reached: number;
  survival_time_seconds: number;
  enemies_defeated: number;
  timestamp: string;
}

export interface Projectile {
  id: string;
  owner_id: string;
  position: Position;
  velocity: Position;
  damage: number;
  lifetime: number;
  max_lifetime: number;
}

export enum UpgradeType {
  IncreaseDamage = "IncreaseDamage",
  IncreaseAttackSpeed = "IncreaseAttackSpeed",
  IncreaseProjectileSpeed = "IncreaseProjectileSpeed",
  MultiShot = "MultiShot",
  PiercingShots = "PiercingShots",
  IncreaseMaxHealth = "IncreaseMaxHealth",
  IncreaseMovementSpeed = "IncreaseMovementSpeed",
  HealthRegeneration = "HealthRegeneration",
  PickupRadius = "PickupRadius",
  Magnet = "Magnet",
  Armor = "Armor",
  Luck = "Luck",
}

// Client to Server
export type ClientMessage =
  | { type: "Join" }
  | { type: "Move"; target: Position }
  | { type: "ChooseUpgrade"; upgrade: UpgradeType };

// Server to Client
export type ServerMessage =
  | { type: "Welcome"; player_id: string }
  | { type: "GameState"; players: Player[]; enemies: Enemy[]; projectiles: Projectile[]; game_time: number }
  | {
      type: "PlayerDied";
      player_id: string;
      max_ring: number;
      survival_time: number;
      enemies_defeated: number;
      score_recorded: boolean;
    }
  | { type: "Scoreboard"; scores: ScoreEntry[] }
  | { type: "LevelUp"; player_id: string; new_level: number; upgrade_choices: UpgradeType[] }
  | { type: "Error"; message: string };
