use rand::seq::SliceRandom;

pub fn run(all_moves: Vec<Vec<usize>>) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    all_moves.choose(&mut rng).unwrap().to_vec()
}