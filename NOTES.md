# Notes

## Preferences (stated 2026-09-07)

- **Brad types the Rust code.** Give a task and a failing test. Give feedback,
  not the answer. Do not paste the solution into the repo.
- **Teach the Svelte frontend too.** It is part of the learning, not a shell.
- Brad pre-empted the "event sourcing is overengineering" objection himself.
  Do not re-litigate it. Back it with the real reason: chess state genuinely
  needs history, and p2p replicates events.

## Working notes

- Repo state at session 1: `src-tauri/src/chess/` holds `board.rs`, `game.rs`,
  `pieces.rs`. `lib.rs` has **no** `mod chess;`. So `cargo check` passes by
  ignoring all three files. They reference `Square`, `Color`, `Board`, `Move`,
  `get_move_set` — none of which are defined anywhere.
- Do not build lesson 1 on that module. Broken compiles eat working memory.
  Build in a fresh module that goes green, and fix `chess/` deliberately later.

## Teachable material already in Brad's own code

Keep these. They are gold, because they are his.

1. `Game.move_history: Vec<Move>` next to six `*_moved: bool` fields.
   An event log, plus six hand-maintained projections of that same log.
2. `Grid::new()` stores each square's light/dark colour. That colour is a pure
   function of `(row + col) % 2`. Stored-vs-derived, a second time.
3. `MoveType` and `ConditionalMoveType` duplicate four variants. In an event
   design these collapse into one event enum. Refactor target for a later lesson.

## Vocabulary decision (frozen 2026-09-07)

`move_history` is renamed **`log`**, and its element type is **`Event`**, not
`Move`. Reason: "move" is a chess word and stays a chess word. See
`reference/glossary.html`. Do not wobble on this.

## Session 2 — 2026-09-09

- Brad restarted his code. `chess/board.rs` is gone. `game.rs` and `pieces.rs`
  remain, still not declared in `lib.rs`, so still never compiled.
- The six castling booleans are gone. Lesson 0001 landed on that point.
- He asked how to manage state in the Tauri backend. Answer became lesson 0002.
- **Verified in his crate, then removed:** the full `manage` + `Mutex` + `State`
  pattern compiles with zero warnings on `tauri = "2"`. The lesson's six tests
  pass against a reference solution. Do not hand him untested skeletons.
- Watch for: he still stores `current_turn` and `Square.color`. Both derivable.
  Do not fix these for him. Wait for him to see it.

## Session 3 — 2026-09-09

- Brad pushed back: lesson 0002 answered design, he asked mechanics. He is right.
  He has only built stateless APIs in Rust. Never a long-lived process.
- **Lead with ownership, not architecture.** He already accepts event sourcing.
  What he lacks is "who owns this value, and for how long".
- Rust Book Ch.4 (Ownership) now outranks Ch.6 (Enums) as priority reading.
- **Runnable beats prose** for time, order and ownership. `assets/lifetime-demo.rs`
  prints the lifecycle, then shows the same loop with `manage` inside it:
  plies go 1,2,3 versus 1,1,1. He sees the bug instead of being told about it.
- Verified from `tauri-2.11.5` source, not from memory:
  - `Builder::manage` **panics** on a duplicate type (`assert!` in app.rs:1943).
  - `Manager::manage` returns **false** and silently does nothing (lib.rs:688).
  - State is keyed by `TypeId`. One value per type, whole app.
  - `State<'r, T>(&'r T)` — literally a shared borrow (state.rs:21).
  - `.setup` is `FnOnce`, called at app.rs:2530, after windows are built.
- Commands cannot be unit tested directly; only Tauri can build a `State`. Split
  each into `foo_inner(&AppState, …)` plus a one-line `#[tauri::command]` shell.
  Verified: two tests pass in his crate, zero warnings.

## Still outstanding

Brad has written neither `log.rs` (lesson 0001) nor `state.rs` (lesson 0002).
Three lessons are delivered and none are proven. **Do not write lesson 0004
until code exists.** Ask what he got stuck on instead.

## Session 4 — 2026-09-09

- Brad hit `E0432 unresolved import crate::game`. Root cause: **no `mod game;`**.
  This is the THIRD time an undeclared module has bitten him (session 1 `chess/`,
  now `game.rs` + `pieces.rs`). It is his single biggest recurring blocker.
- Wrote `reference/rust-modules.html`. Point him at it every time, not a re-explanation.
- **Verified cascade** in his crate (backed up, tested, restored):
  1. as-is → 1 error, `E0432`
  2. + `mod game; mod pieces;` → 10 errors, `E0425` cannot find `Color`/`Piece`/`PieceType`
  3. + `use crate::pieces::…` → `E0603` private ×5, plus `E0277` on
     `const STARTING_PIECES: [Piece<PieceType>]` (unsized — needs `; 32`)
- Tell him the error count going UP after a fix is progress. He may read it as damage.
- Good news: he fixed the `Copy` bug from session 3 himself. `lib.rs` now has
  `let mut guard = state.lock().unwrap(); guard.counter += 1;`. Correct.
- He also fixed the type mismatch by making `setup_app_state() -> Mutex<AppState>`
  line up with `State<'_, Mutex<AppState>>`. He did not use the alias. Do not push
  it again — his version is type-correct.

### Teaching note
He learns by building, then hitting a wall, then asking. That is working. Keep
answers to the wall he hit. Do not bundle the next three lessons into the answer.
