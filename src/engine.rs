use crate::utils::PieceColor::*;
use crate::utils::*;
use rand::seq::SliceRandom;

pub struct Player {
    pub kind: PlayerKind,
    pub depth: usize,
}
pub enum PlayerKind {
    Human,
    Random,
    Minimax,
    Pruning,
}
impl Player {
    pub fn new(kind: PlayerKind, depth: usize) -> Self {
        Player { kind, depth }
    }
    pub fn get_action(&self, board: &mut Board) -> Action {
        match self.kind {
            PlayerKind::Random => board.get_random_action(),
            PlayerKind::Minimax => get_minimax_action(board, self.depth),
            PlayerKind::Pruning => get_alpha_beta_action(board, self.depth),
            _ => panic!("Human wants move"),
        }
    }
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

pub fn get_alpha_beta_action(board: &Board, depth: usize) -> Action {
    let mut cloned_board = board.clone();
    println!("Turn: {:?}", board.turn);
    fn alpha_beta(board: &mut Board, depth: usize, alpha: f64, beta: f64) -> (Option<Action>, f64) {
        let checkmate_worth = if board.turn == First {
            -f64::INFINITY
        } else {
            f64::INFINITY
        };

        if board.is_checkmate() {
            return (board.last_action, checkmate_worth);
        }
        if board.is_moveless() {
            return (board.last_action, 0.);
        }
        if depth == 0 {
            return (board.last_action, board.get_material_difference());
        }

        let mut alpha = alpha;
        let mut beta = beta;

        let mut actions: Vec<Action> = Vec::new();
        let mut best_action = None;
        let mut best_value = if board.turn == First {
            -f64::INFINITY
        } else {
            f64::INFINITY
        };
        let mut next_boards = board.get_next_boards();
        for next_board in next_boards.iter_mut() {
            assert_ne!(next_board.turn, board.turn);
            let (_, value) = alpha_beta(next_board, depth - 1, alpha, beta);

            if board.turn == First {
                if value >= best_value {
                    if value > best_value {
                        actions = Vec::new();
                    }
                    best_action = next_board.last_action;
                    best_value = value;
                    actions.push(next_board.last_action.unwrap());
                }
                alpha = f64::max(alpha, best_value);
                if beta <= alpha {
                    break;
                }
            } else {
                if value <= best_value {
                    if value < best_value {
                        actions = Vec::new();
                    }
                    best_action = next_board.last_action;
                    best_value = value;
                    actions.push(next_board.last_action.unwrap());
                }
                beta = f64::min(beta, best_value);
                if alpha >= beta {
                    break;
                }
            }
        }
        (actions.choose(&mut rand::thread_rng()).cloned(), best_value)
        // (best_action, best_value)
    }

    let alpha = -f64::INFINITY;
    let beta = f64::INFINITY;
    let (action, ab) = alpha_beta(&mut cloned_board, depth, alpha, beta);

    if let Some(action) = action {
        println!("{:.2}", ab);
        action
    } else {
        panic!("No valid action found")
    }
}
