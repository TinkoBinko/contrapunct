use crate::utils::*;
use EngineKind::*;

pub enum EngineKind {
    Random,
    Minimax,
}
pub struct Engine {
    kind: EngineKind,
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
