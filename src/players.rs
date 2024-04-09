mod random;
mod human;
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

    pub fn get_player_type(&self) -> u8 {
        self.player_type
    }

    pub fn run(&self, board: &Board, movee: (Vec<usize>, usize)) -> Vec<usize> {
        match self.player_type {
            1 => human::run(get_all_legal_moves(&board.board, &board.board_history, self.player, &board.castle_pieces), movee),
            _ => random::run(get_all_legal_moves(&board.board, &board.board_history, self.player, &board.castle_pieces)),
        }
    }
}