use std::sync::{Arc, Mutex};
use rand::seq::SliceRandom;

use crate::board;
use crate::legal_moves::get_all_legal_moves;

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

    pub fn get_running(&self) -> bool {
        self.running
    }
}


pub fn start_game(game: Arc<Mutex<Game>>){
    let mut g = game.lock().unwrap();
    if g.running {
        println!("game is already running");
        return;
    }
    g.board = board::Board::new_board(1);
    drop(g);
    run(game)
}


fn run(game: Arc<Mutex<Game>>) {
    let mut result = 0;
    let mut moves = 0;
    let mut player_turn;
    let mut board;
    let mut board_history;
    let mut castle_pieces;

    loop {
        let g = game.lock().unwrap();
        player_turn = g.board.turn;
        board = g.board.board.clone();
        board_history = g.board.board_history.clone();
        castle_pieces = g.board.castle_pieces.clone();
        drop(g);

        moves += 1;
        let legal_moves = get_all_legal_moves(&board, &board_history, player_turn, &castle_pieces);
        if legal_moves.len() == 0 {
            println!("Error, No legal moves for player {}", player_turn);
            break;
        }
        let mut move_made = false;
        while !move_made {
            println!("Player {}'s turn, Legal moves: {:?}", player_turn, legal_moves);
            let mut rng = rand::thread_rng();
            let input = legal_moves.choose(&mut rng).unwrap().to_vec();
            if legal_moves.contains(&input) {
                let mut g = game.lock().unwrap();
                println!("Player {} moves from {:?} to {:?}", player_turn, vec![input[0], input[1]], vec![input[2], input[3]]);
                let promote_to = *vec![2,3,4,5].choose(&mut rng).unwrap(); 
                result = g.board.update_board(vec![input[0], input[1]], vec![input[2], input[3]], promote_to);
                move_made = true;
            } else {
                println!("Illegal move");
            }
        }
        if result != 0 {
            println!("Game end, result {}", result);
            println!("Stats, total moves {}", moves);
            break;
        }
        player_turn *= -1;
    }
}