mod board;
mod legal_moves;
mod ui;
mod game;

fn main() {
    println!("Hello, world!");
    let board = board::Board::new_board(1);
    let game = game::Game::new_game(board);
    game.run();
    println!("program ended: {:?}", ui::start_ui());
}