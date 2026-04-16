use crate::board::Board;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub x: i32,
    pub y: i32,
    pub rotation_state: usize,
}

#[derive(Clone)]
pub struct Tetromino {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub matrix: [[bool; 4]; 4],
    pub rotation_state: usize,
    pub shape_index: usize,
}

pub struct PieceBag {
    queue: Vec<usize>,
}

impl Tetromino {
    pub fn new(shape_index: usize) -> Self {
        Self {
            x: 3,
            y: 0,
            color: COLORS[shape_index],
            matrix: ROTATED_SHAPES[shape_index][0],
            rotation_state: 0,
            shape_index,
        }
    }

    pub fn reset_pos(&mut self) {
        self.x = 3;
        self.y = 0;
        self.rotation_state = 0;
        self.matrix = ROTATED_SHAPES[self.shape_index][0];
    }

    pub fn apply_move(&mut self, target: Move) {
        self.x = target.x;
        self.y = target.y;
        self.rotation_state = target.rotation_state;
        self.matrix = ROTATED_SHAPES[self.shape_index][self.rotation_state];
    }

    pub fn rotate(&mut self, board: &Board) {
        if self.color == YELLOW { return; }

        let old_state = self.rotation_state;
        let old_x = self.x;
        let old_y = self.y;

        let new_state = (old_state + 1) % 4;
        self.matrix = ROTATED_SHAPES[self.shape_index][new_state];

        let transition_idx = match (old_state, new_state) {
            (0, 1) => 0, (1, 0) => 1, (1, 2) => 2, (2, 1) => 3,
            (2, 3) => 4, (3, 2) => 5, (3, 0) => 6, (0, 3) => 7,
            _ => 0,
        };

        let tests = if self.color == SKYBLUE { I_KICK_TABLE[transition_idx] } else { SRS_KICK_TABLE[transition_idx] };

        let mut success = false;
        for (dx, dy) in tests.iter() {
            if self.can_move(board, *dx, *dy) {
                self.x += dx;
                self.y += dy;
                self.rotation_state = new_state;
                success = true;
                break;
            }
        }

        if !success {
            self.matrix = ROTATED_SHAPES[self.shape_index][old_state];
            self.x = old_x;
            self.y = old_y;
        }
    }

    pub fn can_move(&self, board: &Board, dx: i32, dy: i32) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if self.matrix[row][col] {
                    let new_x = self.x + col as i32 + dx;
                    let new_y = self.y + row as i32 + dy;

                    if new_x < 0 || new_x >= 10 || new_y >= 20 { return false; }
                    if new_y >= 0 && board.cells[new_y as usize][new_x as usize].is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl PieceBag {
    pub fn new() -> Self {
        let mut s = Self { queue: Vec::new() };
        s.refill();
        s
    }

    pub fn refill(&mut self) {
        let mut new_bag = vec![0, 1, 2, 3, 4, 5, 6];
        for i in 0..new_bag.len() {
            let j = rand::gen_range(0, new_bag.len());
            new_bag.swap(i, j);
        }
        self.queue.extend(new_bag);
    }

    pub fn next(&mut self) -> usize {
        if self.queue.len() <= 1 { self.refill(); }
        self.queue.remove(0)
    }

    pub fn peek_next(&self) -> usize { self.queue[0] }
}

pub const SHAPES: [[[bool; 4]; 4]; 7] = [
    [[false, false, false, false], [true, true, true, true], [false, false, false, false], [false, false, false, false]],
    [[true, false, false, false], [true, true, true, false], [false, false, false, false], [false, false, false, false]],
    [[false, false, true, false], [true, true, true, false], [false, false, false, false], [false, false, false, false]],
    [[false, true, true, false], [false, true, true, false], [false, false, false, false], [false, false, false, false]],
    [[false, true, true, false], [true, true, false, false], [false, false, false, false], [false, false, false, false]],
    [[false, true, false, false], [true, true, true, false], [false, false, false, false], [false, false, false, false]],
    [[true, true, false, false], [false, true, true, false], [false, false, false, false], [false, false, false, false]],
];

const fn compute_rotations(base: [[[bool; 4]; 4]; 7]) -> [[[[bool; 4]; 4]; 4]; 7] {
    let mut result = [[[[false; 4]; 4]; 4]; 7];
    let mut p = 0;
    while p < 7 {
        result[p][0] = base[p];
        let mut state = 1;
        while state < 4 {
            let mut y = 0;
            while y < 4 {
                let mut x = 0;
                while x < 4 {
                    if p == 0 { result[p][state][x][3 - y] = result[p][state - 1][y][x]; }
                    else if p == 3 { result[p][state][y][x] = result[p][state - 1][y][x]; }
                    else if x < 3 && y < 3 { result[p][state][x][2 - y] = result[p][state - 1][y][x]; }
                    x += 1;
                }
                y += 1;
            }
            state += 1;
        }
        p += 1;
    }
    result
}

pub const ROTATED_SHAPES: [[[[bool; 4]; 4]; 4]; 7] = compute_rotations(SHAPES);
pub const COLORS: [Color; 7] = [SKYBLUE, BLUE, ORANGE, YELLOW, GREEN, PURPLE, RED];

const SRS_KICK_TABLE: [[(i32, i32); 5]; 8] = [
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)], [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)], [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)], [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)], [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
];

const I_KICK_TABLE: [[(i32, i32); 5]; 8] = [
    [(0, 0), (-2, 0), (1, 0), (-2, 1), (1, -2)], [(0, 0), (2, 0), (-1, 0), (2, -1), (-1, 2)],
    [(0, 0), (-1, 0), (2, 0), (-1, -2), (2, 1)], [(0, 0), (1, 0), (-2, 0), (1, 2), (-2, -1)],
    [(0, 0), (2, 0), (-1, 0), (2, -1), (-1, 2)], [(0, 0), (-2, 0), (1, 0), (-2, 1), (1, -2)],
    [(0, 0), (1, 0), (-2, 0), (1, 2), (-2, -1)], [(0, 0), (-1, 0), (2, 0), (-1, -2), (2, 1)],
];
