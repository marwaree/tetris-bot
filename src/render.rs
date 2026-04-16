use crate::board::Board;
use crate::tetromino::Tetromino;
use macroquad::prelude::*;

impl Board {
    pub fn draw(&self, offset_x: f32, offset_y: f32, width: f32, height: f32) {
        for y in 0..20 {
            for x in 0..10 {
                if let Some(cell_color) = self.cells[y][x] {
                    draw_rectangle(
                        offset_x + x as f32 * width,
                        offset_y + y as f32 * height,
                        width,
                        height,
                        cell_color,
                    );
                }
            }
        }
    }
}

pub fn draw_grid(offset_x: f32, offset_y: f32, sx: i32, sy: i32, w: f32, h: f32, color: Color) {
    for i in 0..=sx {
        draw_line(
            w * i as f32 + offset_x,
            offset_y,
            w * i as f32 + offset_x,
            h * sy as f32 + offset_y,
            1.0,
            color,
        );
    }
    for i in 0..=sy {
        draw_line(
            offset_x,
            h * i as f32 + offset_y,
            w * sx as f32 + offset_x,
            h * i as f32 + offset_y,
            1.0,
            color,
        );
    }
}

pub fn draw_active(piece: &Tetromino, ox: f32, oy: f32, cw: f32, ch: f32) {
    for r in 0..4 {
        for c in 0..4 {
            if piece.matrix[r][c] {
                draw_rectangle(
                    ox + (piece.x + c as i32) as f32 * cw,
                    oy + (piece.y + r as i32) as f32 * ch,
                    cw,
                    ch,
                    piece.color,
                );
            }
        }
    }
}

pub fn draw_ghost(piece: &Tetromino, gy: i32, ox: f32, oy: f32, cw: f32, ch: f32) {
    let mut gc = piece.color;
    gc.a = 0.5;
    for r in 0..4 {
        for c in 0..4 {
            if piece.matrix[r][c] {
                draw_rectangle(
                    ox + (piece.x + c as i32) as f32 * cw,
                    oy + (gy + r as i32) as f32 * ch,
                    cw,
                    ch,
                    gc,
                );
                draw_rectangle_lines(
                    ox + (piece.x + c as i32) as f32 * cw,
                    oy + (gy + r as i32) as f32 * ch,
                    cw,
                    ch,
                    2.0,
                    piece.color,
                );
            }
        }
    }
}

pub fn draw_preview(piece: &Tetromino, x: f32, y: f32, s: f32, label: &str) {
    draw_text(label, x, y - 10.0, 20.0, BLACK);
    draw_rectangle_lines(x, y, s * 4.0, s * 4.0, 2.0, LIGHTGRAY);
    for r in 0..4 {
        for c in 0..4 {
            if piece.matrix[r][c] {
                draw_rectangle(x + c as f32 * s, y + r as f32 * s, s, s, piece.color);
            }
        }
    }
}
