slint::include_modules!();

use slint::{Model, ModelRc, VecModel, Timer, TimerMode};


pub fn start_ui() -> Result<(), slint::PlatformError> {
    let ui = appWindow::new()?;

    let clicked = ui.get_square_clicked().iter().map(|x| x.clone()).collect::<Vec<bool>>()[0].clone();
    ui.set_square_clicked(ModelRc::new(VecModel::from(vec![clicked; 8*8])));

    let clicked = ui.get_possible_move().iter().map(|x| x.clone()).collect::<Vec<bool>>()[0].clone();
    ui.set_possible_move(ModelRc::new(VecModel::from(vec![clicked; 8*8])));

    run_ui(ui)
}

fn run_ui(ui: appWindow) -> Result<(), slint::PlatformError> {




    ui.run()
}
