use std::{sync::{Arc, Mutex}, thread, time};

use crate::board::{self, Board};
use crate::players;

pub struct Game {
    running: bool,
    board: board::Board,
    clicks: Vec<Vec<usize>>,
    promote_to: usize,
}

impl Game {
    pub fn new_game(board: board::Board) -> Game {
        Game {
            running: false,
            board: board,
            clicks: Vec::new(),
            promote_to: 0,
        }
    }

    pub fn get_board(&self) -> Vec<Vec<i8>> {
        self.board.board.clone()
    }

    pub fn clicked(&mut self, click: Vec<usize>) {
        if self.clicks.len() == 2 {
            self.clicks[0] = self.clicks.pop().unwrap();
        }
        self.clicks.push(click);
    }
}


pub fn start_game(game: Arc<Mutex<Game>>, player_1: i32, player_2: i32){
    let mut g = game.try_lock().unwrap();
    if g.running {
        println!("game is already running");
        return;
    }
    g.board = board::Board::new_board(1);
    drop(g);
    thread::spawn(move || {
        run(game, player_1 as u8, player_2 as u8)
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
            // p_move = player_1.run(&board);
            p_move = movee(Arc::clone(&game), &player_1, &board);
        } else {
            // p_move = player_2.run(&board);
            p_move = movee(Arc::clone(&game), &player_2, &board);
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

fn movee(game: Arc<Mutex<Game>>, player: &players::Player, board: &Board) -> Vec<usize> {
    if player.get_player_type() != 1 { // not human
        return player.run(board, (Vec::new(), 0));
    }

    loop {
        let mut g = game.lock().unwrap();
        if g.clicks.len() != 2 {
            assert!(g.clicks.len() < 2);
            drop(g);
            thread::sleep(time::Duration::from_millis(10));
            continue;
        }

        // if {
        // when a pawn gets promoted
        // wait for a promotion target
        // }

        let mut movee = g.clicks[0].clone();
        movee.extend(&g.clicks[1]);
        let m = player.run(board, (movee, g.promote_to));
        println!("hej, movee, m = {:?}", m);
        if m.is_empty() {
            drop(g);
            thread::sleep(time::Duration::from_millis(10));
            continue;
        }
        g.clicks = Vec::new();
        return m;
    }
}