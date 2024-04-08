use rand::{seq::SliceRandom, Rng};

pub fn run(all_moves: Vec<Vec<usize>>) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut m = all_moves.choose(&mut rng).unwrap().to_vec();
    let promotion = rng.gen_range(2..6) as usize;
    m.push(promotion);
    m
}