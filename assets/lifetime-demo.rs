//! What Tauri does, with no Tauri. Run it. Watch the order.
//!   rustc --edition 2021 lifetime.rs -o lifetime && ./lifetime
use std::sync::Mutex;

/// A new game is an empty log. That is the whole "initialise".
#[derive(Debug, Default)]
struct GameLog {
    events: Vec<String>,
}

/// Tauri wraps it so many threads can share it. You always alias this.
type AppState = Mutex<GameLog>;

/// Stands in for Tauri's `App`. It OWNS your state for the whole run.
struct App {
    state: AppState,
}

impl App {
    /// `app.manage(x)` — x is MOVED in here. App now owns it.
    fn manage(state: AppState) -> Self {
        println!("  [manage] the App now owns the GameLog");
        Self { state }
    }

    /// `State<'_, AppState>` — a BORROW. The App keeps ownership.
    fn state(&self) -> &AppState {
        &self.state
    }
}

/// A #[tauri::command]. Runs many times. Owns nothing. Borrows the state.
fn play(state: &AppState, mv: &str) -> usize {
    let mut log = state.lock().unwrap(); // lock
    log.events.push(mv.to_string()); // the ONE mutation
    log.events.len()
} // <- lock released here, by scope. You never unlock by hand.

/// "New Game" is not re-initialising. It is one more mutation.
fn new_game(state: &AppState) {
    state.lock().unwrap().events.clear();
    println!("  [new_game] log cleared. Same box, same App.");
}

fn main() {
    println!("1. process starts. No game exists yet.");

    println!("2. setup runs ONCE, ever.");
    let app = App::manage(AppState::default());

    println!("3. event loop. Commands fire whenever the user clicks.");
    for mv in ["e2e4", "e7e5", "g1f3"] {
        let ply = play(app.state(), mv);
        println!("  play({mv}) -> ply {ply}");
    }

    println!("4. state survived all three calls, with no database:");
    println!("  {:?}", app.state().lock().unwrap());

    println!("5. the user clicks New Game.");
    new_game(app.state());
    println!("  {:?}", app.state().lock().unwrap());

    println!("6. process exits. NOW the GameLog is dropped.");

    // ─── EXPERIMENT (lesson 0003, step 2) ───
    // What if you create the game per command, instead of once at setup?
    println!("\n WRONG: App::manage moved INSIDE the loop:");
    for mv in ["e2e4", "e7e5", "g1f3"] {
        let app = App::manage(AppState::default());
        println!("  play({mv}) -> ply {}", play(app.state(), mv));
    }
    println!("  Every ply is 1. Each pass built a new game and threw it away.");
}
