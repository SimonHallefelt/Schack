mod random;
mod human;
mod bot;
use crate::legal_moves::get_all_legal_moves;
use crate::board::Board;

pub struct Player {
    player: i8,
    player_type: u8,
    clicks: Vec<Vec<usize>>,
    promote_to: usize,
}

impl Player {
    pub fn new(player: i8, player_type: u8) -> Player {
        Player {
            player: player,
            player_type: player_type,
            clicks: Vec::new(),
            promote_to: 5, // should be '0' as default
        }
    }

    pub fn get_player_type(&self) -> u8 {
        self.player_type
    }

    pub fn get_promote_to(&self) -> usize {
        self.promote_to
    }

    pub fn set_promote_to(&mut self, promote_to: usize) {
        self.promote_to = promote_to;
    }

    pub fn get_clicks(&self) -> &Vec<Vec<usize>> {
        &self.clicks
    }

    pub fn clicked(&mut self, click: Vec<usize>) {
        if self.clicks.len() == 2 {
            self.clicks[0] = self.clicks.pop().unwrap();
        }
        self.clicks.push(click);
    }

    pub fn clear_clicks(&mut self) {
        self.clicks = Vec::new();
    }

    pub fn run(&self, board: &Board, movee: (Vec<usize>, usize)) -> Vec<usize> {
        match self.player_type {
            2 => bot::run(),
            1 => human::run(get_all_legal_moves(&board.board, &board.board_history, self.player, &board.castle_pieces), movee),
            _ => random::run(get_all_legal_moves(&board.board, &board.board_history, self.player, &board.castle_pieces)),
        }
    }
}