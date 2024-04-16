/// The piece types of a chess game.
#[derive( PartialEq, Eq, Copy, Clone)]
pub enum ChessPieces {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

/// For readability, we don't want to use boolean values (is_player_white == true) to determine the color of a piece. Instead, we use this enum.
#[derive( PartialEq, Eq, Clone, Copy)]
pub enum ChessColors {
    Black,
    White,
}
