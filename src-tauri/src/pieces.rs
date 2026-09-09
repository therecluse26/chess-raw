use serde::Serialize;

#[derive(Copy, Clone, Serialize)]
pub enum Color {
    White,
    Black,
}

#[derive(Serialize)]
pub struct Piece<T> {
    piece_type: T,
    color: Color,
    starting_position: (u8, u8),
}

#[derive(Serialize)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

// pub const STARTING_PIECES: [Piece<PieceType>] = [
//     Piece {
//         piece_type: PieceType::Rook,
//         color: Color::White,
//         starting_position: (0, 0),
//     },
//     Piece {
//         piece_type: PieceType::Knight,
//         color: Color::White,
//         starting_position: (1, 0),
//     },
//     Piece {
//         piece_type: PieceType::Bishop,
//         color: Color::White,
//         starting_position: (2, 0),
//     },
//     Piece {
//         piece_type: PieceType::Queen,
//         color: Color::White,
//         starting_position: (3, 0),
//     },
//     Piece {
//         piece_type: PieceType::King,
//         color: Color::White,
//         starting_position: (4, 0),
//     },
//     Piece {
//         piece_type: PieceType::Bishop,
//         color: Color::White,
//         starting_position: (5, 0),
//     },
//     Piece {
//         piece_type: PieceType::Knight,
//         color: Color::White,
//         starting_position: (6, 0),
//     },
//     Piece {
//         piece_type: PieceType::Rook,
//         color: Color::White,
//         starting_position: (7, 0),
//     },
//     // White pawns
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::White,
//         starting_position: (0, 1),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::White,
//         starting_position: (1, 1),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::White,
//         starting_position: (2, 1),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::White,
//         starting_position: (3, 1),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::White,
//         starting_position: (4, 1),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::White,
//         starting_position: (5, 1),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::White,
//         starting_position: (6, 1),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::White,
//         starting_position: (7, 1),
//     },
//     Piece {
//         piece_type: PieceType::Rook,
//         color: Color::Black,
//         starting_position: (0, 7),
//     },
//     Piece {
//         piece_type: PieceType::Knight,
//         color: Color::Black,
//         starting_position: (1, 7),
//     },
//     Piece {
//         piece_type: PieceType::Bishop,
//         color: Color::Black,
//         starting_position: (2, 7),
//     },
//     Piece {
//         piece_type: PieceType::Queen,
//         color: Color::Black,
//         starting_position: (3, 7),
//     },
//     Piece {
//         piece_type: PieceType::King,
//         color: Color::Black,
//         starting_position: (4, 7),
//     },
//     Piece {
//         piece_type: PieceType::Bishop,
//         color: Color::Black,
//         starting_position: (5, 7),
//     },
//     Piece {
//         piece_type: PieceType::Knight,
//         color: Color::Black,
//         starting_position: (6, 7),
//     },
//     Piece {
//         piece_type: PieceType::Rook,
//         color: Color::Black,
//         starting_position: (7, 7),
//     },
//     // Black pawns
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::Black,
//         starting_position: (0, 6),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::Black,
//         starting_position: (1, 6),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::Black,
//         starting_position: (2, 6),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::Black,
//         starting_position: (3, 6),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::Black,
//         starting_position: (4, 6),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::Black,
//         starting_position: (5, 6),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::Black,
//         starting_position: (6, 6),
//     },
//     Piece {
//         piece_type: PieceType::Pawn,
//         color: Color::Black,
//         starting_position: (7, 6),
//     },
// ];
