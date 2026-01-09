# Game Requirements – Working Title

## 1. Game Overview
- **Genre:** Action Roguelike / Survival (inspired by *Vampire Survivors*)
- **Theme:** Medieval fantasy  
  - Dark medieval setting  
  - Fantasy elements inspired by *MU Online*
- **Perspective:** Top-down 2D
- **Core Loop:**  
  - Spawn  
  - Survive waves  
  - Move outward from the center  
  - Face stronger enemies  
  - Die or disconnect  
  - Score recorded (if eligible)

---

## 2. Multiplayer & Networking
- Multiplayer-only game
- Server-authoritative architecture
  - Backend implemented in **Rust**
  - Server controls all game logic
  - Clients handle rendering and input only
- Anti-cheat
  - Server validates:
    - Player movement
    - Combat resolution
    - Enemy AI
    - Damage calculations
- No login system
  - Players join directly into the current match
  - No accounts or persistent identities

---

## 3. Match Rules
- Infinite match
  - The match never ends
  - Enemies spawn forever
- Join-in-progress enabled
- Player lifecycle
  - When a player dies → character is permanently removed
  - When a player disconnects → character is permanently removed
  - Reconnecting always creates a new character
  - No persistence between sessions

---

## 4. Map & World Design
- Central safe zone
  - All players spawn in the center of the map
  - Enemies cannot attack players inside the safe zone
  - Players cannot attack enemies inside the safe zone
- Radial map layout
  - The map is divided into **10 concentric rings**
  - Each ring represents a difficulty level
  - Enemy strength increases with distance from the safe zone

---

## 5. Combat System
- Automatic combat
  - Outside the safe zone, players attack automatically
  - Players attack the closest enemy
- Enemy behavior
  - Enemies target the closest player
  - Enemies follow and attack that player
- Combat resolution
  - Fully handled by the server
  - Clients only send movement and intent

---

## 6. Enemy System
- Enemy scaling
  - Enemy difficulty increases per ring
  - Stronger rings spawn stronger enemies
- Enemy variety
  - At least **10 different enemy types**
  - Each enemy type has:
    - Unique stats (HP, damage, speed)
    - Optional special abilities
- Spawn rules
  - Ring 1 → weakest enemies
  - Ring 10 → strongest enemies
  - Beyond Ring 10 → enemies remain at maximum difficulty

---

## 7. Progression & Risk System
- Risk vs reward design
  - Safer gameplay near the safe zone
  - Higher risk and stronger enemies farther from the center
- Endless difficulty scaling
  - Increased enemy density
  - Increased enemy strength
  - Increased enemy variety

---

## 8. Scoreboard System
- Score eligibility
  - Player must reach **Ring 10**
  - Player must die or disconnect after passing Ring 10
- Score metrics (initial)
  - Maximum ring reached
  - Time survived
  - Enemies defeated (optional)
- Score persistence
  - Stored server-side
  - No player accounts required

---

## 9. Out of Scope / Non-Goals
- No login or authentication system
- No character persistence
- No match resets
- No PvP combat
