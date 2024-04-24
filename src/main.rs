use std::{sync::{Arc, Mutex}, thread, time};

mod board;
mod legal_moves;
mod ui;
mod game;
mod players;

fn main() {
    println!("Hello, world!");
    let board = board::Board::new_board(1);

    // _make_data();

    let game = game::Game::new_game(board);
    println!("program ended: {:?}", ui::start_ui(game));
}

fn _make_data() {
    let mut games = Vec::new();
    for _ in 0..100 {
        let game = Arc::new(Mutex::new(game::Game::new_game(board::Board::new_board(1))));
        game::start_game(Arc::clone(&game), 3, 2);
        games.push(game);
    }
    loop {
        let mut count = 0;
        for game in games.iter() {
            let g = game.try_lock();
            if g.is_err() {
                thread::sleep(time::Duration::from_millis(100));
                break;
            }
            let r = g.unwrap().get_result();
            if r == 0 {
                thread::sleep(time::Duration::from_millis(100));
                break;
            }
            count += 1;
        }
        if count == games.len() {
            break;
        }
    }
}