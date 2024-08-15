use macroquad::prelude::*;
mod engine;
mod graphics;
#[allow(unused_variables, dead_code)]
mod utils;

use engine::get_minimax_action;
use graphics::*;
use utils::*;

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
    let players = ['h', 'e'];
    let mut current_player = 0;

    let mut board = Board::new(8);

    let start_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    // let start_fen = String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R");
    // let start_fen = String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1");
    board.set_fen(&start_fen);

    let mut timer = 200;
    loop {
        let mut tree = board.get_position_tree(1);
        update_tree(&mut tree);
        println!("{:.2}", tree.value);

        timer -= 1;
        draw_board(&board).await;
        draw_check(&board).await;
        if let Some(last_action) = board.last_action {
            highlight_square(&board, last_action.start).await;
            highlight_square(&board, last_action.end).await;
        }
        draw_pieces(&board).await;
        if board.is_moveless() {
            if board.is_check(board.turn) {
                println!("Game over. {:?} wins", opposite_color(board.turn));
            } else {
                println!("Game over. It's a draw");
            }
        } else {
            if let Some(location) = board.selected {
                let actions = board.get_valid_actions(location);
                for action in actions {
                    circle_mark_square(&board, action.end).await;
                }
            }

            if players[current_player] == 'e' {
                if timer < 0 {
                    timer = 200;
                    let action = board.get_random_action();
                    let action = get_minimax_action(&board, 2);
                    let result = board.commit_move(action);
                    match result {
                        Err(error) => panic!("{:?}", error),
                        Ok(_) => current_player = (current_player + 1) % 2,
                    };
                }
            } else {
                if let Some(location) = get_mouse_input(&board) {
                    let piece = board.get_piece_from_location(location);
                    if board.selected == None {
                        if let Some(piece) = piece {
                            if piece.color == board.turn {
                                board.selected = Some(location);
                            }
                        }
                    } else {
                        if piece != None && piece.unwrap().color == board.turn {
                            board.selected = Some(location);
                        } else {
                            let action =
                                board.get_action_from_locations(board.selected.unwrap(), location);
                            let result = board.commit_move(action);
                            match result {
                                Ok(_) => {
                                    // println!("ok");
                                    current_player = (current_player + 1) % 2;
                                }
                                Err(error) => {
                                    println!("Error: {:?}", error);
                                }
                            }
                            board.selected = None;
                        }
                    };
                }
            }
        }
        next_frame().await;
    }
}
