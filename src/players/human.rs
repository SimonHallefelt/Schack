use std::usize;

pub fn run(all_moves: Vec<Vec<usize>>, mut movee: (Vec<usize>, usize)) -> Vec<usize> {
    println!("human, all_moves = {:?}, movee = {:?}", all_moves, movee);
    let mut moves = Vec::new();
    for m in all_moves {
        let mut mm = Vec::new();
        for i in 0..4 {
            mm.push(m[i]);
        }
        moves.push(mm);
    }
    if moves.contains(&movee.0) {
        movee.0.push(movee.1);
        movee.0
    } else {
        Vec::new()
    }
}