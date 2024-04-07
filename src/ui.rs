slint::include_modules!();

use slint::{Model, ModelRc, VecModel, Timer, TimerMode};
use crate::game::Game;

pub fn start_ui(game: Game) -> Result<(), slint::PlatformError> {
    let ui = appWindow::new()?;

    let clicked = ui.get_square_clicked().iter().map(|x| x.clone()).collect::<Vec<bool>>()[0].clone();
    ui.set_square_clicked(ModelRc::new(VecModel::from(vec![clicked; 8*8])));

    let clicked = ui.get_possible_move().iter().map(|x| x.clone()).collect::<Vec<bool>>()[0].clone();
    ui.set_possible_move(ModelRc::new(VecModel::from(vec![clicked; 8*8])));

    let piece_positions = ui.get_piece_positions().iter().map(|x| x.clone()).collect::<Vec<i32>>()[0].clone();
    // let test = game.get_board().iter();
    ui.set_piece_positions(ModelRc::new(VecModel::from(vec![piece_positions; 8*8])));

    run_ui(ui)
}

fn run_ui(ui: appWindow) -> Result<(), slint::PlatformError> {




    ui.run()
}
