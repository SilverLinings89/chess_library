mod enum_types;
mod structs;

pub use enum_types::enum_types::*;
pub use structs::structs::*;

#[derive( Clone)]
pub struct ChessBoardState {
    pieces: Vec<PositionedChessPiece>,
    move_counter: i32,
    to_move: ChessColors,
    pub white_castling_state: CastlingStateData,
    black_castling_state: CastlingStateData,
    move_history: Vec<ChessMove>,
    halfmove_clock: i32,
}

impl ChessBoardState {
    fn new() -> ChessBoardState {
        ChessBoardState {
            pieces: vec![
                PositionedChessPiece {
                    piece: ChessPieces::King,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 0, column: 4 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Queen,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 0, column: 3 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Rook,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 0, column: 0 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Rook,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 0, column: 7 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Bishop,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 0, column: 2 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Bishop,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 0, column: 5 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Knight,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 0, column: 1 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Knight,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 0, column: 6 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 1, column: 0 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 1, column: 1 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 1, column: 2 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 1, column: 3 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 1, column: 4 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 1, column: 5 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 1, column: 6 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::White,
                    position: ChessBoardPosition { row: 1, column: 7 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::King,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 7, column: 4 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Queen,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 7, column: 3 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Rook,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 7, column: 0 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Rook,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 7, column: 7 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Bishop,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 7, column: 2 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Bishop,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 7, column: 5 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Knight,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 7, column: 1 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Knight,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 7, column: 6 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 6, column: 0 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 6, column: 1 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 6, column: 2 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 6, column: 3 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 6, column: 4 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 6, column: 5 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 6, column: 6 },
                },
                PositionedChessPiece {
                    piece: ChessPieces::Pawn,
                    color: ChessColors::Black,
                    position: ChessBoardPosition { row: 6, column: 7 },
                },
            ],
            move_counter: 0,
            to_move: ChessColors::White,
            white_castling_state: CastlingStateData {
                rook_a_moved: false,
                rook_h_moved: false,
                king_moved: false,
            },
            black_castling_state: CastlingStateData {
                rook_a_moved: false,
                rook_h_moved: false,
                king_moved: false,
            },
            move_history: vec![],
            halfmove_clock: 0,
        }
    }

    fn get_captured_piece_for_move(&self, next_move: ChessMove) -> Option<ChessPieces> {
        for piece in &self.pieces {
            if piece.position.row == next_move.to.row && piece.position.column == next_move.to.column {
                return Some(piece.piece);
            }
        }
        None
    }

    fn get_piece_index_by_position(&self, position: ChessBoardPosition) -> Option<usize> {
        for (index, piece) in self.pieces.iter().enumerate() {
            if piece.position.row == position.row && piece.position.column == position.column {
                return Some(index);
            }
        }
        None
    }
    
    fn force_perform_shallow_move(&self, next_move: ChessMove) -> ChessBoardState {
        let mut new_state = self.clone();
        let captured_piece = self.get_captured_piece_for_move(next_move);
        if captured_piece.is_some() {
            let captured_index = self.get_piece_index_by_position(next_move.to);
            if captured_index.is_some() {
               new_state.pieces.remove(captured_index.unwrap());
            }
        }
        for piece in &mut new_state.pieces {
            if piece.position.row == next_move.from.row && piece.position.column == next_move.from.column && piece.piece == next_move.piece && piece.color == next_move.color {
                piece.position = next_move.to;
                break;
            }
        }
        new_state
    }

    fn perform_move(&mut self, next_move: ChessMove) -> bool {
        if !self.is_move_valid(next_move) {
            return false;
        }
        let captured_piece = self.get_captured_piece_for_move(next_move);
        if captured_piece.is_some() {
            let captured_index = self.get_piece_index_by_position(next_move.to);
            match captured_index {
                Some(index) => self.pieces.remove(index),
                None => return false,
            };
        }
        for piece in &mut self.pieces {
            if piece.position.row == next_move.from.row && piece.position.column == next_move.from.column && piece.piece == next_move.piece && piece.color == next_move.color {
                piece.position = next_move.to;
                break;
            }
        }
        if next_move.piece == ChessPieces::Pawn && (next_move.to.row == 0 || next_move.to.row == 7) {
            let promotion_piece = match next_move.promotion {
                Some(piece) => piece,
                None => ChessPieces::Queen,
            };
            for piece in &mut self.pieces {
                if piece.position == next_move.to && piece.piece == ChessPieces::Pawn && piece.color == next_move.color {
                    piece.piece = promotion_piece;
                    break;
                }
            }
        }
        self.move_counter += 1;
        self.to_move = match self.to_move {
            ChessColors::White => ChessColors::Black,
            ChessColors::Black => ChessColors::White,
        };
        self.move_history.push(next_move);
       self.update_half_move_clock(next_move.piece, captured_piece.is_some());
        self.update_castling_state_for_move(next_move);
        true
    }

    fn update_half_move_clock(&mut self, piece: ChessPieces, move_was_capture: bool) {
        self.halfmove_clock += 1;
        if piece == ChessPieces::Pawn {
            self.halfmove_clock = 0;
            return;
        }
        if move_was_capture {
            self.halfmove_clock = 0;
        }
    }

    fn update_castling_state_for_move(&mut self, next_move: ChessMove) {
        if next_move.color == ChessColors::White {
            if next_move.piece == ChessPieces::King {
                self.white_castling_state.king_moved = true;
            } else if next_move.piece == ChessPieces::Rook {
                if next_move.from.row == 0 && next_move.from.column == 0 {
                    self.white_castling_state.rook_a_moved = true;
                } else if next_move.from.row == 0 && next_move.from.column == 7 {
                    self.white_castling_state.rook_h_moved = true;
                }
            }
        } else {
            if next_move.piece == ChessPieces::King {
                self.black_castling_state.king_moved = true;
            } else if next_move.piece == ChessPieces::Rook {
                if next_move.from.row == 7 && next_move.from.column == 0 {
                    self.black_castling_state.rook_a_moved = true;
                } else if next_move.from.row == 7 && next_move.from.column == 7 {
                    self.black_castling_state.rook_h_moved = true;
                }
            }
        }
    }

    fn get_king_position(&self, side: ChessColors) -> Option<ChessBoardPosition> {
        for piece in &self.pieces {
            if piece.piece == ChessPieces::King && piece.color == side {
                return Some(piece.position);
            }
        }
        None
    }

    fn is_connection_empty(&self, position_1: &ChessBoardPosition, &position_2: &ChessBoardPosition) -> bool {
        let row_diff = (position_1.row as i32 - position_2.row as i32).abs();
        let col_diff = (position_1.column as i32 - position_2.column as i32).abs();
        if row_diff == 0 {
            for piece in &self.pieces {
                if piece.position.row == position_1.row && piece.position.column > position_1.column && piece.position.column < position_2.column {
                    return false;
                }
            }
        } else if col_diff == 0 {
            for piece in &self.pieces {
                if piece.position.column == position_1.column && piece.position.row > position_1.row && piece.position.row < position_2.row {
                    return false;
                }
            }
        } else if row_diff == col_diff {
            for piece in &self.pieces {
                if (piece.position.row as i32 - position_1.row as i32).abs() == (piece.position.column as i32 - position_1.column as i32).abs() && piece.position.row > position_1.row && piece.position.row < position_2.row {
                    return false;
                }
            }
        }
        true
    }

    fn is_in_check(&self, side: ChessColors) ->bool {
        let king_position = self.get_king_position(side).expect("No king found");
        for piece in &self.pieces {
            if piece.color != side {
                match piece.piece {
                    ChessPieces::King => {
                        if (king_position.row as i32 - piece.position.row as i32).abs() <= 1 && (king_position.column as i32 - piece.position.column as i32).abs() <= 1 {
                            return true;
                        }
                    },
                    ChessPieces::Queen => {
                        if (king_position.row as i32 - piece.position.row as i32).abs() == (king_position.column as i32 - piece.position.column as i32).abs() {
                            if self.is_connection_empty(&king_position, &piece.position) {
                                return true;
                            }
                        }
                        if king_position.row == piece.position.row || king_position.column == piece.position.column {
                            if self.is_connection_empty(&king_position, &piece.position) {
                                return true;
                            }
                        }
                    },
                    ChessPieces::Rook => {
                        if king_position.row == piece.position.row || king_position.column == piece.position.column {
                            if self.is_connection_empty(&king_position, &piece.position) {
                                return true;
                            }
                        }
                    },
                    ChessPieces::Bishop => {
                        if (king_position.row as i32 - piece.position.row as i32).abs() == (king_position.column as i32 - piece.position.column as i32).abs() {
                            if self.is_connection_empty(&king_position, &piece.position) {
                                return true;
                            }
                        }
                    },
                    ChessPieces::Knight => {
                        if (king_position.row as i32 - piece.position.row as i32).abs() == 2 && (king_position.column as i32 - piece.position.column as i32).abs() == 1 {
                            return true;
                        }
                        if (king_position.row as i32 - piece.position.row as i32).abs() == 1 && (king_position.column as i32 - piece.position.column as i32).abs() == 2 {
                            return true;
                        }
                    },
                    ChessPieces::Pawn => {
                        if piece.color == ChessColors::White {
                            if king_position.row as i32 - piece.position.row as i32 == 1 && (king_position.column as i32 - piece.position.column as i32).abs() == 1 {
                                return true;
                            }
                        } else {
                            if king_position.row as i32 - piece.position.row as i32 == -1 && (king_position.column as i32 - piece.position.column as i32).abs() == 1 {
                                return true;
                            }
                        }
                    },
                
                }
            }
        }
        false
    }

    fn is_move_valid(&self, next_move: ChessMove) -> bool {
        let mut piece_found = false;
        for piece in &self.pieces {
            if piece.position.row == next_move.from.row && piece.position.column == next_move.from.column && piece.piece == next_move.piece && piece.color == next_move.color {
                piece_found = true;
                if piece.color != next_move.color || piece.piece != next_move.piece {
                    return false;
                }
                break;
            }
        }
        if !piece_found {
            return false;
        }
        if self.is_in_check(next_move.color) {
            let updated_temp_state = self.force_perform_shallow_move(next_move);
            if updated_temp_state.is_in_check(next_move.color) {
                return false;
            }
        }
        true
    }

    fn get_en_passant_target(&self) -> String {
        if self.move_history.len() == 0 {
            return "-".to_string();
        }
        let ret = self.move_history.last().map(|last_move| {
            if last_move.piece == ChessPieces::Pawn && (last_move.from.row as i32 - last_move.to.row as i32).abs() == 2 {
                let row = (last_move.from.row + last_move.to.row) / 2 + 1;
                let col: char = (('a' as u8 )+ last_move.to.column) as char;
                format!("{}{}", col, row)
            } else {
                "-".to_string()
            }
        });
        if ret.is_none() {
            return "-".to_string();
        }
        ret.unwrap()
    }

    fn to_fen(&self) -> String {
    let mut fen = String::new();
    let mut empty_counter = 0;
    for row in (0..8).rev() {
        for column in 0..8 {
            let mut found = false;
            for piece in &self.pieces {
                if piece.position.row == row && piece.position.column == column {
                    found = true;
                    if empty_counter > 0 {
                        fen.push_str(&empty_counter.to_string());
                        empty_counter = 0;
                    }
                    fen.push_str(match piece.piece {
                        ChessPieces::King => if piece.color == ChessColors::White {"K"}  else { "k"},
                        ChessPieces::Queen => if piece.color == ChessColors::White {"Q"}  else { "q"},
                        ChessPieces::Rook => if piece.color == ChessColors::White {"R"}  else { "r"},
                        ChessPieces::Bishop =>  if piece.color == ChessColors::White {"B"}  else { "b"},
                        ChessPieces::Knight => if piece.color == ChessColors::White {"N"}  else { "n"},
                        ChessPieces::Pawn => if piece.color == ChessColors::White {"P"}  else { "p"},
                    });
                    break;
                }
            }
            if !found {
                empty_counter += 1;
            }
        }
        if empty_counter > 0 {
            fen.push_str(&empty_counter.to_string());
            empty_counter = 0;
        }
        if row > 0 {
            fen.push_str("/");
        }
    }
    fen.push_str(" ");
    fen.push_str(match self.to_move {
        ChessColors::White => "w",
        ChessColors::Black => "b",
    });
    fen.push_str(" ");
    if !self.white_castling_state.king_moved {
        fen.push_str(match self.white_castling_state.rook_a_moved {
            false => "K",
            true => "",
        });
        fen.push_str(match self.white_castling_state.rook_h_moved {
            false => "Q",
            true => "",
        });
    }
    if !self.black_castling_state.king_moved {
        fen.push_str(match self.black_castling_state.rook_a_moved {
            false => "k",
            true => "",
        });
        fen.push_str(match self.black_castling_state.rook_h_moved {
            false => "q",
            true => "",
        });
    }
    fen.push_str(" ");
    fen.push_str(self.get_en_passant_target().as_str());
    fen.push_str(" ");
    fen.push_str(&self.halfmove_clock.to_string());
    fen.push_str(" ");
    fen.push_str(&self.move_counter.to_string());
    fen
    }
}

fn main() {
    
    let mut board = ChessBoardState::new();
    println!("{}", board.to_fen());
    board.perform_move(ChessMove {
        from: ChessBoardPosition { row: 1, column: 4 },
        to: ChessBoardPosition { row: 3, column: 4 },
        promotion: None,
        piece: ChessPieces::Pawn,
        color: ChessColors::White,
    });
    println!("{}", board.to_fen());
}
