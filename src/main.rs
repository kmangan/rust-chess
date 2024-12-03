use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Debug)]
struct AppState {
    board: Mutex<[[&'static str; 8]; 8]>, // Chessboard state
}

fn initialize_board() -> [[&'static str; 8]; 8] {
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

fn render_chessboard(board: &[[&str; 8]; 8]) -> String {
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

#[derive(Deserialize)]
struct MoveInput {
    move_notation: String,
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let board = data.board.lock().unwrap();
    let chessboard_html = render_chessboard(&*board);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "<!DOCTYPE html>
            <html>
            <head>
                <title>Rust Chess</title>
            </head>
            <body>
                <h1>Rust Chess Game</h1>
                <form action='/move' method='post'>
                    <input type='text' name='move_notation' placeholder='Enter move (e.g., e2e4)' required>
                    <button type='submit'>Make Move</button>
                </form>
                {}
            </body>
            </html>",
            chessboard_html
        ))
}

async fn make_move(
    data: web::Data<AppState>,
    form: web::Form<MoveInput>,
) -> impl Responder {
    let move_notation = form.move_notation.trim();

    // Simplified parsing of algebraic notation (e.g., "e2e4")
    if move_notation.len() != 4 {
        return HttpResponse::BadRequest().body("Invalid move format. Use format 'e2e4'.");
    }
    let from = &move_notation[0..2];
    let to = &move_notation[2..4];

    let mut board = data.board.lock().unwrap();
    let (from_row, from_col) = parse_position(from);
    let (to_row, to_col) = parse_position(to);

    // Move the piece (no validation for now)
    board[to_row][to_col] = board[from_row][from_col];
    board[from_row][from_col] = " ";

    HttpResponse::SeeOther()
        .header("Location", "/")
        .finish()
}

// Helper to parse positions like "e2" into (row, col)
fn parse_position(pos: &str) -> (usize, usize) {
    let col = pos.chars().next().unwrap() as usize - 'a' as usize;
    let row = 8 - pos.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
    (row, col)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        board: Mutex::new(initialize_board()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .route("/move", web::post().to(make_move))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
