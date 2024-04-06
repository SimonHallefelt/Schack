slint::include_modules!();

use slint::{Model, ModelRc, VecModel, Timer, TimerMode};


pub fn start_ui() -> Result<(), slint::PlatformError> {
    let ui = appWindow::new()?;

    let test = vec![2; 8*8];

    make_ui_board(&ui);

    run_ui(ui)
}

fn run_ui(ui: appWindow) -> Result<(), slint::PlatformError> {




    ui.run()
}

fn make_ui_board(ui: &appWindow) {
    let bs = ui.get_board().iter().map(|p| p.clone()).collect::<Vec<boardSquareData>>()[0].clone();
    let board = ModelRc::new(VecModel::from(vec![bs; 8*8]));
    ui.set_board(board);
}