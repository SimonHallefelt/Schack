
/*
    0 - empty
    1 - pawn
    2 - knight
    3 - bishop
    4 - rook
    5 - queen
    6 - king
 */

#[derive(Clone)]
pub struct Board {
    board: Vec<Vec<i8>>,
    board_history: Vec<Vec<Vec<i8>>>,
    turn: i8,
}

impl Board {

    pub fn new_board(starting_player: i8) -> Board {
        let mut board = vec![vec![0; 8]; 8];
        board[0] = vec![4,3,2,6,5,2,3,4];
        board[1] = vec![1; 8];
        board[6] = vec![-1; 8];
        board[7] = vec![-4,-3,-2,-6,-5,-2,-3,-4];
        print_board(&board);

        Board {
            board: board,
            board_history: vec![],
            turn: starting_player,
        }
    }

    pub fn update_board(&mut self, start: Vec<usize>, end: Vec<usize>, _promote_to: i8) -> i8 {
        let player = self.board[start[0]][start[1]] / self.board[start[0]][start[1]].abs();
        let b = self.board.clone();
        if !legal_move(self, &start, &end, player) {
            println!("illegal move");
            return player * 2 * -1
        }
        self.board_history.push(b);
        self.board[end[0]][end[1]] = self.board[start[0]][start[1]];
        self.board[start[0]][start[1]] = 0;
        self.turn *= -1;
        if won(&self.board) {
            return player;
        }
        0
    }

}

fn legal_move(board: &mut Board, start: &Vec<usize>, end: &Vec<usize>, player: i8) -> bool {
    if board.turn != player {
        return false;
    }
    for i in vec![start[0],start[1],end[0], end[1]] {
        if i > 7 {
            return false;
        }
    }
    println!("hej, legal move, match");
    let legal = match (board.board[start[0]][start[1]]).abs() {
        1 => legal_pawn_move(&mut board.board, &board.board_history, start, end, player),
        2 => legal_knight_move(&board.board, start, end, player),
        3 => legal_bishop_move(&board.board, start, end, player),
        4 => legal_rook_move(&board.board, start, end, player),
        5 => legal_queen_move(&board.board, start, end, player),
        6 => legal_king_move(&board.board, start, end, player),
        _ => false,
    };
    legal
}

fn legal_pawn_move(board: &mut Vec<Vec<i8>>, bh: &Vec<Vec<Vec<i8>>>, start: &Vec<usize>, end: &Vec<usize>, player: i8) -> bool {
    let mut b = board.clone();
    if start[1] != end[1] { // side move
        println!("hej, legal move, pawn, side");
        if (start[1] as i8 - end[1] as i8).abs() != 1 {
            return false;
        }
        if start[0] as i8 + player != end[0] as i8 {
            return false;
        }
        if board[end[0]][end[1]] * player < 0 { // capture opponent 
            b[start[0]][start[1]] = 0;
            b[end[0]][end[1]] = player;
            if !player_in_check(&b) {
                return true;
            }
        } else if board[end[0]][end[1]] == 0 { // En passant
            if bh.len() == 0 {
                return false;
            }
            let last = bh.last().unwrap();
            if last[start[0]][end[1]] != 0 && board[start[0]][end[1]] != player * -1 {
                return false;
            }
            if last[(start[0] as i8 + 2 * player) as usize][end[1]] != player * -1 {
                return false;
            }
            if board[(start[0] as i8 + 2 * player) as usize][end[1]] != 0 {
                return false;
            }
            b[start[0]][start[1]] = 0;
            b[start[0]][end[1]] = 0;
            b[end[0]][end[1]] = player;
            if !player_in_check(&b) {
                board[start[0]][end[1]] = 0;
                return true;
            }
        } else {
            return false;
        }
    } else if (start[0] as i8 - end[0] as i8).abs() <= 2 { // move forward
        println!("hej, legal move, pawn, forward");
        if board[end[0]][end[1]] != 0 {
            return false;
        }
        if board[(start[0] as i8 + player) as usize][start[1]] != 0 {
            return false;
        }
        if (end[0] as i8 - start[0] as i8) * player <= 0 {
            return false;
        }
        if (start[0] as i8 - end[0] as i8).abs() == 2 {
            println!("hej, legal move, pawn, forward, 2");
            if start[0] != 1 && start[0] != 6 {
                return false;
            }
        }
        if !player_in_check(&b) {
            return true;
        }
    }
    println!("hej, legal move, pawn, false");
    false
}

fn legal_knight_move(board: &Vec<Vec<i8>>, start: &Vec<usize>, end: &Vec<usize>, player: i8) -> bool {
    true
}

fn legal_bishop_move(board: &Vec<Vec<i8>>, start: &Vec<usize>, end: &Vec<usize>, player: i8) -> bool {
    true
}

fn legal_rook_move(board: &Vec<Vec<i8>>, start: &Vec<usize>, end: &Vec<usize>, player: i8) -> bool {
    true
}

fn legal_queen_move(board: &Vec<Vec<i8>>, start: &Vec<usize>, end: &Vec<usize>, player: i8) -> bool {
    true
}

fn legal_king_move(board: &Vec<Vec<i8>>, start: &Vec<usize>, end: &Vec<usize>, player: i8) -> bool {
    true
}

fn player_in_check(board: &Vec<Vec<i8>>) -> bool {
    false
}

fn won(board: &Vec<Vec<i8>>) -> bool {
    false
}

fn print_board(board: &Vec<Vec<i8>>) {
    for r in board {
        for i in r {
            if *i < 0 {
                print!("{} ", i);
            } else {
                print!(" {} ", i);
            }
        }
        println!();
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new_board(1).board;
        assert_eq!(board[0], vec![4,3,2,6,5,2,3,4]);
        assert_eq!(board[1], vec![1; 8]);
        assert_eq!(board[2], vec![0; 8]);
        assert_eq!(board[3], vec![0; 8]);
        assert_eq!(board[4], vec![0; 8]);
        assert_eq!(board[5], vec![0; 8]);
        assert_eq!(board[6], vec![-1; 8]);
        assert_eq!(board[7], vec![-4,-3,-2,-6,-5,-2,-3,-4]);
    }

    #[test]
    fn test_update_board() {
        let mut board = Board::new_board(1);
        let start = vec![1, 0];
        let end = vec![2, 0];
        assert_eq!(board.update_board(start, end, 0), 0);
        assert_eq!(board.board[1], vec![0,1,1,1,1,1,1,1]);
        assert_eq!(board.board[2], vec![1,0,0,0,0,0,0,0]);
    }

    #[test]
    fn wrong_turn() {
        let mut board = Board::new_board(1);
        let start = vec![6, 0];
        let end = vec![5, 0];
        assert_eq!(board.update_board(start, end, 0), 2);
    }

    #[test]
    fn legal_pawn_move_1() {
        let mut board = Board::new_board(1);
        let start = vec![1, 0];
        let end = vec![3, 0];
        assert_eq!(board.update_board(start, end, 0), 0);
        assert_eq!(board.board[1], vec![0,1,1,1,1,1,1,1]);
        assert_eq!(board.board[2], vec![0,0,0,0,0,0,0,0]);
        assert_eq!(board.board[3], vec![1,0,0,0,0,0,0,0]);
    }

    #[test]
    fn legal_pawn_move_2() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![1, 0], vec![2, 0], 0), 0);
        assert_eq!(board.update_board(vec![6, 0], vec![4, 0], 0), 0);
        assert_eq!(board.update_board(vec![1, 1], vec![2, 1], 0), 0);
        assert_eq!(board.update_board(vec![6, 1], vec![5, 1], 0), 0);
        assert_eq!(board.update_board(vec![2, 1], vec![4, 1], 0), -2);
    }

    #[test]
    fn legal_pawn_move_3() {
        let mut board = Board::new_board(1);
        assert_eq!(board.update_board(vec![1, 0], vec![3, 0], 0), 0);
        assert_eq!(board.update_board(vec![6, 0], vec![4, 0], 0), 0);
        assert_eq!(board.update_board(vec![1, 1], vec![3, 1], 0), 0);
        assert_eq!(board.update_board(vec![6, 2], vec![4, 2], 0), 0);
        assert_eq!(board.update_board(vec![3, 1], vec![4, 0], 0), 0);
        assert_eq!(board.update_board(vec![6, 1], vec![4, 1], 0), 0);
        assert_eq!(board.update_board(vec![4, 0], vec![5, 1], 0), 0);
    }
}