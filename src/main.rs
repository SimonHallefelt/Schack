use std::sync::{Arc, Mutex};

mod board;
mod legal_moves;
mod ui;
mod game;

fn main() {
    println!("Hello, world!");
    let board = board::Board::new_board(1);

    let game = game::Game::new_game(board.clone());
    game::start_game(Arc::new(Mutex::new(game)));

    let game = game::Game::new_game(board);
    println!("program ended: {:?}", ui::start_ui(game));
}