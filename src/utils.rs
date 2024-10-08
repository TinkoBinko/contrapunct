use rand::seq::SliceRandom;
use ActionKind::*;
use CastlingKind::*;
use PieceColor::*;
use PieceKind::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CastlingKind {
    Short,
    Long,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    First,
    Second,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionKind {
    Normal,
    Capture,
    EnPassant,
    Castling(CastlingKind),
    Promotion(PieceKind),
}
#[derive(Debug)]
pub struct TreeNode {
    pub board: Board,
    pub action: Option<Action>,
    pub value: f64,
    pub children: Vec<TreeNode>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}
#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub start: Location,
    pub end: Location,
    pub kind: ActionKind,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
    pub moved: bool,
}
type Position = Vec<Vec<Option<Piece>>>;

#[derive(Debug, Clone)]
pub struct Board {
    pub size: usize,
    pub position: Position,
    pub turn: PieceColor,
    pub selected: Option<Location>,
    pub last_action: Option<Action>,
    pub action_list: Vec<Action>,
}

#[derive(Debug)]
pub enum MoveError {
    InvalidLocationStringLength,
    InvalidLocationString,
    InvalidAction,
    StartSquareEmpty,
    InvalidPieceColor,
    RemainsInCheck,
}

pub fn get_piece_kind_worth(kind: PieceKind) -> f64 {
    match kind {
        Pawn => 1.,
        Rook => 5.25,
        Knight => 3.45,
        Bishop => 3.55,
        Queen => 10.,
        King => 4.,
    }
}
pub fn opposite_color(color: PieceColor) -> PieceColor {
    if color == First {
        Second
    } else {
        First
    }
}
pub fn sign_of_i8(num: i8) -> i8 {
    if num > 0 {
        1
    } else if num < 0 {
        -1
    } else {
        0
    }
}
pub fn actions_to_ends(actions: Vec<Action>) -> Vec<Location> {
    actions.iter().map(|action| action.end).collect()
}
pub fn actions_to_algebraic_ends(actions: Vec<Action>) -> Vec<String> {
    actions_to_ends(actions)
        .iter()
        .map(|end| location_to_algebraic(*end))
        .collect()
}
pub fn algebraic_to_action(input: String) -> Action {
    let input = input.trim();
    if input.len() != 4 {
        panic!("Invalid move length");
    }
    let mut start = Location { row: 0, col: 0 };
    let mut end = Location { row: 0, col: 0 };
    let kind = ActionKind::Normal;

    let letters = "abcdefgh";

    for (i, ch) in input.chars().enumerate() {
        let lower = ch.to_ascii_lowercase();
        if i % 2 == 0 && letters.contains(lower) {
            let col = letters.find(lower).unwrap();
            if i == 0 {
                start.col = col;
            } else if i == 2 {
                end.col = col;
            }
        } else if ch.is_digit(10) && ch >= '1' && ch <= '8' {
            let row = ch.to_digit(10).unwrap() - 1;
            if i == 1 {
                start.row = row as usize;
            } else if i == 3 {
                end.row = row as usize;
            }
        };
    }

    Action { start, end, kind }
}
pub fn algebraic_to_location(input: String) -> Result<Location, MoveError> {
    let input = input.trim();
    if input.len() != 2 {
        return Err(MoveError::InvalidLocationStringLength);
    }
    let mut location = Location { row: 0, col: 0 };

    let letters = "abcdefgh";

    for (i, ch) in input.chars().enumerate() {
        let lower = ch.to_ascii_lowercase();
        if i == 0 && letters.contains(lower) {
            let col = letters.find(lower).unwrap();
            location.col = col;
        } else if i == 1 && ch.is_digit(10) && ch >= '1' && ch <= '8' {
            let row = ch.to_digit(10).unwrap() - 1;
            location.row = row as usize;
        } else {
            return Err(MoveError::InvalidLocationString);
        }
    }
    Ok(location)
}
pub fn location_to_algebraic(location: Location) -> String {
    let Location { row, col } = location;
    let mut string = String::new();

    let rank = (row as u8 + b'1') as char;
    let file = (col as u8 + b'a') as char;

    string.push(file);
    string.push(rank);

    string
}

pub fn count_last_layer(node: &TreeNode) -> usize {
    let mut sum = 0;
    for child in node.children.iter() {
        if child.children.is_empty() {
            sum += 1;
        }
        sum += count_last_layer(child);
    }
    sum
}
pub fn count_tree_nodes(node: &TreeNode) -> usize {
    let mut sum = 1;
    for child in node.children.iter() {
        sum += count_tree_nodes(child);
    }
    sum
}
pub fn update_tree(node: &mut TreeNode) {
    let func = if node.board.turn == First {
        f64::max
    } else {
        f64::min
    };
    let mut best = if node.board.turn == First {
        -f64::INFINITY
    } else {
        f64::INFINITY
    };
    let checkmate_worth = if node.board.turn == First {
        -f64::INFINITY
    } else {
        f64::INFINITY
    };
    for child in node.children.iter_mut() {
        update_tree(child);
        best = func(best, child.value);
    }
    if node.children.is_empty() {
        if node.board.is_checkmate() {
            node.value = checkmate_worth;
        } else {
            node.value = node.board.get_material_difference();
        }
    } else {
        node.value = best;
    }
}

fn is_valid_promotion(board: &Board, start: Location, end: Location, color: PieceColor) -> bool {
    let size = board.size;
    let dx = end.col as i8 - start.col as i8;
    let dy = end.row as i8 - start.row as i8;
    if is_valid_pawn_capture(board, size, start, end, color) {
        return true;
    }
    if board.get_piece_from_location(end) == None
        && is_valid_pawn_translation(board.size, start.row, dx, dy, color)
    {
        return true;
    }
    false
}
fn is_valid_en_passant(
    board: &Board,
    size: usize,
    start: Location,
    end: Location,
    color: PieceColor,
) -> bool {
    let dx = end.col as i8 - start.col as i8;
    let dy = end.row as i8 - start.row as i8;

    let direction: i8 = if color == First { -1 } else { 1 };
    // let home_row = if direction == 1 { 1 } else { size - 2 };
    let opposite_home_row = if direction == 1 { size - 2 } else { 1 };

    if dx.abs() == 1 && dy == direction {
        let end = Location {
            row: (end.row as i8 - direction) as usize,
            col: end.col,
        };
        if start.row as i8 != opposite_home_row as i8 - 2 * direction {
            return false;
        }
        if let Some(p) = board.get_piece_from_location(end) {
            if let Some(action) = board.last_action {
                if p.color != color && action.end.row == start.row && action.end.col == end.col {
                    return true;
                }
            }
        }
    }

    false
}
fn is_valid_pawn_capture(
    board: &Board,
    size: usize,
    start: Location,
    end: Location,
    color: PieceColor,
) -> bool {
    let dx = end.col as i8 - start.col as i8;
    let dy = end.row as i8 - start.row as i8;

    let direction: i8 = if color == First { -1 } else { 1 };
    // let home_row = if direction == 1 { 1 } else { size - 2 };
    // let opposite_home_row = if direction == 1 { size - 2 } else { 1 };

    if dx.abs() == 1 && dy == direction {
        if let Some(p) = board.get_piece_from_location(end) {
            if p.color != color {
                return true;
            }
        }
    }

    false
}
fn is_valid_pawn_translation(
    size: usize,
    start_row: usize,
    dx: i8,
    dy: i8,
    color: PieceColor,
) -> bool {
    let direction: i8 = if color == First { -1 } else { 1 };
    let home_row = if direction == 1 { 1 } else { size - 2 };

    if dx == 0 {
        if dy == direction {
            return true;
        };
        if start_row == home_row && dy == 2 * direction {
            return true;
        };
    }

    false
}
fn is_valid_rook_translation(dx: i8, dy: i8) -> bool {
    if (dy == 0) ^ (dx == 0) {
        return true;
    };

    false
}
fn is_valid_knight_translation(dx: i8, dy: i8) -> bool {
    if dx.abs() + dy.abs() == 3 && dx != 0 && dy != 0 {
        return true;
    };

    false
}
fn is_valid_bishop_translation(dx: i8, dy: i8) -> bool {
    if (dy.abs() == dx.abs()) && !(dx == 0 && dy == 0) {
        return true;
    };

    false
}
fn is_valid_queen_translation(dx: i8, dy: i8) -> bool {
    if is_valid_rook_translation(dx, dy) || is_valid_bishop_translation(dx, dy) {
        return true;
    };

    false
}
fn is_valid_king_translation(dx: i8, dy: i8) -> bool {
    if dx.abs() <= 1 && dy.abs() <= 1 && (dx != 0 || dy != 0) {
        return true;
    };

    false
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor) -> Piece {
        Piece {
            kind,
            color,
            moved: false,
        }
    }
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            size,
            position: vec![vec![None; size]; size],
            turn: First,
            selected: None,
            last_action: None,
            action_list: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.position = vec![vec![None; self.size]; self.size];
    }
    pub fn set_piece(&mut self, piece: Piece, location: Location) {
        let Location { row, col } = location;
        self.position[row][col] = Some(piece);
    }
    pub fn clear_piece(&mut self, location: Location) {
        let Location { row, col } = location;
        self.position[row][col] = None;
    }
    pub fn set_fen(&mut self, fen: &String) {
        self.clear();
        let mut row: usize = 0;
        let mut col: usize = 0;
        for char in fen.chars() {
            match char {
                '/' => {
                    row += 1;
                    col = 0;
                }
                '1'..='9' => {
                    col += char.to_digit(10).unwrap() as usize;
                }
                letter => {
                    let color = if char.is_lowercase() { Second } else { First };
                    let kind = match char.to_lowercase().next().unwrap() {
                        'p' => Pawn,
                        'r' => Rook,
                        'n' => Knight,
                        'b' => Bishop,
                        'q' => Queen,
                        'k' => King,
                        _ => {
                            panic!("Invalid FEN!");
                        }
                    };
                    let piece = Piece::new(kind, color);
                    self.set_piece(piece, Location { row, col });
                    col += 1;
                }
            };
        }
    }

    pub fn get_piece_from_location(&self, location: Location) -> Option<Piece> {
        let Location { row, col } = location;
        self.position[row][col]
    }
    pub fn get_location_from_piece(&self, piece: Piece) -> Option<Location> {
        let Piece { kind, color, moved } = piece;
        let mut location = Location { row: 0, col: 0 };
        let mut found_piece = false;

        for row in 0..self.position.len() {
            for col in 0..self.position[row].len() {
                if let Some(p) = self.position[row][col] {
                    if p.kind == kind && p.color == color {
                        found_piece = true;
                        location = Location { row, col };
                    }
                }
            }
        }

        if found_piece {
            Some(location)
        } else {
            None
        }
    }

    pub fn commit_move(&mut self, action: Action) -> Result<(), MoveError> {
        let test = self.test_move(action);
        if let Err(message) = test {
            return Err(message);
        }
        self.make_move(action);
        if let Some(last_action) = self.last_action {
            self.action_list.push(last_action);
        }
        self.last_action = Some(action);
        self.turn = opposite_color(self.turn);
        self.selected = None;
        Ok(())
    }
    pub fn make_move(&mut self, action: Action) {
        let Action { start, end, kind } = action;
        let start_piece = self.get_piece_from_location(start);

        if let Some(mut start_piece) = start_piece {
            start_piece.moved = true;
            match kind {
                Normal => {
                    self.clear_piece(start);
                    self.clear_piece(end);
                    self.set_piece(start_piece, end);
                }
                Capture => {
                    self.clear_piece(start);
                    self.set_piece(start_piece, end);
                }
                Castling(ckind) => {
                    let direction: i8 = if self.turn == First { -1 } else { 1 };
                    let home_row = if direction == 1 { 0 } else { self.size - 1 };
                    let rook_col = if ckind == Long { 0 } else { self.size - 1 };

                    let king_location = Location {
                        row: home_row,
                        col: 4,
                    };
                    let mut king = self.get_piece_from_location(king_location).unwrap();
                    king.moved = true;
                    let dir: i8 = if ckind == Long { -1 } else { 1 };
                    let rook_location = Location {
                        row: home_row,
                        col: rook_col,
                    };
                    let mut rook = self.get_piece_from_location(rook_location).unwrap();
                    rook.moved = true;
                    let new_king_location = Location {
                        row: home_row,
                        col: (4 + 2 * dir) as usize,
                    };
                    let new_rook_location = Location {
                        row: home_row,
                        col: (4 + dir) as usize,
                    };
                    self.clear_piece(king_location);
                    self.clear_piece(rook_location);
                    self.set_piece(king, new_king_location);
                    self.set_piece(rook, new_rook_location);
                }
                EnPassant => {
                    self.clear_piece(start);
                    self.clear_piece(Location {
                        row: start.row,
                        col: end.col,
                    });
                    self.set_piece(start_piece, end);
                }
                Promotion(pkind) => {
                    self.clear_piece(start);
                    self.clear_piece(end);
                    let new_piece = Piece::new(pkind, self.turn);
                    self.set_piece(new_piece, end);
                }
            };
        }
    }
    pub fn test_move(&mut self, action: Action) -> Result<(), MoveError> {
        let Action { start, end, kind } = action;
        let start_piece = self.get_piece_from_location(start);
        if start_piece == None {
            return Err(MoveError::StartSquareEmpty);
        }
        let start_piece = start_piece.unwrap();
        // let end_piece = self.get_piece_from_location(end);

        if self.turn != start_piece.color {
            // println!("It's {:?}'s turn", self.turn);
            return Err(MoveError::InvalidPieceColor);
        }

        if !self.is_valid_action(action) {
            // println!("Invalid action");
            return Err(MoveError::InvalidAction);
        };

        let current_position = self.position.clone();

        self.make_move(action);

        if self.is_check(self.turn) {
            self.position = current_position;
            return Err(MoveError::RemainsInCheck);
        }
        self.position = current_position;

        Ok(())
    }

    pub fn is_valid_translation(&self, action: Action) -> bool {
        let Action { start, end, kind } = action;

        let piece = self.get_piece_from_location(start);
        if piece == None {
            return false;
        };
        let piece = piece.unwrap();

        let color = piece.color;
        let size = self.size;
        let dx = end.col as i8 - start.col as i8;
        let dy = end.row as i8 - start.row as i8;

        match piece.kind {
            Pawn => is_valid_pawn_translation(size, start.row, dx, dy, color),
            Rook => is_valid_rook_translation(dx, dy),
            Knight => is_valid_knight_translation(dx, dy),
            Bishop => is_valid_bishop_translation(dx, dy),
            Queen => is_valid_queen_translation(dx, dy),
            King => is_valid_king_translation(dx, dy),
        }
    }
    pub fn is_valid_capture(&self, action: Action) -> bool {
        let Action { start, end, kind } = action;
        if kind != Capture {
            panic!("Not a capture kind.")
        };

        let piece = self.get_piece_from_location(start);
        if piece == None {
            return false;
        };
        let piece = piece.unwrap();

        let color = piece.color;
        let size = self.size;
        let dx = end.col as i8 - start.col as i8;
        let dy = end.row as i8 - start.row as i8;

        match piece.kind {
            Pawn => is_valid_pawn_capture(self, size, start, end, color),
            _ => self.is_valid_translation(action),
        }
    }
    pub fn is_path_blocked(&self, start: Location, end: Location) -> bool {
        let dx = end.col as i8 - start.col as i8;
        let dy = end.row as i8 - start.row as i8;

        if !is_valid_bishop_translation(dx, dy) && !is_valid_rook_translation(dx, dy) {
            return false;
        }

        let sx = sign_of_i8(dx);
        let sy = sign_of_i8(dy);

        let mut col = start.col as i8;
        let mut row = start.row as i8;

        col += sx;
        row += sy;

        while sy * row <= sy * (end.row as i8) && sx * col <= sx * (end.col as i8) {
            let location = Location {
                row: row as usize,
                col: col as usize,
            };

            let piece = self.get_piece_from_location(location);

            if piece != None {
                if !(col == end.col as i8 && row == end.row as i8) {
                    return true;
                }
            }

            col += sx;
            row += sy;
        }
        false
    }
    pub fn is_end_blocked(&self, start: Location, end: Location) -> bool {
        let start_piece = self.get_piece_from_location(start).unwrap();
        if let Some(piece) = self.get_piece_from_location(end) {
            if piece.color == start_piece.color {
                return true;
            }
            if start_piece.kind == Pawn {
                return true;
            }
        }
        false
    }
    pub fn is_valid_action(&self, action: Action) -> bool {
        let Action { start, end, kind } = action;

        let end_piece = self.get_piece_from_location(end);
        let start_piece = self.get_piece_from_location(start);
        if start_piece == None {
            return false;
        };
        if start_piece.unwrap().color != self.turn {
            return false;
        }

        match kind {
            Normal => {
                if end_piece != None {
                    return false;
                }
                if !self.is_valid_translation(action) {
                    return false;
                }
                if self.is_path_blocked(start, end) {
                    return false;
                }
                if self.is_end_blocked(start, end) {
                    return false;
                }
            }
            Capture => {
                if let Some(end_piece) = end_piece {
                    if end_piece.color == start_piece.unwrap().color {
                        return false;
                    }
                }
                if !self.is_valid_capture(action) {
                    return false;
                }
                if self.is_path_blocked(start, end) {
                    return false;
                }
            }
            Castling(kind) => {
                let direction: i8 = if self.turn == First { -1 } else { 1 };
                let home_row = if direction == 1 { 0 } else { self.size - 1 };
                let rook_col = if kind == Long { 0 } else { self.size - 1 };
                if ![2, 6].contains(&end.col) {
                    return false;
                };
                if start.row != home_row || end.row != home_row {
                    return false;
                }

                let king_location = Location {
                    row: home_row,
                    col: 4,
                };
                let king = self.get_piece_from_location(king_location);
                if let Some(king) = king {
                    if king.kind != King || king.moved == true {
                        return false;
                    }
                } else {
                    return false;
                }
                let rook_location = Location {
                    row: home_row,
                    col: rook_col,
                };
                let rook = self.get_piece_from_location(rook_location);
                if let Some(rook) = rook {
                    if rook.kind != Rook || rook.moved == true {
                        return false;
                    }
                } else {
                    return false;
                }

                let start = if kind == Long { 1 } else { self.size - 3 };
                let len = if kind == Long { 2 } else { 1 };
                for i in start..=start + len {
                    let location = Location {
                        row: home_row,
                        col: i,
                    };
                    if self.get_piece_from_location(location) != None
                        || (self.is_square_attacked(location, opposite_color(self.turn)) && i != 1)
                    {
                        return false;
                    }
                }
                return true;
            }
            EnPassant => return is_valid_en_passant(self, self.size, start, end, self.turn),
            Promotion(_) => return is_valid_promotion(self, start, end, self.turn),
        }

        true
    }

    pub fn get_valid_actions(&mut self, start: Location) -> Vec<Action> {
        let mut actions = Vec::new();
        for row in 0..self.position.len() {
            for col in 0..self.position[row].len() {
                let end = Location { row, col };
                let action = self.get_action_from_locations(start, end);
                if let Ok(_) = self.test_move(action) {
                    actions.push(action);
                }
            }
        }
        actions
    }
    pub fn get_all_valid_actions(&mut self) -> Vec<Action> {
        let mut actions = Vec::new();
        for row in 0..self.size {
            for col in 0..self.size {
                let location = Location { row, col };
                if let Some(piece) = self.get_piece_from_location(location) {
                    if piece.color != self.turn {
                        continue;
                    }
                    actions.extend(self.get_valid_actions(location));
                }
            }
        }
        actions
    }
    pub fn get_random_action(&mut self) -> Action {
        let actions = self.get_all_valid_actions();
        *actions.choose(&mut rand::thread_rng()).unwrap()
    }
    pub fn get_action_from_locations(&self, start: Location, end: Location) -> Action {
        let piece = self.get_piece_from_location(start);
        let end_piece = self.get_piece_from_location(end);
        let mut kind = Normal;

        if let Some(piece) = piece {
            if piece.kind == King && end.col.abs_diff(start.col) > 1 {
                if end.col as isize - start.col as isize > 0 {
                    kind = Castling(Short);
                } else {
                    kind = Castling(Long);
                }
            } else if piece.kind == Pawn {
                let direction: i8 = if piece.color == First { -1 } else { 1 };
                let last_row = if direction == 1 { self.size - 1 } else { 0 };
                let neighbor_location = Location {
                    row: start.row,
                    col: end.col,
                };
                let neighbor = self.get_piece_from_location(neighbor_location);
                if end.row == last_row {
                    kind = Promotion(Queen);
                } else if end_piece != None {
                    kind = Capture;
                } else if neighbor != None && start.col.abs_diff(end.col) == 1 {
                    kind = EnPassant;
                } else {
                    kind = Normal;
                }
            } else if end_piece != None && end_piece.unwrap().color != piece.color {
                kind = Capture
            }
        }
        Action { start, end, kind }
    }
    pub fn get_material(&self, color: PieceColor) -> f64 {
        let mut total = 0.;
        for row in 0..self.size {
            for col in 0..self.size {
                if let Some(piece) = self.position[row][col] {
                    if piece.color == color {
                        total += get_piece_kind_worth(piece.kind);
                    }
                }
            }
        }
        total
    }
    pub fn get_material_difference(&self) -> f64 {
        self.get_material(First) - self.get_material(Second)
    }

    pub fn count_valid_actions(&mut self) -> usize {
        self.get_all_valid_actions().len()
    }

    pub fn is_square_attacked(&self, end: Location, color: PieceColor) -> bool {
        for row in 0..self.size {
            for col in 0..self.size {
                let start = Location { row, col };
                if let Some(piece) = self.get_piece_from_location(start) {
                    if piece.color != color {
                        continue;
                    }
                    let action = Action {
                        start,
                        end,
                        kind: Capture,
                    };
                    if self.is_valid_capture(action)
                        && !self.is_path_blocked(action.start, action.end)
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn is_check(&self, color: PieceColor) -> bool {
        let king = Piece::new(King, color);
        let king_location = self.get_location_from_piece(king).unwrap();

        if self.is_square_attacked(king_location, opposite_color(self.turn)) {
            // println!("yes");
            return true;
        };
        false
    }
    pub fn is_moveless(&mut self) -> bool {
        self.count_valid_actions() == 0
    }
    pub fn is_checkmate(&mut self) -> bool {
        self.is_check(self.turn) && self.is_moveless()
    }

    pub fn get_next_actions_and_boards(&self) -> Vec<(Action, Board)> {
        let mut boards_and_actions = Vec::new();
        let mut board = self.clone();
        let actions = board.get_all_valid_actions();
        for action in actions.iter() {
            let mut board = self.clone();
            let _ = board.commit_move(*action);
            boards_and_actions.push((*action, board));
        }
        boards_and_actions
    }
    pub fn get_next_boards(&self) -> Vec<Board> {
        let mut boards = Vec::new();
        let mut board = self.clone();
        let actions = board.get_all_valid_actions();
        for action in actions.iter() {
            let mut board = self.clone();
            let _ = board.commit_move(*action);
            boards.push(board);
        }
        boards
    }
    pub fn get_position_tree(&self, depth: usize) -> TreeNode {
        fn build_tree(
            board: Board,
            action: Option<Action>,
            current_depth: usize,
            max_depth: usize,
        ) -> TreeNode {
            if current_depth >= max_depth {
                return TreeNode {
                    board,
                    action,
                    value: 0.,
                    children: Vec::new(),
                };
            }

            let mut children = Vec::new();
            let next_actions_and_boards = board.get_next_actions_and_boards();
            for (action, board) in next_actions_and_boards.iter() {
                let child_node =
                    build_tree(board.clone(), Some(*action), current_depth + 1, max_depth);
                children.push(child_node);
            }
            TreeNode {
                board,
                action,
                value: 0.,
                children,
            }
        }

        build_tree(self.clone(), self.last_action, 0, depth)
    }
}
