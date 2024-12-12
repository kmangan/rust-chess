use rusty_chess::{initialize_board, make_move, parse_position};

#[test]
fn test_initialize_board_integration() {
    let board = initialize_board();
    assert_eq!(board[0][0], "R"); // Rook at a8
    assert_eq!(board[1][0], "P"); // Pawn at a7
}

#[test]
fn test_parse_position_integration() {
    assert_eq!(parse_position("e2"), (6, 4)); // Row 6, Col 4
}

#[test]
fn test_make_move_integration() {
    let mut board = initialize_board();
    assert!(make_move(&mut board, "e2", "e4").is_ok());
    assert_eq!(board[4][4], "P"); // Pawn at e4
}
