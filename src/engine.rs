use crate::utils::*;
use rand::seq::SliceRandom;
use EngineKind::*;

pub enum EngineKind {
    Random,
    Minimax,
}
pub struct Engine {
    kind: EngineKind,
}

pub fn get_minimax_actions(board: &Board, depth: usize) -> Vec<Action> {
    let func = if board.turn == PieceColor::First {
        f64::max
    } else {
        f64::min
    };
    let mut best = if board.turn == PieceColor::First {
        -f64::INFINITY
    } else {
        f64::INFINITY
    };

    let mut node = board.get_position_tree(depth);
    update_tree(&mut node);
    let mut actions: Vec<Action> = Vec::new();
    for child in node.children.iter() {
        let prev_best = best;
        best = func(best, child.value);
        if best != prev_best {
            actions = Vec::new();
        }
        if child.value == best {
            actions.push(child.action.unwrap())
        }
    }

    actions
}

pub fn get_minimax_action(board: &Board, depth: usize) -> Action {
    let actions = get_minimax_actions(board, depth);
    *actions.choose(&mut rand::thread_rng()).unwrap()
}

// impl Engine {
//     fn return_action(&self, board: &Board) -> Action {
//         match self.kind {
//             Random => board.get_random_action(),
//             Minimax => get_minimax_action(board),
//         }
//     }
// }
//
// fn get_minimax_action(board: &mut Board) -> Action {}
