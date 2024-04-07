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

    pub fn get_board(self) -> board::Board {
        return self.board;
    }

    pub fn run(self) {
        if self.running {
            println!("game is already running");
            return;
        }
        run(self.board)
    }
}


fn run(mut board: board::Board) {
    let mut result = 0;
    let mut moves = 0;
    loop {
        moves += 1;
        if moves == 20 {
            //break;
        }
        let player = board.turn;
        // board.print_board();
        let legal_moves = get_all_legal_moves(&board.board, board.board_history.clone(), player, &board.castle_pieces);
        if legal_moves.len() == 0 {
            println!("Error, No legal moves for player {}", player);
            break;
        }
        let mut move_made = false;
        while !move_made {
            println!("Player {}'s turn, Legal moves: {:?}", player, legal_moves);
            let mut rng = rand::thread_rng();
            let input = legal_moves.choose(&mut rng).unwrap().to_vec();
            if legal_moves.contains(&input) {
                println!("Player {} moves from {:?} to {:?}", player, vec![input[0], input[1]], vec![input[2], input[3]]);
                let promote_to = *vec![2,3,4,5].choose(&mut rng).unwrap(); 
                result = board.update_board(vec![input[0], input[1]], vec![input[2], input[3]], promote_to);
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
    }
}