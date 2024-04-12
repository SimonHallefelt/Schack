use std::collections::{HashMap, HashSet};

use crate::legal_moves::get_all_legal_moves;

pub fn run(board: &Vec<Vec<i8>>, board_history: &Vec<Vec<Vec<i8>>>, player: i8, castle_pieces: &HashSet<(usize,usize)>) -> Vec<usize> {
    let mut best_move = Vec::new();
    let board = prep_board(board, player);
    let bh;
    if board_history.is_empty() {
        bh = vec![]
    } else {
        bh = vec![prep_board(board_history.last().unwrap(), player)];
    }
    let mut cp = HashSet::new();
    if player == 1 {
        cp = castle_pieces.clone();
    } else {
        for p in castle_pieces {
            cp.insert((7-p.0,p.1));
        }
    }
    let alm = get_all_legal_moves(&board, &bh, 1, &cp); // upgrade, give random order

    let mut alpha = -10000;
    let beta = 10000;
    let mut hm = HashMap::new();
    let mut new_board;
    let mut new_castle_pieces;
    let new_board_history = vec![board.clone()];
    let mut a;
    for lm in alm {
        new_board = board.clone();
        new_castle_pieces = cp.clone();
        make_move(&lm, &mut new_board, &mut new_castle_pieces);
        a = alpha_beta(&new_board, alpha, beta, -1, 4, &new_castle_pieces, &new_board_history, &mut hm);

        if a > alpha {
            alpha = a;
            best_move = lm;
        }
    }

    if player == -1 {
        best_move[0] = 7-best_move[0];
        best_move[2] = 7-best_move[2];
    }

    best_move
}

fn alpha_beta(board: &Vec<Vec<i8>>, mut alpha: i32, mut beta: i32, turn: i8, depth: i32, castle_pieces: &HashSet<(usize,usize)>, board_history: &Vec<Vec<Vec<i8>>>, hm: &mut HashMap<Vec<Vec<i8>>, i32>) -> i32 {
    if depth == 0 {
        return score(board);
    }
    if hm.contains_key(board) {
        return *hm.get(board).unwrap();
    }
    let alm = get_all_legal_moves(board, board_history, turn, castle_pieces);
    if alm.is_empty() {
        if in_check(board, turn) {
            return 10000 * turn as i32 * -1;
        }
        return 0;
    }
    let mut new_board;
    let mut new_castle_pieces;
    let new_board_history = vec![board.clone()];
    let mut score;
    for lm in alm {
        new_board = board.clone();
        new_castle_pieces = castle_pieces.clone();
        make_move(&lm, &mut new_board, &mut new_castle_pieces);
        score = alpha_beta(&new_board, alpha, beta, turn*-1, depth-1, &new_castle_pieces, &new_board_history, hm);

        if turn == 1 {
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
    
    if turn == 1 {
        hm.insert(board.clone(), alpha);
        alpha
    } else {
        hm.insert(board.clone(), beta);
        beta
    }
}

fn score(board: &Vec<Vec<i8>>) -> i32 {
    let mut score = 0;
    for r in board {
        for &c in r {
            score += piece_score(c)
        }
    }
    score
}

fn make_move(movee: &Vec<usize>, board: &mut Vec<Vec<i8>>, castle_pieces: &mut HashSet<(usize,usize)>) {
    if board[movee[0]][movee[1]].abs() == 1 && movee[1] != movee[3] && board[movee[2]][movee[3]].abs() == 0 { // En passant
        board[movee[0]][movee[3]] = 0;
    } else if board[movee[0]][movee[1]].abs() == 6 && (movee[1] as i8 - movee[3] as i8).abs() == 2 { // castle
        if movee[1] > movee[3] {
            board[movee[2]][movee[3]+1] = board[movee[0]][0];
            board[movee[0]][0] = 0;
        } else {
            board[movee[2]][movee[3]-1] = board[movee[0]][7];
            board[movee[0]][7] = 0;
        }
    }

    board[movee[2]][movee[3]] = board[movee[0]][movee[1]];
    board[movee[0]][movee[1]] = 0;

    if board[movee[2]][movee[3]].abs() == 1 && movee[4] != 0 { // promote
        board[movee[2]][movee[3]] *= movee[4] as i8;
    }

    castle_pieces.remove(&(movee[0], movee[1]));
    castle_pieces.remove(&(movee[2], movee[3]));
}

fn in_check(board: &Vec<Vec<i8>>, player: i8) -> bool {
    let king = 6 * player;
    let king_pos = find_king(king, board);

    if pawn_attack(player, king_pos, board) {
        true
    } else if knight_attack(player, king_pos, board) {
        true
    } else if rook_queen_attack(player, king_pos, board) {
        true
    } else if bishop_queen_attack(player, king_pos, board) {
        true
    } else {
        false
    }
}

fn pawn_attack(player: i8, king_pos: (usize,usize), board: &Vec<Vec<i8>>) -> bool {
    if player == 1 {
        if king_pos.0 != 7 {
            if king_pos.1 != 0 {
                if board[king_pos.0+1][king_pos.1-1] == -1 {
                    return true;
                }
            }
            if king_pos.1 != 7 {
                if board[king_pos.0+1][king_pos.1+1] == -1 {
                    return true;
                }
            }
        }
    } else {
        if king_pos.0 != 0 {
            if king_pos.1 != 0 {
                if board[king_pos.0-1][king_pos.1-1] == 1 {
                    return true;
                }
            }
            if king_pos.1 != 7 {
                if board[king_pos.0-1][king_pos.1+1] == 1 {
                    return true;
                }
            }
        }
    }
    false
}

fn knight_attack(player: i8, king_pos: (usize,usize), board: &Vec<Vec<i8>>) -> bool {
    let dir = vec![(2,1),(-2,1),(-2,-1),(2,-1), (1,2),(-1,2),(-1,-2),(1,-2)];
    for d in dir {
        let a = king_pos.0 as i8 + d.0;
        let b = king_pos.1 as i8 + d.1;
        if a < 0 || a > 7 || b < 0 || b > 7 {
            continue;
        }
        if board[a as usize][b as usize] == 2 * player * -1 {
            return true;
        }
    }
    false
}

fn rook_queen_attack(player: i8, king_pos: (usize,usize), board: &Vec<Vec<i8>>) -> bool {
    let dir = vec![(1,0),(-1,0),(0,1),(0,-1)];
    let targets: HashSet<i8> = HashSet::from_iter(vec![4*player*-1, 5*player*-1]);
    return possible_direction_moves(board, king_pos, targets, dir);
}

fn bishop_queen_attack(player: i8, king_pos: (usize,usize), board: &Vec<Vec<i8>>) -> bool {
    let dir = vec![(1,1),(-1,1),(-1,-1),(1,-1)];
    let targets: HashSet<i8> = HashSet::from_iter(vec![3*player*-1, 5*player*-1]);
    return possible_direction_moves(board, king_pos, targets, dir);
}

fn possible_direction_moves(board: &Vec<Vec<i8>>, start: (usize, usize), targets: HashSet<i8>, dir: Vec<(i8, i8)>) -> bool {
    for d in dir {
        for i in 1..8 {
            let a = start.0 as i8 + d.0 * i;
            let b = start.1 as i8 + d.1 * i;
            if a < 0 || a > 7 || b < 0 || b > 7 {
                break;
            }
            if board[a as usize][b as usize] != 0 {
                if targets.contains(&board[a as usize][b as usize]) {
                    return true;
                }
                break;
            }
        }
    }
    false
}

fn find_king(king: i8, board: &Vec<Vec<i8>>) -> (usize,usize) {
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == king {
                return (i, j);
            }
        }
    }
    assert!(false);
    (10,10)
}

fn prep_board(board: &Vec<Vec<i8>>, player: i8) -> Vec<Vec<i8>> {
    if player == 1 {
        return board.clone();
    }
    let mut new_board = Vec::new();
    for r in board.iter().rev() {
        let mut row = Vec::new();
        for c in r {
            row.push(c*-1);
        }
        new_board.push(row);
    }
    new_board
}

fn piece_score(piece: i8) -> i32 {
    match piece {
        6 => 1000,
        5 => 7,
        4 => 5,
        3 => 3,
        2 => 3,
        1 => 1,
        -6 => -100,
        -5 => -7,
        -4 => -5,
        -3 => -3,
        -2 => -3,
        -1 => -1,
        _ => 0
    }
}
