struct Player {
    name: Option<String>,
    color: Color,
}

struct Square {
    file: char,
    rank: u8,
    color: Color,
}

struct Grid {
    squares: [[Square; 8]; 8],
}

struct Board {
    grid: Grid,
    orientation: Color,
}

struct Move {
    player: Player,
    from: (u8, u8),
    to: (u8, u8),
    piece: Piece<PieceType>,
    captured_piece: Option<Piece<PieceType>>,
}

struct Game {
    board: Board,
    pieces: Vec<Piece<PieceType>>,
    current_turn: Color,
    move_history: Vec<Move>,
}
