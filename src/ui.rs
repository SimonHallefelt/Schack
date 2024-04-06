slint::include_modules!();

use slint::{Model, ModelRc, VecModel, Timer, TimerMode};


pub fn start_ui() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;



    run_ui(ui)
}

pub fn run_ui(ui: AppWindow) -> Result<(), slint::PlatformError> {

    ui.run()
}