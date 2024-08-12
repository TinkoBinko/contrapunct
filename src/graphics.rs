use crate::utils::{Board, Location, Piece, PieceColor::*, PieceKind::*};
use macroquad::prelude::*;

pub async fn draw_piece(board: &Board, location: Location, piece: Piece) {
    let square_size = f32::max(screen_width(), screen_height()) / board.size as f32;
    let position = Vec2::new(
        location.col as f32 * square_size,
        location.row as f32 * square_size,
    );
    let constant = 0.8;
    let color = if piece.color == First { WHITE } else { BLACK };
    let opposite_color = if piece.color == First { BLACK } else { WHITE };

    let far = square_size * constant;
    let close = square_size * (1.0 - constant);
    let mid = square_size * 0.5;

    let bottom_left = Vec2::new(close, far);
    let bottom_right = Vec2::new(far, far);
    let top_left = Vec2::new(close, close);
    let top_right = Vec2::new(far, close);
    let bottom = Vec2::new(mid, far);
    let top = Vec2::new(mid, close);
    let right = Vec2::new(far, mid);
    let left = Vec2::new(close, mid);
    let center = Vec2::new(mid, mid);

    draw_triangle(
        top + position,
        bottom_left + position,
        bottom_right + position,
        color,
    );
    match piece.kind {
        Pawn => {
            draw_circle(
                position.x + mid,
                position.y + (mid + close) / 2.0,
                close,
                color,
            );
        }
        Rook => {
            draw_rectangle(
                position.x + close,
                position.y + far,
                far - close,
                -close,
                color,
            );
            draw_rectangle(
                position.x + close,
                position.y + close,
                far - close,
                close,
                color,
            );
        }
        Knight => draw_triangle(
            top_right + position,
            bottom_right + position,
            top_left + position,
            color,
        ),
        Bishop => {
            //
            // draw_ellipse(
            //     position.x + mid,
            //     position.y + close * 1.2,
            //     close / 1.2,
            //     close,
            //     0.,
            //     opposite_color,
            // )
        }
        Queen => draw_triangle(
            center + position,
            top_left + position,
            top_right + position,
            color,
        ),
        King => {
            draw_circle(position.x + mid, position.y + close, close / 1.2, color);
            draw_circle(
                position.x + mid - close,
                position.y + mid / 1.2,
                close / 1.2,
                color,
            );
            draw_circle(
                position.x + mid + close,
                position.y + mid / 1.2,
                close / 1.2,
                color,
            );
        }
    };
}
pub async fn draw_pieces(board: &Board) {
    for row in 0..board.size {
        for col in 0..board.size {
            let location = Location { row, col };
            let piece = board.get_piece_from_location(location);
            if let Some(piece) = piece {
                draw_piece(board, location, piece).await;
            }
        }
    }
}
pub async fn draw_board(board: &Board) {
    let square_size = f32::max(screen_width(), screen_height()) / board.size as f32;
    for row in 0..board.size {
        for col in 0..board.size {
            let color = if (col + row) % 2 == 0 { GRAY } else { BROWN };
            draw_rectangle(
                square_size * col as f32,
                square_size * row as f32,
                square_size,
                square_size,
                color,
            );
        }
    }
}
pub async fn highlight_square(board: &Board, location: Location) {
    let Location { row, col } = location;
    let square_size = f32::max(screen_width(), screen_height()) / board.size as f32;
    let color = if (col + row) % 2 == 0 { GREEN } else { LIME };
    draw_rectangle(
        square_size * col as f32,
        square_size * row as f32,
        square_size,
        square_size,
        color,
    );
}
pub async fn circle_mark_square(board: &Board, location: Location) {
    let Location { row, col } = location;
    let square_size = f32::max(screen_width(), screen_height()) / board.size as f32;
    let color = if (col + row) % 2 == 0 { GREEN } else { LIME };
    draw_circle(
        square_size * (col as f32 + 0.5),
        square_size * (row as f32 + 0.5),
        0.1 * square_size,
        color,
    );
}
pub async fn draw_check(board: &Board) {
    if board.is_check(board.turn) {
        let king = Piece::new(King, board.turn);
        let king_location = board.get_location_from_piece(king).unwrap();

        let Location { row, col } = king_location;
        let square_size = f32::max(screen_width(), screen_height()) / board.size as f32;
        draw_rectangle(
            square_size * col as f32,
            square_size * row as f32,
            square_size,
            square_size,
            RED,
        );
    }
}

pub fn get_mouse_input(board: &Board) -> Option<Location> {
    let square_size = f32::max(screen_width(), screen_height()) / board.size as f32;
    if is_mouse_button_pressed(MouseButton::Left) {
        let col = (mouse_position().0 / square_size) as usize;
        let row = (mouse_position().1 / square_size) as usize;
        return Some(Location { row, col });
    }
    None
}
