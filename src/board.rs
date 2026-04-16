use crate::tetromino::Tetromino;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Board {
    pub cells: [[Option<Color>; 10]; 20],
}

impl Board {
    pub fn new() -> Self {
        Self { cells: [[None; 10]; 20] }
    }

    pub fn lock_piece(&mut self, active_piece: &Tetromino) {
        for row in 0..4 {
            for col in 0..4 {
                if active_piece.matrix[row][col] {
                    let board_x = active_piece.x + col as i32;
                    let board_y = active_piece.y + row as i32;
                    if board_y >= 0 && board_y < 20 && board_x >= 0 && board_x < 10 {
                        self.cells[board_y as usize][board_x as usize] = Some(active_piece.color);
                    }
                }
            }
        }
    }

    pub fn clear_lines(&mut self) -> i32 {
        let mut lines_cleared = 0;
        let mut y = 20 - 1;
        while y >= 0 {
            let is_full = self.cells[y as usize].iter().all(|cell| cell.is_some());
            if is_full {
                lines_cleared += 1;
                for row_to_move in (1..=y as usize).rev() {
                    self.cells[row_to_move] = self.cells[row_to_move - 1];
                }
                self.cells[0] = [None; 10];
            } else {
                if y == 0 { break; }
                y -= 1;
            }
        }
        lines_cleared
    }
}
