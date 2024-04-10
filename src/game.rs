use std::{sync::{Arc, Mutex}, thread, time};

use crate::{board::{self, Board}, players::Player};
use crate::players;

pub struct Game {
    running: bool,
    board: board::Board,
    player_1: Player,
    player_2: Player,
}

impl Game {
    pub fn new_game(board: board::Board) -> Game {
        Game {
            running: false,
            board: board,
            player_1: players::Player::new(1, 0),
            player_2: players::Player::new(1, 0),
        }
    }

    pub fn get_board(&self) -> Vec<Vec<i8>> {
        self.board.board.clone()
    }

    fn get_active_player(&mut self) -> &mut Player {
        if self.board.turn == 1 {
            &mut self.player_1
        } else {
            &mut self.player_2
        }
    }

    pub fn clicked(&mut self, click: Vec<usize>) {
        self.get_active_player().clicked(click);
    }

    pub fn set_promote(&mut self, player: i32, promote_to: usize) {
        if player == 1 {
            self.player_1.set_promote_to(promote_to)
        } else {
            self.player_2.set_promote_to(promote_to)
        }
    }
}


pub fn start_game(game: Arc<Mutex<Game>>, player_1: i32, player_2: i32){
    let mut g = game.try_lock().unwrap();
    if g.running {
        println!("game is already running");
        return;
    }
    g.board = board::Board::new_board(1);
    g.player_1 = players::Player::new(1, player_1 as u8);
    g.player_2 = players::Player::new(-1, player_2 as u8);
    drop(g);
    thread::spawn(move || {
        run(game)
    });
}


fn run(game: Arc<Mutex<Game>>) {
    println!("start new game");
    let mut result;
    let mut moves = 0;
    let mut player_turn;
    let mut board;
    let mut p_move;

    loop {
        moves += 1;

        let mut g = game.lock().unwrap();
        player_turn = g.board.turn;
        board = g.board.clone();
        g.get_active_player().clear_clicks();
        drop(g);

        p_move = movee(Arc::clone(&game), &board);
        // thread::sleep(time::Duration::from_millis(1));

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

fn movee(game: Arc<Mutex<Game>>, board: &Board) -> Vec<usize> {
    let mut g = game.lock().unwrap();
    if g.get_active_player().get_player_type() != 1 { // not human
        return g.get_active_player().run(board, (Vec::new(), 0));
    }
    drop(g);

    loop {
        let mut g = game.lock().unwrap();
        let player = g.get_active_player();
        if player.get_clicks().len() != 2 {
            assert!(player.get_clicks().len() < 2);
            drop(g);
            thread::sleep(time::Duration::from_millis(10));
            continue;
        }

        // if {
        // when a pawn gets promoted
        // wait for a promotion target
        // }

        let mut movee = player.get_clicks()[0].clone();
        movee.extend(&player.get_clicks()[1]);
        let m = player.run(board, (movee, player.get_promote_to()));
        if m.is_empty() {
            drop(g);
            thread::sleep(time::Duration::from_millis(10));
            continue;
        }
        return m;
    }
}