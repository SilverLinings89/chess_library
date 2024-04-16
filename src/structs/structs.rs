use crate::{ChessColors, ChessPieces};

#[derive( Clone, Copy)]
/// For a piece on the board, this struct contains the piece type, the color and the position.
pub struct PositionedChessPiece {
    pub piece: ChessPieces,
    pub color: ChessColors,
    pub position: ChessBoardPosition,
}

#[derive( Clone, Copy)]
pub struct CastlingStateData {
    pub rook_a_moved: bool,
    pub rook_h_moved: bool,
    pub king_moved: bool,
}

#[derive( Clone, Copy)]
pub struct ChessMove {
    pub from: ChessBoardPosition,
    pub to: ChessBoardPosition,
    pub promotion: Option<ChessPieces>,
    pub piece: ChessPieces,
    pub color: ChessColors,
}

use std::fmt;

/// The position of a piece on the chess board. The row is a number between 0 and 7, the column is a number between 0 and 7.
#[derive( PartialEq, Eq, Clone, Copy)]
pub struct ChessBoardPosition {
    pub row: u8,
    pub column: u8,
}

impl fmt::Display for ChessBoardPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", (('a' as u8) + self.column) as char, self.row + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let pos = ChessBoardPosition { row: 0, column: 0 };
        assert_eq!(format!("{}", pos), "a1");
        let pos2 = ChessBoardPosition { row: 7, column: 7 };
        assert_eq!(format!("{}", pos2), "h8");
    }
}