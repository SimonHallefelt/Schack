use std::{sync::{Arc, Mutex}, thread};

use crate::board;
use crate::players;

pub struct Game {
    running: bool,
    board: board::Board,
}

impl Game {
    pub fn new_game(board: board::Board) -> Game {
        Game {
            running: false,
            board: board,
        }
    }

    pub fn get_board(&self) -> Vec<Vec<i8>> {
        self.board.board.clone()
    }
}


pub fn start_game(game: Arc<Mutex<Game>>, player_1: u8, player_2: u8){
    let mut g = game.lock().unwrap();
    if g.running {
        println!("game is already running");
        return;
    }
    g.board = board::Board::new_board(1);
    drop(g);
    thread::spawn(move || {
        run(game, player_1, player_2)
    });
}


fn run(game: Arc<Mutex<Game>>, p1: u8, p2: u8) {
    let mut result;
    let mut moves = 0;
    let mut player_turn;
    let mut board;
    let mut p_move;
    let player_1 = players::Player::new(1, p1);
    let player_2 = players::Player::new(-1, p2);

    loop {
        moves += 1;

        let g = game.lock().unwrap();
        player_turn = g.board.turn;
        board = g.board.clone();
        drop(g);

        if player_turn == 1 {
            p_move = player_1.run(&board);
        } else {
            p_move = player_2.run(&board);
        }
        
        let mut g = game.lock().unwrap();
        println!("Player {} moves from {:?} to {:?}", player_turn, vec![p_move[0], p_move[1]], vec![p_move[2], p_move[3]]);
        result = g.board.update_board(vec![p_move[0], p_move[1]], vec![p_move[2], p_move[3]], p_move[4] as i8);

        if result != 0 {
            println!("Game end, result {}", result);
            println!("Stats, total moves {}", moves);
            break;
        }
    }
}