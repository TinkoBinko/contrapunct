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
    let mut node = TreeNode {
        board: board.clone(),
        action: board.last_action,
        value: 0.,
        children: Vec::new(),
    };

    fn alpha_beta(
        node: &mut TreeNode,
        depth: usize,
        alpha: f64,
        beta: f64,
    ) -> (Option<Action>, f64) {
        let checkmate_worth = if node.board.turn == First {
            -f64::INFINITY
        } else {
            f64::INFINITY
        };
        let next_actions_and_boards = node.board.get_next_actions_and_boards();
        node.children = next_actions_and_boards
            .iter()
            .map(|(new_action, board)| TreeNode {
                board: board.clone(),
                action: Some(*new_action),
                value: 0.,
                children: Vec::new(),
            })
            .collect();

        if depth == 0 || node.children.is_empty() {
            if node.board.is_checkmate() {
                node.value = checkmate_worth;
            } else {
                node.value = node.board.get_material_difference();
            }
            return (node.action, node.value);
        }

        let mut alpha = alpha;
        let mut beta = beta;

        let mut best_action = None;

        if node.board.turn == First {
            // println!("First");
            let mut value = -f64::INFINITY;
            for child in node.children.iter_mut() {
                let (_, ab) = alpha_beta(child, depth - 1, alpha, beta);
                if ab > value {
                    value = ab;
                    best_action = child.board.last_action;
                }
                alpha = f64::max(alpha, value);
                if alpha >= beta {
                    break;
                }
            }
            return (best_action, value);
        } else {
            // println!("Second");
            let mut value = f64::INFINITY;
            for child in node.children.iter_mut() {
                let (_, ab) = alpha_beta(child, depth - 1, alpha, beta);
                if ab < value {
                    value = ab;
                    best_action = child.board.last_action;
                }
                beta = f64::min(beta, value);
                if beta <= alpha {
                    break;
                }
            }
            return (best_action, value);
        }
    }

    let alpha = -f64::INFINITY;
    let beta = f64::INFINITY;
    let (action, ab) = alpha_beta(&mut node, depth, alpha, beta);

    if let Some(action) = action {
        println!("{:.2}", ab);
        println!("{:?}", action);
        action
    } else {
        panic!("No valid action found")
    }
}
