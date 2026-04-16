mod ai;
mod board;
mod render;
mod tetromino;
use ai::*;
use board::*;
use macroquad::prelude::*;
use render::*;
use tetromino::*;

#[macroquad::main("Tetris Bot")]
async fn main() {
    let (sx, sy, cw, ch) = (10, 20, 30.0, 30.0);
    let (mut lines, mut last_up, ds) = (0, get_time(), 0.85);
    let (mut last_mv, mut das, mut cur_dir) = (0.0, 0.0, None);
    let (mut bot_t, mut bot_s, mpf) = (0.0, 0.0, 200);
    let (das_d, arr_i, sd_arr) = (0.15, 0.03, 0.01);
    rand::srand(miniquad::date::now() as u64);

    let mut board = Board::new();
    let mut bag = PieceBag::new();
    let mut active = Tetromino::new(bag.next());
    let (mut held, mut can_h) = (None, true);

    loop {
        clear_background(WHITE);
        let now = get_time();

        if now - last_up > ds {
            if active.can_move(&board, 0, 1) {
                active.y += 1;
            } else {
                board.lock_piece(&active);
                can_h = true;
                lines += board.clear_lines();
                active = Tetromino::new(bag.next());
                if !active.can_move(&board, 0, 0) {
                    board = Board::new();
                }
            }
            last_up = now;
        }

        for k in [KeyCode::Left, KeyCode::Right, KeyCode::Down] {
            if is_key_pressed(k) {
                let (dx, dy) = match k {
                    KeyCode::Left => (-1, 0),
                    KeyCode::Right => (1, 0),
                    _ => (0, 1),
                };
                if active.can_move(&board, dx, dy) {
                    active.x += dx;
                    active.y += dy;
                }
                cur_dir = Some(k);
                das = now;
                last_mv = now;
            }
        }

        if let Some(k) = cur_dir {
            if is_key_down(k) {
                let (dx, dy) = match k {
                    KeyCode::Left => (-1, 0),
                    KeyCode::Right => (1, 0),
                    _ => (0, 1),
                };
                let (e_das, e_arr) = if k == KeyCode::Down {
                    (0.0, sd_arr)
                } else {
                    (das_d, arr_i)
                };
                if now - das >= e_das && now - last_mv >= e_arr {
                    if active.can_move(&board, dx, dy) {
                        active.x += dx;
                        active.y += dy;
                    }
                    last_mv = now;
                }
            } else {
                cur_dir = None;
            }
        }

        if is_key_pressed(KeyCode::Up) {
            active.rotate(&board);
        }
        if is_key_pressed(KeyCode::Space) {
            while active.can_move(&board, 0, 1) {
                active.y += 1;
            }
            board.lock_piece(&active);
            can_h = true;
            lines += board.clear_lines();
            active = Tetromino::new(bag.next());
            last_up = now;
            if !active.can_move(&board, 0, 0) {
                board = Board::new();
            }
        }

        if is_key_pressed(KeyCode::D) && can_h {
            if let Some(mut al_h) = held.take() {
                let mut to_h = active;
                to_h.reset_pos();
                held = Some(to_h);
                al_h.reset_pos();
                active = al_h;
            } else {
                let mut to_h = active;
                to_h.reset_pos();
                held = Some(to_h);
                active = Tetromino::new(bag.next());
            }
            can_h = false;
        }

        let mut gy = active.y;
        while active.can_move(&board, 0, (gy - active.y) + 1) {
            gy += 1;
        }
        let (ox, oy) = (
            (screen_width() - sx as f32 * cw) / 2.0,
            (screen_height() - sy as f32 * ch) / 2.0,
        );

        board.draw(ox, oy, cw, ch);
        draw_ghost(&active, gy, ox, oy, cw, ch);
        draw_active(&active, ox, oy, cw, ch);
        render::draw_grid(ox, oy, sx, sy, cw, ch, GRAY);
        if let Some(ref p) = held {
            draw_preview(p, ox - 100.0, oy, 20.0, "HOLD");
        }
        draw_preview(
            &Tetromino::new(bag.peek_next()),
            ox + (sx as f32 * cw) + 20.0,
            oy,
            20.0,
            "NEXT",
        );
        draw_text(
            &format!("LINES CLEARED: {}", lines),
            screen_width() / 2.0 - 70.0,
            oy - 10.0,
            20.0,
            BLACK,
        );

        if is_key_pressed(KeyCode::Equal) {
            bot_s -= 0.01;
        }
        if is_key_pressed(KeyCode::Minus) {
            bot_s += 0.01;
        }
        draw_text(
            &format!("Bot Speed: {:.3}s", bot_s),
            10.0,
            50.0,
            20.0,
            BLACK,
        );

        let mpf_v = if bot_s > 0.0 { 1 } else { mpf };
        for _ in 0..mpf_v {
            if now - bot_t >= bot_s {
                if let Some(action) =
                    determine_best_bot_action(&board, &active, &held, &mut bag, can_h)
                {
                    match action {
                        BotAction::SwapAndMoveTo(tm) => {
                            if let Some(al_h) = held.take() {
                                let mut to_h = active.clone();
                                to_h.reset_pos();
                                held = Some(to_h);
                                active = al_h;
                            } else {
                                let mut to_h = active.clone();
                                to_h.reset_pos();
                                held = Some(to_h);
                                active = Tetromino::new(bag.next());
                            }
                            active.reset_pos();
                            active.apply_move(tm);
                        }
                        BotAction::MoveTo(tm) => {
                            active.apply_move(tm);
                        }
                    }
                    board.lock_piece(&active);
                    lines += board.clear_lines();
                    active = Tetromino::new(bag.next());
                    can_h = true;
                    if !active.can_move(&board, 0, 0) {
                        board = Board::new();
                    }
                }
                bot_t = now;
            }
        }
        next_frame().await;
    }
}
