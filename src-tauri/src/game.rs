use serde::Serialize;

use crate::pieces::{Color, Piece, PieceType};
use std::sync::Mutex;

pub struct AppState {
    pub game: Game,
}

pub fn setup_app_state() -> Mutex<AppState> {
    Mutex::new(AppState {
        game: initialize_game(),
    })
}

#[derive(Serialize)]
struct Player {
    name: Option<String>,
    color: Color,
}

#[derive(Copy, Clone, Serialize)]
struct Square {
    file: char,
    rank: u8,
    color: Color,
}

#[derive(Copy, Clone, Serialize)]
struct Grid {
    squares: [[Square; 8]; 8],
}

fn build_grid() -> Grid {
    let squares = std::array::from_fn(|rank| {
        std::array::from_fn(|file| {
            let square_color = if (rank + file) % 2 == 0 {
                Color::White
            } else {
                Color::Black
            };
            Square {
                file: (b'a' + file as u8) as char,
                rank: (rank + 1) as u8,
                color: square_color,
            }
        })
    });

    Grid { squares }
}

#[derive(Serialize)]
struct Board {
    grid: Grid,
    orientation: Color,
}

#[derive(Serialize)]
struct Move {
    player: Player,
    from: (u8, u8),
    to: (u8, u8),
    piece: Piece<PieceType>,
    captured_piece: Option<Piece<PieceType>>,
}

#[derive(Serialize)]
pub struct Game {
    board: Board,
    players: [Player; 2],
    pieces: Vec<Piece<PieceType>>,
    current_turn: Color,
    move_history: Vec<Move>,
}

fn initialize_game() -> Game {
    let board = Board {
        grid: build_grid(),
        orientation: Color::White,
    };

    let players = [
        Player {
            name: Some("Player 1".to_string()),
            color: Color::White,
        },
        Player {
            name: Some("Player 2".to_string()),
            color: Color::Black,
        },
    ];

    let pieces = Vec::new();

    Game {
        board,
        players,
        pieces,
        current_turn: Color::White,
        move_history: Vec::new(),
    }
}
