pub fn initialize_board() -> [[&'static str; 8]; 8] {
    [
        ["R", "Kn", "B", "Q", "K", "B", "Kn", "R"], // Black pieces
        ["P"; 8],                                   // Black pawns
        [" "; 8],                                   // Empty row
        [" "; 8],                                   // Empty row
        [" "; 8],                                   // Empty row
        [" "; 8],                                   // Empty row
        ["P"; 8],                                   // White pawns
        ["R", "Kn", "B", "Q", "K", "B", "Kn", "R"], // White pieces
    ]
}

pub fn parse_position(pos: &str) -> (usize, usize) {
    let col = pos.chars().next().unwrap() as usize - 'a' as usize;
    let row = 8 - pos.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
    (row, col)
}

pub fn make_move(
    board: &mut [[&str; 8]; 8],
    from: &str,
    to: &str,
) -> Result<(), &'static str> {
    let (from_row, from_col) = parse_position(from);
    let (to_row, to_col) = parse_position(to);

    if from_row >= 8 || from_col >= 8 || to_row >= 8 || to_col >= 8 {
        return Err("Move out of bounds");
    }

    if board[from_row][from_col] == " " {
        return Err("No piece at the source position");
    }

    board[to_row][to_col] = board[from_row][from_col];
    board[from_row][from_col] = " ";
    Ok(())
}

pub fn render_chessboard(board: &[[&str; 8]; 8]) -> String {
    let mut html = String::from("<table style='border-collapse: collapse;'>");
    for (i, row) in board.iter().enumerate() {
        html.push_str("<tr>");
        for (j, cell) in row.iter().enumerate() {
            let background_color = if (i + j) % 2 == 0 { "#eee" } else { "#333" };
            let text_color = if (i + j) % 2 == 0 { "#000" } else { "#fff" };
            html.push_str(&format!(
                "<td style='width: 50px; height: 50px; text-align: center; background-color: {}; color: {}; border: 1px solid #000;'>{}</td>",
                background_color, text_color, cell
            ));
        }
        html.push_str("</tr>");
    }
    html.push_str("</table>");
    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_board() {
        let board = initialize_board();
        assert_eq!(board[0][0], "R"); // Rook at a8
        assert_eq!(board[1][0], "P"); // Pawn at a7
        assert_eq!(board[7][4], "K"); // King at e1
    }

    #[test]
    fn test_parse_position() {
        assert_eq!(parse_position("e2"), (6, 4)); // Row 6, Col 4
        assert_eq!(parse_position("a8"), (0, 0)); // Row 0, Col 0
    }

    #[test]
    fn test_make_move() {
        let mut board = initialize_board();
        assert!(make_move(&mut board, "e2", "e4").is_ok());
        assert_eq!(board[4][4], "P"); // Pawn at e4
        assert_eq!(board[6][4], " "); // e2 is empty
    }
}
