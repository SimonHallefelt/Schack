use std::{io::Write, sync::{Arc, Mutex}, thread, time::{self, Instant}};

use crate::{board::Board, players::{self, Player}};

pub struct Game {
    board: Board,
    player_1: Player,
    player_2: Player,
    power_balance: f32,
    result: i8,
    id: u64,
}

impl Game {
    pub fn new_game(board: Board) -> Game {
        Game {
            board: board,
            player_1: Player::new(1, 0),
            player_2: Player::new(1, 0),
            power_balance: 0.5, // 0-1
            result: 0,
            id: 0,
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

    pub fn get_power_balance(&self) -> f32{
        self.power_balance
    }

    pub fn get_result(&self) -> i8 {
        self.result
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


pub fn start_game(game: Arc<Mutex<Game>>, player_1: i32, player_2: i32) {
    let mut g = game.lock().unwrap();
    g.board = Board::new_board(1);
    g.player_1 = Player::new(1, player_1 as u8);
    g.player_2 = Player::new(-1, player_2 as u8);
    g.result = 0;
    g.id += 1;
    let id = g.id;
    drop(g);
    thread::spawn(move || {
        run(game, id)
    });
}


fn run(game: Arc<Mutex<Game>>, id: u64) {
    println!("start new game");
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
        g.power_balance = calculate_power_balance(&board);
        drop(g);

        let start_time = Instant::now();
        p_move = movee(Arc::clone(&game), &board, id);
        let elapsed_time = start_time.elapsed();

        let mut g = game.lock().unwrap();
        if g.id != id { break; }
        g.get_active_player().add_time(elapsed_time.as_millis());
        println!("Player {} moves from {:?} to {:?}", player_turn, vec![p_move[0], p_move[1]], vec![p_move[2], p_move[3]]);
        g.result = g.board.update_board(vec![p_move[0], p_move[1]], vec![p_move[2], p_move[3]], p_move[4] as i8);
        println!("move = {}", g.board.get_move_history().last().unwrap());

        if g.result != 0 {
            println!("Game end, result {}", g.result);
            println!("Stats, total moves {}", moves);
            println!("Player white, total time {}ms, slowest move {}ms", g.player_1.get_total_time(), g.player_1.get_slowest_move());
            println!("Player black, total time {}ms, slowest move {}ms", g.player_2.get_total_time(), g.player_2.get_slowest_move());
            if g.result.abs() != 2 {
                add_new_data(g.result, g.player_1.get_player_type(), g.player_2.get_player_type(), g.board.get_move_history());
            }
            break;
        }
    }
}

fn movee(game: Arc<Mutex<Game>>, board: &Board, id: u64) -> Vec<usize> {
    let mut g = game.lock().unwrap();
    let player_turn = g.board.turn;
    let player_type = g.get_active_player().get_player_type();
    drop(g);

    if player_type != 1 { // not human
        return players::run(player_turn, player_type, board, (Vec::new(), 0));
    }

    loop {
        let mut g = game.lock().unwrap();
        if g.id != id { return Vec::new(); }
        let player = g.get_active_player();
        if player.get_clicks().len() != 2 {
            assert!(player.get_clicks().len() < 2);
            drop(g);
            thread::sleep(time::Duration::from_millis(10));
            continue;
        }

        let mut movee = player.get_clicks()[0].clone();
        movee.extend(&player.get_clicks()[1]);
        let promote = player.get_promote_to();
        drop(g);
        let m = players::run(player_turn, player_type, board, (movee, promote));
        if m.is_empty() {
            thread::sleep(time::Duration::from_millis(10));
            continue;
        }
        return m;
    }
}

fn calculate_power_balance(board: &Board) -> f32 {
    let mut p1 = 1;
    let mut p2 = 1;
    for row in board.board.iter() {
        for piece in row.iter() {
            if *piece > 0 {
                p1 += piece_score(*piece);
            } else {
                p2 += piece_score(*piece);
            }
        }
    }
    (p1 as f32) / (p1 + p2) as f32
}

fn piece_score(piece: i8) -> i32 {
    match piece.abs() {
        5 => 7,
        4 => 5,
        3 => 3,
        2 => 3,
        1 => 1,
        _ => 0
    }
}


fn add_new_data(result: i8, player_1: u8, player_2: u8, moves: Vec<String>) {
    let massage = format!("{} {} {} {}\n", result, player_1, player_2, moves.join(" "));
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("src\\AI\\data\\new_raw_data.txt")
        .unwrap();
    //write!(file, "{}", massage).unwrap();
    file.write_all(massage.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _check_legal_game(game: Arc<Mutex<Game>>) {
        loop {
            let g = game.try_lock();
            if g.is_err() {
                thread::sleep(time::Duration::from_millis(1));
                continue;
            }
            let r = g.unwrap().result;
            if r != 0 {
                assert!(r.abs() != 2);
                break;
            }
        }
    }

    // #[test]
    fn _random_vs_random() {
        for _ in 0..100 {
            let game = Arc::new(Mutex::new(Game::new_game(Board::new_board(1))));
            start_game(Arc::clone(&game), 0, 0);
            _check_legal_game(Arc::clone(&game));
        }
    }

    // #[test]
    fn _random_vs_bot() {
        for _ in 0..10 {
            let game = Arc::new(Mutex::new(Game::new_game(Board::new_board(1))));
            start_game(Arc::clone(&game), 0, 2);
            _check_legal_game(Arc::clone(&game));
        }
    }

    // #[test]
    fn _random_vs_bit_bot() {
        for _ in 0..10 {
            let game = Arc::new(Mutex::new(Game::new_game(Board::new_board(1))));
            start_game(Arc::clone(&game), 0, 3);
            _check_legal_game(Arc::clone(&game));
        }
    }

    // #[test]
    fn _bot_vs_bit_bot() {
        for _ in 0..10 {
            let game = Arc::new(Mutex::new(Game::new_game(Board::new_board(1))));
            start_game(Arc::clone(&game), 2, 3);
            _check_legal_game(Arc::clone(&game));
        }
    }
}