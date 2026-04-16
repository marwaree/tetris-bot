use crate::board::Board;
use crate::tetromino::{Move, Tetromino, PieceBag};
use std::collections::{HashSet, VecDeque};

pub enum BotAction {
    MoveTo(Move),
    SwapAndMoveTo(Move),
}

impl Board {
    pub fn get_all_possible_moves(&self, active_piece: &Tetromino) -> Vec<Move> {
        let mut possible_moves = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        let start_state = Move { x: active_piece.x, y: active_piece.y, rotation_state: active_piece.rotation_state };
        queue.push_back(start_state);
        visited.insert((start_state.x, start_state.y, start_state.rotation_state));

        let mut sim = active_piece.clone();
        while let Some(current) = queue.pop_front() {
            sim.apply_move(current);
            if !sim.can_move(self, 0, 1) {
                if !possible_moves.contains(&current) { possible_moves.push(current); }
            }
            let translations = [(-1, 0), (1, 0), (0, 1)];
            for (dx, dy) in translations {
                if sim.can_move(self, dx, dy) {
                    let next = Move { x: current.x + dx, y: current.y + dy, rotation_state: current.rotation_state };
                    if visited.insert((next.x, next.y, next.rotation_state)) { queue.push_back(next); }
                }
            }
            let mut rot_sim = sim.clone();
            rot_sim.rotate(self);
            let next_rot = Move { x: rot_sim.x, y: rot_sim.y, rotation_state: rot_sim.rotation_state };
            if visited.insert((next_rot.x, next_rot.y, next_rot.rotation_state)) { queue.push_back(next_rot); }
        }
        possible_moves
    }

    pub fn get_col_height(&self, x: usize) -> usize {
        for y in 0..20 { if self.cells[y][x].is_some() { return 20 - y; } }
        0
    }

    pub fn count_holes(&self) -> usize {
        let mut holes = 0;
        for x in 0..10 {
            let mut block_found = false;
            for y in 0..20 {
                if self.cells[y][x].is_some() { block_found = true; }
                else if block_found && self.cells[y][x].is_none() { holes += 1; }
            }
        }
        holes
    }

    pub fn get_bumpiness(&self) -> usize {
        let mut bumpiness = 0;
        for x in 0..9 {
            let h1 = self.get_col_height(x) as i32;
            let h2 = self.get_col_height(x + 1) as i32;
            bumpiness += (h1 - h2).abs() as usize;
        }
        bumpiness
    }

    pub fn get_aggregate_height(&self) -> usize {
        let mut total = 0;
        for x in 0..10 { total += self.get_col_height(x); }
        total
    }

    pub fn count_blocked_holes(&self) -> usize {
        let mut blockage_score = 0;
        for x in 0..10 {
            let mut found_hole_below = false;
            let mut blocks_above = 0;
            for y in (0..20).rev() {
                if self.cells[y][x].is_none() { found_hole_below = true; }
                else if self.cells[y][x].is_some() && found_hole_below { blocks_above += 1; }
            }
            blockage_score += blocks_above;
        }
        blockage_score
    }

    pub fn count_column_transitions(&self) -> usize {
        let mut transitions = 0;
        for x in 0..10 {
            for y in 0..19 {
                if self.cells[y][x].is_some() != self.cells[y + 1][x].is_some() { transitions += 1; }
            }
            if self.cells[19][x].is_none() { transitions += 1; }
        }
        transitions
    }

    pub fn evaluate_move(&self, active_piece: &Tetromino, m: Move) -> f32 {
        let mut test_board = self.clone();
        let mut test_piece = active_piece.clone();
        test_piece.apply_move(m);
        let landing_height = 20.0 - (test_piece.y as f32);
        test_board.lock_piece(&test_piece);
        let lines_cleared = test_board.clear_lines();
        let line_reward = match lines_cleared { 1 => 1.0, 2 => 4.0, 3 => 9.0, 4 => 100.0, _ => 0.0 };
        let (w_lh, w_lr, w_h, w_bh, w_b, w_ah) = (-4.5, 8.0, -18.0, -10.0, -2.5, -1.0);
        let mut score = (landing_height * w_lh) + (line_reward * w_lr) + (test_board.count_holes() as f32 * w_h)
            + (test_board.count_blocked_holes() as f32 * w_bh) + (test_board.get_bumpiness() as f32 * w_b)
            + (test_board.get_aggregate_height() as f32 * w_ah);
        score += test_board.count_column_transitions() as f32 * -8.0;
        score
    }
}

pub fn determine_best_bot_action(board: &Board, active_piece: &Tetromino, held_piece: &Option<Tetromino>, bag: &mut PieceBag, can_hold: bool) -> Option<BotAction> {
    let active_moves = board.get_all_possible_moves(active_piece);
    let best_active = active_moves.into_iter().map(|m| (m, board.evaluate_move(active_piece, m))).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut best_held = None;
    if can_hold {
        let mut piece_to_test = if let Some(held) = held_piece { held.clone() } else { Tetromino::new(bag.peek_next()) };
        piece_to_test.reset_pos();
        let held_moves = board.get_all_possible_moves(&piece_to_test);
        best_held = held_moves.into_iter().map(|m| (m, board.evaluate_move(&piece_to_test, m))).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    }
    match (best_active, best_held) {
        (Some((_am, ascore)), Some((hm, hscore))) if hscore > ascore => Some(BotAction::SwapAndMoveTo(hm)),
        (Some((am, _)), _) => Some(BotAction::MoveTo(am)),
        _ => None,
    }
}
