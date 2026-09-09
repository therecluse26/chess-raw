# Resources

Trusted sources for this mission. Lessons cite these. Verified 2026-09-07.

## Rust language — primary

- **The Rust Programming Language, Ch. 6: Enums and Pattern Matching** —
  <https://doc.rust-lang.org/book/ch06-00-enums.html>
  The official book. This is the chapter that makes the event enum click.
  Status: primary source for lesson 0001.

- **The Rust Programming Language, Ch. 19: Patterns and Matching** —
  <https://doc.rust-lang.org/book/ch19-00-patterns.html>
  Deeper `match` — bindings, guards, destructuring. Needed once events carry data.

- **The Rust Programming Language, Ch. 11.1: Writing Tests** —
  <https://doc.rust-lang.org/book/ch11-01-writing-tests.html>
  `#[test]`, `assert_eq!`, `cargo test`. Every lesson task is graded by a test.

- **The Rust Programming Language, Ch. 15.5: Interior Mutability** —
  <https://doc.rust-lang.org/book/ch15-05-interior-mutability.html>
  Why shared state needs a `Mutex`. Backs lesson 0002.

- **The Rust Programming Language, Ch. 18: OOP Features** —
  <https://doc.rust-lang.org/book/ch18-00-oop.html>
  Why Rust prefers enums over trait objects for closed sets like piece types.

- **The Rust Programming Language, Ch. 7.5: Separating Modules into Files** —
  <https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html>
  A file is not in the crate until `mod` names it. Brad's most frequent blocker.
  See also 7.2 (privacy) and 7.3 (paths).

- **Rust Design Patterns (unofficial)** —
  <https://rust-unofficial.github.io/patterns/>
  Idiom reference. Use to check a design smells right before committing to it.

## Chess domain — primary

- **Chess Programming Wiki: Forsyth-Edwards Notation** —
  <https://www.chessprogramming.org/Forsyth-Edwards_Notation>
  The six FEN fields. Proof that a chess position is not just piece placement.
  Note: the wiki is currently read-only recovery, but readable.

- **Chess Programming Wiki: Algebraic Chess Notation** —
  <https://www.chessprogramming.org/Algebraic_Chess_Notation>
  How to name squares and moves. Our vocabulary for events comes from here.

## Event sourcing — primary

- **Martin Fowler, "Event Sourcing"** —
  <https://martinfowler.com/eaaDev/EventSourcing.html>
  The original clear statement of the pattern. Read the "Basic idea" section
  and the part on rebuilding application state.

## Tauri v2

- **Calling Rust from the Frontend** —
  <https://v2.tauri.app/develop/calling-rust/>
  `#[tauri::command]`, `invoke`, `generate_handler!`. Our bridge.

- **State Management** —
  <https://v2.tauri.app/develop/state-management/>
  `app.manage`, `State<'_, T>`, `Mutex`, and the mismatching-type runtime panic.
  Status: primary source for lesson 0002. Verified against `tauri = "2"`.

- **Calling the Frontend from Rust** —
  <https://v2.tauri.app/develop/calling-frontend/>
  Events and channels. This is how the board gets told a move happened.

## Svelte

- **Svelte docs: Runes** —
  <https://svelte.dev/docs/svelte/what-are-runes>
  `$state`, `$derived`. Note: `$derived` is the same stored-vs-derived idea as
  the Rust core. Useful symmetry to teach with.

## Communities

Not yet chosen. Ask Brad before adding. Candidates to propose later:
r/rust, the official Rust users forum (users.rust-lang.org), and the
Tauri Discord.
