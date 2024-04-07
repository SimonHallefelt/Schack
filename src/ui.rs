slint::include_modules!();

use slint::{Model, ModelRc, VecModel, Timer, TimerMode, Image};
use std::sync::{Arc, Mutex};

use crate::game::Game;
use std::path::Path;

pub fn start_ui(game: Game) -> Result<(), slint::PlatformError> {
    let ui = appWindow::new()?;

    let clicked = ui.get_square_clicked().iter().map(|x| x.clone()).collect::<Vec<bool>>()[0].clone();
    ui.set_square_clicked(ModelRc::new(VecModel::from(vec![clicked; 8*8])));

    let clicked = ui.get_possible_move().iter().map(|x| x.clone()).collect::<Vec<bool>>()[0].clone();
    ui.set_possible_move(ModelRc::new(VecModel::from(vec![clicked; 8*8])));

    let piece_positions = ui.get_piece_positions().iter().map(|x| x.clone()).collect::<Vec<Image>>()[0].clone();
    ui.set_piece_positions(ModelRc::new(VecModel::from(vec![piece_positions; 8*8])));


    run_ui(ui, Arc::new(Mutex::new(game)))
}

fn run_ui(ui: appWindow, game: Arc<Mutex<Game>>) -> Result<(), slint::PlatformError> {
    
    let ui2 = ui.as_weak();
    let b = game.lock().unwrap().get_board();
    update_board(b, ui2);



    ui.run()
}

fn update_board(board: Vec<Vec<i8>>, ui2: slint::Weak<appWindow>) {
    let ui = ui2.upgrade().unwrap();
    let mut piece_positions = vec![];
    for i in (0..8).rev() {
        for j in 0..8 {
            let piece = board[i][j];
            let piece_name = get_piece_name(piece);
            piece_positions.push(Image::load_from_path(Path::new(&piece_name)).unwrap());
        }
    }
    ui.set_piece_positions(ModelRc::new(VecModel::from(piece_positions)));
}

fn get_piece_name(num: i8) -> String {
    assert!(num > -7);
    assert!(num < 7);
    match num {
        1 => "icons\\white-pawn.png".to_string(),
        2 => "icons\\white-knight.png".to_string(),
        3 => "icons\\white-bishop.png".to_string(),
        4 => "icons\\white-rook.png".to_string(),
        5 => "icons\\white-queen.png".to_string(),
        6 => "icons\\white-king.png".to_string(),
        -1 => "icons\\black-pawn.png".to_string(),
        -2 => "icons\\black-knight.png".to_string(),
        -3 => "icons\\black-bishop.png".to_string(),
        -4 => "icons\\black-rook.png".to_string(),
        -5 => "icons\\black-queen.png".to_string(),
        -6 => "icons\\black-king.png".to_string(),
        _ => "icons\\empty.png".to_string(),
    }
}
