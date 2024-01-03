use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ChessPiece {
    piece_type: PieceType,
    color: Color,
}

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_symbol = match self.piece_type {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        };

        let color_symbol = match self.color {
            Color::White => 'w',
            Color::Black => 'b',
        };

        write!(f, "{}{}", color_symbol, piece_symbol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ChessSquare {
    piece: Option<ChessPiece>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ChessBoard {
    squares: [[ChessSquare; 8]; 8],
}

fn parse_move(mv: &str) -> Result<((usize, usize), (usize, usize)), &str> {
    // Parse the move string, e.g., "e2e4" to ((4, 6), (4, 4))
    if mv.len() != 4 {
        return Err("Invalid move format");
    }

    let file_from = mv.chars().nth(0).unwrap() as usize - 'a' as usize;
    let rank_from = 8 - mv.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;

    let file_to = mv.chars().nth(2).unwrap() as usize - 'a' as usize;
    let rank_to = 8 - mv.chars().nth(3).unwrap().to_digit(10).unwrap() as usize;

    Ok(((file_from, rank_from), (file_to, rank_to)))
}

impl ChessBoard {
    fn new() -> ChessBoard {
        let mut board = ChessBoard { squares: [[ChessSquare { piece: None }; 8]; 8] };
        board.initialize_pieces();
        board
    }

    fn initialize_pieces(&mut self) {
        // Place pawns for both colors
        for (rank, color) in [(1, Color::White), (6, Color::Black)] {
            for file in 0..8 {
                self.squares[rank][file].piece = Some(ChessPiece {
                    piece_type: PieceType::Pawn,
                    color,
                });
            }
        }

        // Place other pieces for both colors
        for (rank, color) in [(0, Color::White), (7, Color::Black)] {
            let piece_order = [
                PieceType::Rook,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Queen,
                PieceType::King,
                PieceType::Bishop,
                PieceType::Knight,
                PieceType::Rook,
            ];

            for (file, &piece_type) in (0..8).zip(piece_order.iter()) {
                self.squares[rank][file].piece = Some(ChessPiece {
                    piece_type,
                    color,
                });
            }
        }
    }

    fn display(&self) {
        for row in &self.squares {
            for square in row {
                if let Some(piece) = square.piece {
                    print!("{:<3}", piece);
                } else {
                    print!("  ");
                }
            }
            println!();
        }
    }

    fn validate_rook_move(&self, from: (usize, usize), to: (usize, usize)) -> Result<(), &str> {
        // Check if the move is either horizontally or vertically
        if from.0 != to.0 && from.1 != to.1 {
            return Err("Rooks can only move horizontally or vertically");
        }

        // Define the range and step based on the move direction
        let (start, end, step) = if from.0 == to.0 {
            // Horizontal move
            (from.1.min(to.1), from.1.max(to.1), 1)
        } else {
            // Vertical move
            (from.0.min(to.0), from.0.max(to.0), 1)
        };

        // Check if there are pieces in the way along the path of the move
        for i in start + step..end {
            if self.squares[i][from.0].piece.is_some() {
                return Err("There is a piece in the way");
            }
        }

        Ok(())
    }

    fn validate_pawn_move(&self, from: (usize, usize), to: (usize, usize)) -> Result<(), &str> {
        let piece = match self.squares.get(from.1).and_then(|row| row.get(from.0)).and_then(|square| square.piece) {
            Some(piece) => piece,
            None => return Err("No piece at the specified starting square"),
        };

        // Check if the pawn is moving within its file
        if from.0 != to.0 {
            return Err("Pawns cannot move horizontally");
        }

        // Check if the pawn is moving forward
        let direction = match piece.color {
            Color::White => 1,
            Color::Black => -1,
        };

        let rank_difference = (to.1 as isize - from.1 as isize) * direction;
        let file_difference = (to.0 as isize - from.0 as isize).abs();

        match (rank_difference, file_difference) {
            (1, 0) => {
                // Valid one square forward move
                if self.squares[to.1][to.0].piece.is_some() {
                    return Err("Invalid move: destination square is occupied");
                }
            }
            (2, 0) if from.1 == 1 && self.squares[from.1 + direction][from.0].piece.is_none() && self.squares[to.1][to.0].piece.is_none() => {
                // Valid two squares forward move from starting square
            }
            (1, 1) if self.squares[to.1][to.0].piece.is_some() && self.squares[to.1][to.0].piece.unwrap().color != piece.color => {
                // Valid diagonal capture
            }
            _ => return Err("Invalid pawn move"),
        }

        Ok(())
    }

    fn validate_knight_move(&self, from: (usize, usize), to: (usize, usize)) -> Result<(), &str> {
        let piece = match self.squares.get(from.1).and_then(|row| row.get(from.0)).and_then(|square| square.piece) {
            Some(piece) => piece,
            None => return Err("No piece at the specified starting square"),
        };

        // Check if the move is an L-shape
        let dx = (from.0 as isize - to.0 as isize).abs();
        let dy = (from.1 as isize - to.1 as isize).abs();

        if (dx == 1 && dy == 2) || (dx == 2 && dy == 1) {
            // Valid knight move
            if let Some(dest_piece) = self.squares.get(to.1).and_then(|row| row.get(to.0)).and_then(|square| square.piece) {
                if dest_piece.color == piece.color {
                    return Err("Cannot capture own piece");
                }
            }

            Ok(())
        } else {
            Err("Invalid move for a knight")
        }
    }

    fn validate_bishop_move(&self, from: (usize, usize), to: (usize, usize)) -> Result<(), &str> {
        let piece = match self.squares.get(from.1).and_then(|row| row.get(from.0)).and_then(|square| square.piece) {
            Some(piece) => piece,
            None => return Err("No piece at the specified starting square"),
        };

        // Check if the move is along a diagonal path
        let dx = (from.0 as isize - to.0 as isize).abs();
        let dy = (from.1 as isize - to.1 as isize).abs();

        if dx != dy {
            return Err("Invalid move for a bishop");
        }

        // Check if there is a piece at the destination square and it is not of the same color
        if let Some(dest_piece) = self.squares.get(to.1).and_then(|row| row.get(to.0)).and_then(|square| square.piece) {
            if dest_piece.color == piece.color {
                return Err("Cannot capture own piece");
            }
        }

        // Check for pieces in the way along the diagonal path
        let rank_range = (from.1 + 1)..to.1; // Exclusive range

        for (rank, file) in rank_range.zip(from.0 + 1..to.0) {
            if let Some(square) = self.squares.get(rank).and_then(|row| row.get(file)) {
                if square.piece.is_some() {
                    return Err("There is a piece in the way");
                }
            }
        }

        Ok(())
    }

    fn validate_move(&self, from: (usize, usize), to: (usize, usize)) -> Result<(), &str> {
        // Check if there is a piece at the starting square
        let from_piece = self.squares.get(from.1).and_then(|row| row.get(from.0)).and_then(|square| square.piece);

        if from_piece.is_none() {
            return Err("No piece at the specified starting square");
        }

        // Validate the move based on the piece type
        match from_piece.unwrap().piece_type {
            PieceType::Pawn => self.validate_pawn_move(from, to),
            PieceType::Rook => self.validate_rook_move(from, to),
            PieceType::Knight => self.validate_knight_move(from, to),
            PieceType::Bishop => self.validate_bishop_move(from, to),
            // Add cases for other piece types as needed
            _ => Err("Invalid move for the given piece type"),
        }
    }

    fn perform_move(squares: &mut [[ChessSquare; 8]; 8], from: (usize, usize), to: (usize, usize)) {
        // Move the piece to the destination square
        let moved_piece = squares[from.1][from.0].piece.take();
        squares[to.1][to.0].piece = moved_piece;
    }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut chess_board = ChessBoard::new();
    chess_board.display();

    // Specify the file path with the moves
    let file_path = "src/moves.txt";

    if let Ok(file) = File::open(file_path) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(move_str) = line {
                match parse_move(&move_str) {
                    Ok((from, to)) => {
                        match chess_board.validate_move(from, to) {
                            Ok(_) => {
                                println!("Move '{}' is valid. Performing the move.", move_str);
                                ChessBoard::perform_move(&mut chess_board.squares, from, to);
                                chess_board.display();
                            }
                            Err(err) => println!("Invalid move '{}': {}", move_str, err),
                        }
                    }
                    Err(err) => println!("Invalid move format in line '{}': {}", move_str, err),
                }
            }
        }
    } else {
        println!("Error opening the file at path: {}", file_path);
    }
}
