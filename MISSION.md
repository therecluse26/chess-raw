# Mission

## The goal

Build a working single-device chess game in Rust and Tauri, driven by an
event-sourced game core, and become genuinely proficient in Rust along the way.

## Why this, why now

Brad wants to improve at Rust. He has a small amount of experience, and does not
call himself proficient. Chess is the vehicle: the rules are known, so all the
difficulty is in the modelling, not in the requirements.

The end state is an **online peer-to-peer chess game**. That end state is what
makes event sourcing the right choice, not decoration. Two peers with no server
must agree on one game. An append-only log of accepted moves is the thing you
replicate. You send events, never commands.

Chess also genuinely needs history. A board position alone is not a legal game
state. Castling rights, the en passant target square, the halfmove clock and the
fullmove counter are all derived from what happened before. This is why FEN
carries six fields, not one.

## Scope right now

In scope:

- The Rust game core: pieces, board, legal moves, check, checkmate, draws.
- The event log, the `apply` fold, and derived state.
- The Tauri command bridge between Rust and the frontend.
- The Svelte frontend. Brad wants to learn this too, not just have it.

Out of scope for now, by Brad's own instruction:

- Networking and peer-to-peer.
- Engine, AI opponent, evaluation.

Every lesson must stay inside that boundary, but may point forward to p2p as the
reason a design choice matters.

## Success looks like

1. Two humans can play a full, legal game of chess on one machine.
2. The game state is a fold over an event log, and nothing else.
3. Brad can read and write idiomatic Rust enums, pattern matches, `Option`,
   `Result`, borrows and tests without needing a lookup for every line.
4. The core is ready to have a network layer bolted on, with no rewrite.

## How Brad learns here

- **Brad types the Rust.** Lessons give a task and a failing test. The teacher
  gives feedback, not the answer.
- The Svelte frontend is taught, not hidden.

## Constraints

- Existing repo: `/home/brad/Code/personal/chess-raw`.
- Tauri v2, Svelte, Rust.
- The `src-tauri/src/chess/` module already exists but is **not** wired into
  `lib.rs`. It does not compile as written. It is raw material, not a foundation.
