use std::usize;

pub fn run(all_moves: Vec<Vec<usize>>, mut movee: (Vec<usize>, usize)) -> Vec<usize> {
    println!("human, all_moves = {:?}, movee = {:?}", all_moves, movee);
    if all_moves.contains(&movee.0) {
        movee.0.push(movee.1);
        movee.0
    } else {
        Vec::new()
    }
}