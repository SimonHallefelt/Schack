use core::time;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use rand::Rng;

pub fn run(board: &Vec<Vec<i8>>, board_history: &Vec<Vec<Vec<i8>>>, player: i8, castle_pieces: &HashSet<(usize,usize)>) -> Vec<usize> {
    // flip board horizontally if player is -1
    let mut new_board = Vec::new();
    let mut new_castle_pieces = HashSet::new();
    let mut new_board_history = Vec::new();
    if player == -1 {
        for row in board.iter().rev() {
            let mut new_row = Vec::new();
            for p in row {
                new_row.push(p.clone() * -1);
            }
            new_board.push(new_row);
        }
        for p in castle_pieces {
            if p.0 != 4 {
                new_castle_pieces.insert((7-p.0,p.1));
            }
        }
        if !board_history.is_empty() {
            let mut bhl = Vec::new();
            for row in board_history.last().unwrap().iter().rev() {
                let mut new_row = Vec::new();
                for p in row {
                    new_row.push(p.clone() * -1);
                }
                bhl.push(new_row);
            }
            new_board_history = vec![bhl];
        }
    } else {
        new_board = board.clone();
        new_castle_pieces = castle_pieces.clone();
        if !board_history.is_empty() {
            new_board_history = vec![board_history.last().unwrap().clone()];
        }
    }

    // get best move
    let best_move = setup_and_start(&new_board, &new_board_history, &new_castle_pieces); // always player 1

    // adjust move for player
    let mut m = Vec::new();
    if player == 1 {
        m = best_move;
    } else {
        m.push(7-best_move[0]);
        m.push(best_move[1]);
        m.push(7-best_move[2]);
        m.push(best_move[3]);
        m.push(best_move[4]);
    }
    m
}





#[derive(Clone, PartialEq, Eq, Hash, Debug)]

struct BitBoard {
    depth: i32,
    player: i8,
    white_king: u64,
    white_queen: u64,
    white_rooks: u64,
    white_bishops: u64,
    white_knights: u64,
    white_pawns: u64,
    black_king: u64,
    black_queen: u64,
    black_rooks: u64,
    black_bishops: u64,
    black_knights: u64,
    black_pawns: u64,
    white_pieces: u64,
    black_pieces: u64,
    // white_en_passant: u64,
    // black_en_passant: u64,
    // white_castle_k: bool,
    // white_castle_q: bool,
    // black_castle_k: bool,
    // black_castle_q: bool,
}

impl BitBoard {
    fn new() -> BitBoard {
        BitBoard {
            depth: 0,
            player: 1,
            white_king: 0,
            white_queen: 0,
            white_rooks: 0,
            white_bishops: 0,
            white_knights: 0,
            white_pawns: 0,
            black_king: 0,
            black_queen: 0,
            black_rooks: 0,
            black_bishops: 0,
            black_knights: 0,
            black_pawns: 0,
            white_pieces: 0,
            black_pieces: 0,
            // white_en_passant: 0,
            // black_en_passant: 0,
            // white_castle_k: false,
            // white_castle_q: false,
            // black_castle_k: false,
            // black_castle_q: false,
        }
    }

    fn make_move(&mut self, m: &Vec<u64>) { // m: [from, to, piece, promote]
        // dose not handle castling or en passant

        let from = m[0];
        let to = m[1];
        let piece = m[2];
        let promote = m[3];

        if self.player == 1 {
            self.white_pieces ^= from ^ to;
            if self.black_pieces & to != 0 {
                self.black_pieces ^= to;
                self.black_pawns ^= to & self.black_pawns;
                self.black_knights ^= to & self.black_knights;
                self.black_bishops ^= to & self.black_bishops;
                self.black_rooks ^= to & self.black_rooks;
                self.black_queen ^= to & self.black_queen;
                self.black_king ^= to & self.black_king;
            }
        } else {
            self.black_pieces ^= from ^ to;
            if self.white_pieces & to != 0 {
                self.white_pieces ^= to;
                self.white_pawns ^= to & self.white_pawns;
                self.white_knights ^= to & self.white_knights;
                self.white_bishops ^= to & self.white_bishops;
                self.white_rooks ^= to & self.white_rooks;
                self.white_queen ^= to & self.white_queen;
                self.white_king ^= to & self.white_king;
            }
        }

        if promote == 0 {
            match piece as i8 * self.player {
                1 => self.white_pawns ^= from ^ to,
                2 => self.white_knights ^= from ^ to,
                3 => self.white_bishops ^= from ^ to,
                4 => self.white_rooks ^= from ^ to,
                5 => self.white_queen ^= from ^ to,
                6 => self.white_king ^= from ^ to,
                -1 => self.black_pawns ^= from ^ to,
                -2 => self.black_knights ^= from ^ to,
                -3 => self.black_bishops ^= from ^ to,
                -4 => self.black_rooks ^= from ^ to,
                -5 => self.black_queen ^= from ^ to,
                -6 => self.black_king ^= from ^ to,
                _ => (),
            }
        } else {
            match self.player {
                1 => self.white_pawns ^= from,
                -1 => self.black_pawns ^= from,
                _ => (),
            }
            match promote as i8 * self.player {
                2 => self.white_knights ^= to,
                3 => self.white_bishops ^= to,
                4 => self.white_rooks ^= to,
                5 => self.white_queen ^= to,
                -2 => self.black_knights ^= to,
                -3 => self.black_bishops ^= to,
                -4 => self.black_rooks ^= to,
                -5 => self.black_queen ^= to,
                _ => (),
            }
        }

        self.depth += 1;
        self.player *= -1;
    }
}

fn setup_and_start(board: &Vec<Vec<i8>>, board_history: &Vec<Vec<Vec<i8>>>, castle_pieces: &HashSet<(usize,usize)>) -> Vec<usize> {
    // setup bit board data
    let mut bb = BitBoard::new();
    for i in 0..8 {
        for j in 0..8 {
            let p = board[i][j];
            if p == 0 {
                continue;
            }
            let pos = 1 << (i*8+j);
            match p {
                1 => bb.white_pawns |= pos,
                2 => bb.white_knights |= pos,
                3 => bb.white_bishops |= pos,
                4 => bb.white_rooks |= pos,
                5 => bb.white_queen |= pos,
                6 => bb.white_king |= pos,
                -1 => bb.black_pawns |= pos,
                -2 => bb.black_knights |= pos,
                -3 => bb.black_bishops |= pos,
                -4 => bb.black_rooks |= pos,
                -5 => bb.black_queen |= pos,
                -6 => bb.black_king |= pos,
                _ => (),
            }
            if p > 0 {
                bb.white_pieces |= pos;
            } else {
                bb.black_pieces |= pos;
            }
        }
    }

    // check for possible en passant moves
    // let last_board = board_history.last().unwrap();
    // if !last_board.is_empty() {  
    //     for i in 0..8 {
    //         if board[6][i] == 0 && board[4][i] == -1 && last_board[6][i] == -1 && last_board[4][i] == 0 {
    //             if i > 0 {
    //                 bb.white_en_passant |= 1 << (4*8+(i-1));
    //             }
    //             if i < 7 {
    //                 bb.white_en_passant |= 1 << (4*8+(i+1));
    //             }
    //         }
    //     }
    // }

    // castling rights







    // get best move
    let best_move = alpha(bb, 5); // best_move: [from, to, piece, promote]

    // format move
    let mut m: Vec<usize> = Vec::new();
    let from = best_move[0].trailing_zeros() as usize;
    let to = best_move[1].trailing_zeros() as usize;
    let promote = best_move[3] as usize;
    m.push(from/8);
    m.push(from%8);
    m.push(to/8);
    m.push(to%8);
    m.push(promote);
    m
}


fn alpha(bit_board: BitBoard, depth: i32) -> Vec<u64> {
    println!("start score = {}", score(&bit_board)-900);

    let mut alpha = -100000;
    let beta = alpha*-1;
    let mut hm = HashMap::new();
    // let alm = all_legal_moves(&bit_board);
    let alm = shuffle_vec(all_legal_moves(&bit_board));
    let mut bb;
    let mut score;
    let mut best_move = Vec::new();
    for lm in alm {
        bb = bit_board.clone();
        bb.make_move(&lm);
        score = alpha_beta(&bb, &mut hm, alpha, beta, -1, depth);

        if score > alpha {
            alpha = score;
            best_move = lm;
        }
    }

    println!("best_move = {:?}, end score = {}", best_move, alpha-900);
    println!("BitBoard = {:?}", bit_board);

    best_move
}

fn shuffle_vec(mut vec: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut rng = rand::thread_rng();
    for i in 0..vec.len() {
        let j = rng.gen_range(0..vec.len());
        vec.swap(i, j);
    }
    vec
}

fn alpha_beta(bit_board: &BitBoard, hm: &mut HashMap<BitBoard, i32>, mut alpha: i32, mut beta: i32, player: i8, depth: i32) -> i32 {
    if depth == 0 {
        return score(bit_board);
    }
    if hm.contains_key(bit_board) {
        return *hm.get(bit_board).unwrap();
    }
    if bit_board.white_king == 0 || bit_board.black_king == 0 {
        return 20000 * player as i32 * -1 + depth * player as i32 * -1;
    }
    let alm = all_legal_moves(bit_board);
    if alm.is_empty() {
        return 10000 * player as i32 * -1 + depth * player as i32 * -1;
    }

    let mut bb;
    let mut score;
    for lm in alm {
        bb = bit_board.clone();
        bb.make_move(&lm);
        score = alpha_beta(&bb, hm, alpha, beta, player*-1, depth-1);

        if player == 1 {
            if score > alpha {
                alpha = score;
            }
        } else {
            if score < beta {
                beta = score;
            }
        }
        if alpha >= beta {
            break;
        }
    }

    if player == 1 {
        hm.insert(bit_board.clone(), alpha);
        alpha
    } else {
        hm.insert(bit_board.clone(), beta);
        beta
    }
}

fn all_legal_moves(bit_board: &BitBoard) -> Vec<Vec<u64>> { // moves: [move: [from, to, piece, promote]]
    // get all legal moves for player, dose not look at castling, en passant, or check
    let mut moves = Vec::new();
    moves.extend(pawn_moves(bit_board));
    moves.extend(knight_moves(bit_board));
    moves.extend(bishop_moves(bit_board));
    moves.extend(rook_moves(bit_board));
    moves.extend(queen_moves(bit_board));
    moves.extend(king_moves(bit_board));
    moves
}

fn pawn_moves(bit_board: &BitBoard) -> Vec<Vec<u64>> { // dose not look at en passant
    let mut moves = Vec::new();
    let dir;
    let start_row;
    let my_pieces;
    let mut pawns;
    let opponent_pieces;

    if bit_board.player == 1 {
        dir = 8;
        start_row = 1;
        my_pieces = bit_board.white_pieces;
        pawns = bit_board.white_pawns;
        opponent_pieces = bit_board.black_pieces;
    } else {
        dir = -8;
        start_row = 6;
        my_pieces = bit_board.black_pieces;
        pawns = bit_board.black_pawns;
        opponent_pieces = bit_board.white_pieces;
    }

    if my_pieces == 0 {
        return moves;
    }

    let mut p = pawns.trailing_zeros() as i32;
    while p != 64 {
        pawns ^= 1 << p;

        let mut to = p + dir;
        // attack
        let col = p%8;
        let row = p/8;
        if col > 0 && (1 << to - 1) & opponent_pieces != 0 {
            if row == 7-start_row {
                for promote in 2..6 {
                    moves.push(vec![1 << p, 1 << to - 1, 1, promote]);
                }
            } else {
                moves.push(vec![1 << p, 1 << to - 1, 1, 0]);
            }
        }
        if col < 7 && (1 << to + 1) & opponent_pieces != 0 {
            if row == 7-start_row {
                for promote in 2..6 {
                    moves.push(vec![1 << p, 1 << to + 1, 1, promote]);
                }
            } else {
                moves.push(vec![1 << p, 1 << to + 1, 1, 0]);
            }
        }
        // move forward
        if (1 << to) & (my_pieces | opponent_pieces) == 0 {
            if row == 7-start_row {
                for promote in 2..6 {
                    moves.push(vec![1 << p, 1 << to, 1, promote]);
                }
            } else {
                moves.push(vec![1 << p, 1 << to, 1, 0]);
                if row == start_row {
                    to += dir;
                    if (1 << to) & (my_pieces | opponent_pieces) == 0 {
                        moves.push(vec![1 << p, 1 << to, 1, 0]);
                    }
                }
            }
        }
        p = pawns.trailing_zeros() as i32;
    }

    moves
}

fn knight_moves(bit_board: &BitBoard) -> Vec<Vec<u64>> {
    let mut moves = Vec::new();
    let dir = [-17, -15, -10, -6, 6, 10, 15, 17];
    let pieces;
    let mut knights;

    if bit_board.player == 1 {
        pieces = bit_board.white_pieces;
        knights = bit_board.white_knights;
    } else {
        pieces = bit_board.black_pieces;
        knights = bit_board.black_knights;
    }

    let mut k = knights.trailing_zeros() as i32;
    while k != 64 {
        knights ^= 1 << k;

        let col = k%8;
        for d in dir.iter() {
            let to = k as i32 + d;
            let col_diff = (col - to%8).abs();
            if to < 0 || to > 63 || col_diff > 2 {
                continue;
            }
            if (1 << to) & pieces != 0 { 
                continue;
            }
            moves.push(vec![1 << k, 1 << to, 2, 0]);
        }
        k = knights.trailing_zeros() as i32;
    }

    moves
}

fn bishop_moves(bit_board: &BitBoard) -> Vec<Vec<u64>> {
    let my_pieces;
    let bishops;
    let opponent_pieces;

    if bit_board.player == 1 {
        my_pieces = bit_board.white_pieces;
        bishops = bit_board.white_bishops;
        opponent_pieces = bit_board.black_pieces;
    } else {
        my_pieces = bit_board.black_pieces;
        bishops = bit_board.black_bishops;
        opponent_pieces = bit_board.white_pieces;
    }

    c_moves(bishops, my_pieces, opponent_pieces, 3)
}

fn rook_moves(bit_board: &BitBoard) -> Vec<Vec<u64>> {
    let my_pieces;
    let rooks;
    let opponent_pieces;

    if bit_board.player == 1 {
        my_pieces = bit_board.white_pieces;
        rooks = bit_board.white_rooks;
        opponent_pieces = bit_board.black_pieces;
    } else {
        my_pieces = bit_board.black_pieces;
        rooks = bit_board.black_rooks;
        opponent_pieces = bit_board.white_pieces;
    }

    h_and_v_moves(rooks, my_pieces, opponent_pieces, 4)
}

fn queen_moves(bit_board: &BitBoard) -> Vec<Vec<u64>> {
    let my_pieces;
    let queen;
    let opponent_pieces;

    if bit_board.player == 1 {
        my_pieces = bit_board.white_pieces;
        queen = bit_board.white_queen;
        opponent_pieces = bit_board.black_pieces;
    } else {
        my_pieces = bit_board.black_pieces;
        queen = bit_board.black_queen;
        opponent_pieces = bit_board.white_pieces;
    }

    let mut moves = h_and_v_moves(queen, my_pieces, opponent_pieces, 5);
    moves.extend(c_moves(queen, my_pieces, opponent_pieces, 5));
    moves
}

fn king_moves(bit_board: &BitBoard) -> Vec<Vec<u64>> { // dose not look at castling
    let mut moves = Vec::new();
    let dir = [-9, -8, -7, -1, 1, 7, 8, 9];
    let king;
    let pieces;

    if bit_board.player == 1 {
        king = bit_board.white_king.trailing_zeros() as i32;
        pieces = bit_board.white_pieces;
    } else {
        king = bit_board.black_king.trailing_zeros() as i32;
        pieces = bit_board.black_pieces;
    }

    let col = king%8;
    for d in dir.iter() {
        let to = king as i32 + d;
        let col_diff = (col - to%8).abs();
        if to < 0 || to > 63 || col_diff > 1 {
            continue;
        }
        if (1 << to) & pieces != 0 {
            continue;
        }
        moves.push(vec![1 << king, 1 << to, 6, 0]);
    }

    moves
}

fn h_and_v_moves(mut pieces: u64, my_pieces: u64, opponent_pieces: u64, piece: u64) -> Vec<Vec<u64>> {
    let mut moves = Vec::new();
    let dir1 = [-8, 8];
    let dir2 = [-1, 1];

    let mut r = pieces.trailing_zeros() as i32;
    while r != 64 {
        pieces ^= 1 << r;

        for d in dir1 {
            let mut to = r + d;
            while to >= 0 && to < 64 {
                if (1 << to) & my_pieces != 0 {
                    break;
                }
                moves.push(vec![1 << r, 1 << to, piece, 0]);
                if (1 << to) & opponent_pieces != 0 {
                    break;
                }
                to += d;
            }
        }
        let row = r/8;
        for d in dir2 {
            let mut to = r + d;
            while to >= 0 && to < 64 {
                if (1 << to) & my_pieces != 0 {
                    break;
                }
                if row != to/8 {
                    break;
                }
                moves.push(vec![1 << r, 1 << to, piece, 0]);
                if (1 << to) & opponent_pieces != 0 {
                    break;
                }
                to += d;
            }
        }
        r = pieces.trailing_zeros() as i32;
    }

    moves
}

fn c_moves(mut pieces: u64, my_pieces: u64, opponent_pieces: u64, piece: u64) -> Vec<Vec<u64>> {
    let mut moves = Vec::new();
    let dir = [-9, -7, 7, 9];

    let mut b = pieces.trailing_zeros() as i32;
    while b != 64 {
        pieces ^= 1 << b;

        let row = b/8;
        let col = b%8;
        for d in dir.iter() {
            let mut to = b + d;
            while to >= 0 && to < 64 {
                if (1 << to) & my_pieces != 0 {
                    break;
                }
                if (row - to/8).abs() != (col - to%8).abs() {
                    break;
                }
                moves.push(vec![1 << b, 1 << to, piece, 0]);
                if (1 << to) & opponent_pieces != 0 {
                    break;
                }
                to += d;
            }
        }
        b = pieces.trailing_zeros() as i32;
    }

    moves
}



fn score(bit_board: &BitBoard) -> i32 {
    let mut score = 0;
    score += piece_score(1) * bit_board.white_pawns.count_ones() as i32;
    score += piece_score(2) * bit_board.white_knights.count_ones() as i32;
    score += piece_score(3) * bit_board.white_bishops.count_ones() as i32;
    score += piece_score(4) * bit_board.white_rooks.count_ones() as i32;
    score += piece_score(5) * bit_board.white_queen.count_ones() as i32;
    score += piece_score(6);
    score += piece_score(-1) * bit_board.black_pawns.count_ones() as i32;
    score += piece_score(-2) * bit_board.black_knights.count_ones() as i32;
    score += piece_score(-3) * bit_board.black_bishops.count_ones() as i32;
    score += piece_score(-4) * bit_board.black_rooks.count_ones() as i32;
    score += piece_score(-5) * bit_board.black_queen.count_ones() as i32;
    score += piece_score(-6);
    score
}

fn piece_score(piece: i8) -> i32 {
    match piece {
        1 => 1,
        2 => 3,
        3 => 3,
        4 => 5,
        5 => 9,
        6 => 1000,
        -1 => -1,
        -2 => -3,
        -3 => -3,
        -4 => -5,
        -5 => -9,
        -6 => -100,
        _ => 0,
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn print_bit_board(bb: BitBoard) {
        let mut b = Vec::new();
        for i in 0..8 {
            let mut row = Vec::new();
            for j in 0..8 {
                let pos = 1 << (i*8+j);
                if bb.white_pawns & pos != 0 {
                    row.push(1);
                } else if bb.white_knights & pos != 0 {
                    row.push(2);
                } else if bb.white_bishops & pos != 0 {
                    row.push(3);
                } else if bb.white_rooks & pos != 0 {
                    row.push(4);
                } else if bb.white_queen & pos != 0 {
                    row.push(5);
                } else if bb.white_king & pos != 0 {
                    row.push(6);
                } else if bb.black_pawns & pos != 0 {
                    row.push(-1);
                } else if bb.black_knights & pos != 0 {
                    row.push(-2);
                } else if bb.black_bishops & pos != 0 {
                    row.push(-3);
                } else if bb.black_rooks & pos != 0 {
                    row.push(-4);
                } else if bb.black_queen & pos != 0 {
                    row.push(-5);
                } else if bb.black_king & pos != 0 {
                    row.push(-6);
                } else {
                    row.push(0);
                }
            }
            b.push(row);
        }
        for r in b.iter().rev() {
            for p in r {
                if *p < 0 {
                    print!("{} ", p);
                } else {
                    print!(" {} ", p);
                }
            }
            println!()
        }
    }

    #[test]
    fn in_check_1() {
        let mut board = vec![
            vec![-4,-2,-3,-5,-6,-3,-2,-4],
            vec![ 0, 0, 0, 0,-1, 0,-3,-1],
            vec![-1, 0,-1, 0, 0, 0, 0, 0],
            vec![ 0,-1, 0,-1, 0,-1, 0, 5],
            vec![ 0, 0, 0, 0, 0, 0, 0, 0],
            vec![ 0, 1, 0, 3, 1, 0, 0, 0],
            vec![ 1, 0, 1, 1, 0, 1, 1, 1],
            vec![ 4, 2, 3, 0, 6, 0, 2, 4],
        ];
        board.reverse();
        let board_history = vec![];
        let castle_pieces = HashSet::new();
        let result = run(&board, &board_history, -1, &castle_pieces);
        assert_eq!(result, vec![7, 4, 6, 3, 0]);
    }

    #[test]
    fn in_check_2() {
        let mut board = vec![
            vec![-4,-2, 5, 0,-6, 0, 0,-4],
            vec![ 0, 0, 0, 0,-4, 0, 0, 0],
            vec![ 0, 0, 0, 0, 0,-1,-1,-2],
            vec![-1, 0, 0,-3,-5, 0, 0, 0],
            vec![ 0, 0, 0, 0, 1, 0, 1, 0],
            vec![ 0, 0,-1, 0, 0, 0, 0, 1],
            vec![ 1, 1, 0, 0, 2, 6, 3, 0],
            vec![ 4, 2, 0, 0, 0, 0, 0, 0],
        ];
        board.reverse();
        let board_history = vec![];
        let castle_pieces = HashSet::new();
        let result = run(&board, &board_history, -1, &castle_pieces);
        assert_eq!(result, vec![7, 4, 6, 5, 0]);
    }

    #[test]
    fn in_check_3() {
        let mut board = vec![
            vec![ 0,-6, 0,-3, 0, 0, 0, 0],
            vec![ 0, 0, 0, 0, 0, 0, 0, 0],
            vec![ 0, 0, 0, 0, 0, 0, 0, 0],
            vec![-1, 0,-1, 3, 0,-1, 0, 0],
            vec![ 0,-1, 0, 0, 0, 1, 0, 0],
            vec![ 0, 4, 0, 0, 0, 0, 0, 2],
            vec![ 0, 0, 0, 0, 0, 0, 0, 0],
            vec![ 0, 2, 3, 0, 6, 0, 0, 0],
        ];
        board.reverse();
        let board_history = vec![];
        let castle_pieces = HashSet::new();
        let result = run(&board, &board_history, -1, &castle_pieces);
        assert_ne!(result, vec![7, 1, 7, 0, 0]);
        assert_ne!(result, vec![7, 1, 6, 1, 0]);
    }

    #[test]
    fn queen_moves_1() {
        let bb = BitBoard { depth: 0,
                                    player: 1, 
                                    white_king: 1, 
                                    white_queen: 16, 
                                    white_rooks: 0, 
                                    white_bishops: 0, 
                                    white_knights: 0, 
                                    white_pawns: 0, 
                                    black_king: 65536, 
                                    black_queen: 0, 
                                    black_rooks: 0, 
                                    black_bishops: 64, 
                                    black_knights: 0, 
                                    black_pawns: 0, 
                                    white_pieces: 1+16, 
                                    black_pieces: 65536+64, };
        let qm = queen_moves(&bb);
        println!("{:?}", qm);
        assert_eq!(qm.len(), 3+4+7+3+2);
    }

    #[test]
    fn bishop_moves_1() {
        let bb = BitBoard { depth: 0,
                                    player: -1, 
                                    white_king: 2, 
                                    white_queen: 0, 
                                    white_rooks: 0, 
                                    white_bishops: 0, 
                                    white_knights: 1 << 8*1+5, 
                                    white_pawns: 0, 
                                    black_king: 1 << 8*5+5, 
                                    black_queen: 0, 
                                    black_rooks: 0, 
                                    black_bishops: 1 << 8*3+3, 
                                    black_knights: 0, 
                                    black_pawns: 0, 
                                    white_pieces: 2+(1 << 8*1+5), 
                                    black_pieces: ((1 as u64) << 8*5+5)+((1 as u64) << 8*3+3)};
        let bm = bishop_moves(&bb);
        println!("{:?}", bm);
        println!("bit_board = {:?}", bb);
        print_bit_board(bb);
        assert_eq!(bm.len(), 3+3+1+2);
    }

    #[test]
    fn bishop_moves_2() {
        let bb = BitBoard { depth: 0,
                                    player: -1, 
                                    white_king: 1, 
                                    white_queen: 0, 
                                    white_rooks: 0, 
                                    white_bishops: 8, 
                                    white_knights: 0, 
                                    white_pawns: 9210691584, 
                                    black_king: 1152921504606846976, 
                                    black_queen: 0, 
                                    black_rooks: 2199023255552, 
                                    black_bishops: 288230376285929472, 
                                    black_knights: 144255925564211200, 
                                    black_pawns: 137438953472, 
                                    white_pieces: 9210691593, 
                                    black_pieces: 1585410142919196672 };
        let bm = bishop_moves(&bb);
        println!("{:?}", bm);
        println!("bit_board = {:?}", bb);
        print_bit_board(bb);
        assert_eq!(bm.len(), 3+1+4+3 + 2+2);
        assert!(bm.contains(&vec![1 << 8*3+3, 1, 3, 0]));
    }

    #[test]
    fn knight_moves_1() {
        let bb = BitBoard { depth: 0,
                                    player: 1, 
                                    white_king: 4, 
                                    white_queen: 0, 
                                    white_rooks: 0, 
                                    white_bishops: 1 << (2*8+2), 
                                    white_knights: 2, 
                                    white_pawns: 0, 
                                    black_king: 1 << (1*8+0), 
                                    black_queen: 0, 
                                    black_rooks: 0, 
                                    black_bishops: 0, 
                                    black_knights: 1 << (2*8), 
                                    black_pawns: 0, 
                                    white_pieces: 4 + (1 << (2*8+2)) + 2, 
                                    black_pieces: (1 << (1*8+0)) + (1 << (2*8)) };
        let km = knight_moves(&bb);
        println!("{:?}", km);
        println!("bit_board = {:?}", bb);
        print_bit_board(bb);
        assert_eq!(km.len(), 2);
    }

    #[test]
    fn king_moves_1() {
        let bb = BitBoard { depth: 0,
                                    player: 1, 
                                    white_king: 1, 
                                    white_queen: 0, 
                                    white_rooks: 0, 
                                    white_bishops: 0, 
                                    white_knights: 2, 
                                    white_pawns: 0, 
                                    black_king: 1 << (5*8), 
                                    black_queen: 0, 
                                    black_rooks: 0, 
                                    black_bishops: 0, 
                                    black_knights: 1 << (1*8), 
                                    black_pawns: 0, 
                                    white_pieces: 1 + 2, 
                                    black_pieces: (1 << (5*8)) + (1 << (1*8)) };
        let km = king_moves(&bb);
        println!("{:?}", km);
        println!("bit_board = {:?}", bb);
        print_bit_board(bb);
        assert_eq!(km.len(), 2);
    }

    #[test]
    fn promotion() {
        let bb = BitBoard { depth: 0,
                                    player: -1, 
                                    white_king: 1, 
                                    white_queen: 0, 
                                    white_rooks: 0, 
                                    white_bishops: 64, 
                                    white_knights: 0, 
                                    white_pawns: 0, 
                                    black_king: 4, 
                                    black_queen: 0, 
                                    black_rooks: 0, 
                                    black_bishops: 0, 
                                    black_knights: 0, 
                                    black_pawns: 1 << 8*1+5, 
                                    white_pieces: 1 + 64, 
                                    black_pieces: (1 << 8*1+5) + 4 };
        let pm = pawn_moves(&bb);
        println!("{:?}", pm);
        println!("bit_board = {:?}", bb);
        print_bit_board(bb);
        assert_eq!(pm.len(), 4+4);
        assert!(pm.contains(&vec![1 << 8*1+5, 32, 1, 2]));
        assert!(pm.contains(&vec![1 << 8*1+5, 64, 1, 5]));
    }

    #[test]
    fn en_passant() {
        assert!(false);
    }

    #[test]
    fn castling() {
        assert!(false);
    }
}