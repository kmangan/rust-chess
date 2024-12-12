use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusty_chess::{initialize_board, make_move, render_chessboard}; // Import from lib.rs
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Debug)]
struct AppState {
    board: Mutex<[[&'static str; 8]; 8]>,
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

async fn make_move_endpoint(
    data: web::Data<AppState>,
    form: web::Form<MoveInput>,
) -> impl Responder {
    let move_notation = form.move_notation.trim();
    if move_notation.len() != 4 {
        return HttpResponse::BadRequest().body("Invalid move format. Use format 'e2e4'.");
    }
    let from = &move_notation[0..2];
    let to = &move_notation[2..4];

    let mut board = data.board.lock().unwrap();
    if make_move(&mut board, from, to).is_err() {
        return HttpResponse::BadRequest().body("Invalid move.");
    }

    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish()
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
            .route("/move", web::post().to(make_move_endpoint))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
