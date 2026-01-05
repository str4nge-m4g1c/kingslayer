# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Kingslayer (Regicide TUI)** is a Terminal User Interface implementation of the cooperative card game Regicide. The project is being developed in Rust with a two-phase approach:

- **Phase 1 (MVP)**: Solo play against AI
- **Phase 2 (Expansion)**: LAN-based multiplayer for 2-4 players

## Tech Stack

- **Language**: Rust
- **TUI Framework**: Ratatui (recommended)
- **Networking** (Phase 2): Tokio with standard TCP sockets
- **Serialization** (Phase 2): JSON for game state exchange

## Core Architecture

### Game State Structure

The game is structured around several core entities that must maintain strict separation of concerns:

1. **Castle Deck (Enemy Deck)**: Layered construction with 4 Kings (bottom), 4 Queens (middle), 4 Jacks (top). Suits within layers are randomized. The top card is always the current enemy.

2. **Tavern Deck (Player Deck)**: Standard 52-card deck (Ace-10 only, face cards removed) + Jesters. Jester count varies by player count (0 for solo, 0 for 2-player, 1 for 3-player, 2 for 4-player).

3. **Hand Management**: Maximum hand size varies by player count (8 solo, 7 for 2-player, 6 for 3-player, 5 for 4-player).

### Game Loop Architecture

Each turn follows a strict 4-step sequence:

1. **Input Phase**: Validate card play (single card, OR combo of 2-4 matching ranks totaling ≤10, OR Ace + any card)
2. **Resolution Phase**: Check immunity, apply suit powers, deal damage
3. **Victory/Defeat Check**: Compare damage to enemy HP
4. **Enemy Attack Phase**: Player must discard cards with value ≥ enemy attack

### Card System

#### Suit Powers (Critical Game Mechanics)

- **Hearts (Heal)**: Shuffle discard pile, move N cards from discard to bottom of Tavern deck (N = attack value)
- **Diamonds (Draw)**: Draw N cards distributed among players (N = attack value)
- **Clubs (Double Damage)**: Attack value counts ×2 against enemy HP
- **Spades (Shield)**: Reduce enemy attack value for current turn; effects are cumulative and persist until enemy defeated

#### Special Cards

- **Ace (Animal Companion)**: Value 1. Can pair with any other card (except Jester) to combine values and activate both suits.
- **Jester**: Value 0. Cancels enemy immunity. Skips enemy attack phase. Player chooses who goes next. Temporarily changes communication rules.

#### Enemy Stats

- Jack: 20 HP, 10 Attack
- Queen: 30 HP, 15 Attack
- King: 40 HP, 20 Attack

#### Enemy Immunity

Each enemy is immune to suit powers (NOT damage) of cards matching their suit. The Jester cancels this immunity. Note: Spades played before Jester against Spades enemy retroactively apply; Clubs played before Jester against Clubs enemy do NOT count double.

### Critical Game Logic Rules

1. **Exact Damage Victory**: If damage exactly equals enemy HP, enemy is captured (placed face-down on top of Tavern deck). Otherwise, enemy goes to discard pile.

2. **Combo Resolution**: When multiple suit powers trigger, Hearts always resolves before Diamonds.

3. **Defeated Enemy Cards**: Jacks drawn = 10 value, Queens = 15, Kings = 20 when played or discarded.

4. **Yielding**: Players can yield (skip to Step 4) unless all other players yielded on their last turn.

5. **Loss Conditions**:
   - Player cannot discard enough cards to satisfy enemy damage
   - Player cannot play a card or yield on their turn

## TUI Layout Design

The terminal must be divided into four panes:

1. **Top Pane (The Castle)**: Current enemy card (ASCII art), HP bar (e.g., `[||||||||||]`), attack stat
2. **Middle Pane (The Battlefield)**: Currently played cards, active shield value, damage capability
3. **Bottom Pane (Hand)**: Player's cards with selection indicators (`> <` or inverted colors)
4. **Side Pane (Log)**: Scrollable event log (e.g., "Player played 5 of Hearts. Healed 5 cards.")

### Visual Constraints

- Strict ASCII/Unicode text only
- No graphical assets or copyrighted character art
- Card rendering example:
```
.-------.
| 5     |
|   ♥   |
|     5 |
'-------'
```
- Use color codes: Red for Hearts/Diamonds, Blue/White for Spades/Clubs

## Phase 2: Multiplayer Architecture

### Network Model

- **Host**: Runs game logic authority, listens on port 5555, displays local IP
- **Clients**: Connect via TCP, send input events, receive state objects

### Communication Protocol

Message types (JSON-serialized):
- `HANDSHAKE`: Client sends name; host assigns player ID
- `GAME_STATE`: Host sends full board state
- `PLAYER_ACTION`: Client sends action (e.g., `{"action": "PLAY", "cards": ["Ace of Spades", "King of Hearts"]}`)
- `ERROR`: Host sends invalid move notification

### Multiplayer-Specific Rules

Turn order is clockwise. TUI must display "Waiting for Player X..." when not local user's turn.

## Development Sequence

The PRD recommends this build order:

1. **Data Structures**: Implement `Card`, `Deck`, `Player`, `Enemy` structs. Build `Deck::shuffle()` and `Castle::construct()`.
2. **TUI Skeleton**: Initialize Ratatui, create 4-pane layout, implement ASCII card renderer.
3. **Solo Game Loop**: Wire input handling to game logic, implement "discard to survive" calculation.
4. **Network Layer**: Build Server (Host) and Client classes, implement JSON GameState serializer.
5. **Polish**: Add colors and ASCII art for face cards (crowns, swords).

## Key Implementation Notes

- **Immunity Logic**: Must carefully track which suit powers trigger vs. are blocked. Damage always applies regardless of immunity.
- **Shield Persistence**: Shield effects from Spades are cumulative and persist until the enemy is defeated, not just for one turn.
- **Combo Validation**: When validating combos, ensure total value ≤10 and all cards have same rank (except Animal Companion combos).
- **Jester Timing**: When Jester is played against Spades enemy, previously played Spades begin working retroactively.
- **Communication Rules**: In multiplayer, players cannot reveal hand contents. Exception: After Jester is played, players may express desire to go next ("I have a good play" allowed; "I have a 10 of Clubs" forbidden).
- **Terminal Resizing**: TUI should handle terminal resizing gracefully without breaking layout.
- **Client Disconnection**: If a client disconnects, host should pause the game.

## Game Balance Data

Solo difficulty levels are determined by Jester usage:
- 0 Jesters used = Gold Victory
- 1 Jester used = Silver Victory
- 2 Jesters used = Bronze Victory

In solo mode, Jester power: "Discard hand and refill to 8 cards (doesn't count as drawing for immunity)". Can activate at start of Step 1 or Step 4.
