slint::include_modules!();

use slint::{Model, ModelRc, VecModel, Timer, TimerMode, Image};
use std::{sync::{Arc, Mutex}, path::Path};

use crate::game;
use crate::game::Game;

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
    
    let ui_time = ui.as_weak();
    let g_time = game.clone();
    let time = Timer::default();
    time.start(TimerMode::Repeated, std::time::Duration::from_millis(1), move || {
        if let Ok(g) = g_time.lock() {
            let ui = ui_time.upgrade().unwrap().as_weak();
            update_board(g.get_board(), ui);
        }
    });

    ui.on_start_game({
        let game = Arc::clone(&game);
        move |player_type_1, player_type_2| {
            game::start_game(Arc::clone(&game), player_type_1, player_type_2);
        }
    });

    ui.on_board_square_clicked({
        let ui_weak = ui.as_weak().clone();
        let g = Arc::clone(&game);
        move |num| {
            board_clicked(ui_weak.clone(), Arc::clone(&g), num as usize);
        }
    });

    ui.on_white_promote_to({
        let g = game.clone();
        move |num| {
            set_promote(1, Arc::clone(&g), num as usize);
        }
    });

    ui.on_black_promote_to({
        let g = game.clone();
        move |num| {
            set_promote(-1, Arc::clone(&g), num as usize);
        }
    });

    ui.run()
}



fn update_board(board: Vec<Vec<i8>>, ui_weak: slint::Weak<appWindow>) {
    let ui = ui_weak.upgrade().unwrap();
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

fn board_clicked(ui_weak: slint::Weak<appWindow>, game: Arc<Mutex<Game>>, num: usize) {
    let ui = ui_weak.upgrade().unwrap();
    let p = num_to_pos(num);
    let mut clicked = vec![];
    for i in (0..8).rev() {
        for j in 0..8 {
            if i == p[0] && j == p[1] {
                clicked.push(true);
            } else {
                clicked.push(false);
            }
        }
    }
    ui.set_square_clicked(ModelRc::new(VecModel::from(clicked)));
    let mut g = game.lock().unwrap();
    g.clicked(p);
}

fn num_to_pos(num: usize) -> Vec<usize> {
    let col = num % 8;
    let row = 7 - (num / 8);
    vec![row, col]
}

fn set_promote(player: i32, game: Arc<Mutex<Game>>, num: usize) {
    let mut g = game.lock().unwrap();
    g.set_promote(player, num);
}