mod random;
use crate::legal_moves::get_all_legal_moves;
use crate::board::Board;

pub struct Player {
    player: i8,
    player_type: u8,
}

impl Player {
    pub fn new(player: i8, player_type: u8) -> Player {
        Player {
            player: player,
            player_type: player_type,
        }
    }

    pub fn run(&self, board: &Board) -> Vec<usize> {
        match self.player_type {
            _ => random::run(get_all_legal_moves(&board.board, &board.board_history, self.player, &board.castle_pieces)),
        }
    }
}