#![allow(dead_code, unused_variables, unused_imports)]
use macroquad::prelude::*;
mod graphics;
mod utils;

use graphics::{
    circle_mark_square, draw_board, draw_check, draw_pieces, get_mouse_input, highlight_square,
};
use utils::{
    actions_to_algebraic_ends, algebraic_to_action, algebraic_to_location, location_to_algebraic,
};
use utils::{Action, ActionKind, Board, Location, MoveError, Piece, PieceColor, PieceKind};

fn window_conf() -> Conf {
    Conf {
        window_width: 1000,
        window_height: 1000,
        fullscreen: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board::new(8);

    let start_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    board.set_fen(&start_fen);

    loop {
        draw_board(&board).await;
        draw_check(&board).await;
        if let Some(last_action) = board.last_action {
            highlight_square(&board, last_action.start).await;
            highlight_square(&board, last_action.end).await;
        }
        draw_pieces(&board).await;

        if let Some(location) = board.selected {
            let actions = board.get_valid_actions(location);
            for action in actions {
                circle_mark_square(&board, action.end).await;
            }
        }

        let location = get_mouse_input(&board);
        match location {
            Some(location) => {
                if board.selected == None {
                    board.selected = Some(location);
                } else {
                    let action = board.get_action_from_locations(board.selected.unwrap(), location);
                    let result = board.commit_move(action);
                    match result {
                        Ok(_) => (),
                        Err(error) => {
                            println!("Error: {:?}", error);
                            board.selected = None;
                        }
                    }
                };
            }
            None => {}
        };
        next_frame().await;
    }
}
