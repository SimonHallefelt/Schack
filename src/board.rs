
/*
    0 - empty
    1 - pawn
    2 - knight
    3 - bishop
    4 - rook
    5 - queen
    6 - king
 */
pub fn new_board() -> Vec<Vec<i8>> {
    let mut board = vec![vec![0; 8]; 8];
    board[0] = vec![4,3,2,6,5,2,3,4];
    board[1] = vec![1; 8];
    board[6] = vec![1; 8];
    board[7] = vec![4,3,2,6,5,2,3,4];

    print_board(&board);
    board
}


fn print_board(board: &Vec<Vec<i8>>) {
    // println!("{:?}", board);
    for r in board {
        for i in r {
            print!(" {} ", i);
        }
        println!();
    }
}