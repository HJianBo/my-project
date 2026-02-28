use macroquad::prelude::*;
use macroquad::rand::gen_range;

const BOARD_W: i32 = 10;
const BOARD_H: i32 = 20;
const CELL: f32 = 28.0;
const OFFSET_X: f32 = 40.0;
const OFFSET_Y: f32 = 40.0;
const HORIZONTAL_REPEAT_DELAY: f32 = 0.15;
const HORIZONTAL_REPEAT_SLOW: f32 = 0.08;
const HORIZONTAL_REPEAT_MEDIUM: f32 = 0.05;
const HORIZONTAL_REPEAT_FAST: f32 = 0.03;

const SHAPES: [[[(i32, i32); 4]; 4]; 7] = [
    // I
    [
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(1, 0), (1, 1), (1, 2), (1, 3)],
    ],
    // O
    [
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
    ],
    // T
    [
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (2, 1), (1, 2)],
        [(0, 1), (1, 1), (2, 1), (1, 2)],
        [(1, 0), (0, 1), (1, 1), (1, 2)],
    ],
    // S
    [
        [(1, 0), (2, 0), (0, 1), (1, 1)],
        [(1, 0), (1, 1), (2, 1), (2, 2)],
        [(1, 1), (2, 1), (0, 2), (1, 2)],
        [(0, 0), (0, 1), (1, 1), (1, 2)],
    ],
    // Z
    [
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(2, 0), (1, 1), (2, 1), (1, 2)],
        [(0, 1), (1, 1), (1, 2), (2, 2)],
        [(1, 0), (0, 1), (1, 1), (0, 2)],
    ],
    // J
    [
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (1, 2)],
        [(0, 1), (1, 1), (2, 1), (2, 2)],
        [(1, 0), (1, 1), (0, 2), (1, 2)],
    ],
    // L
    [
        [(2, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 2)],
        [(0, 1), (1, 1), (2, 1), (0, 2)],
        [(0, 0), (1, 0), (1, 1), (1, 2)],
    ],
];

const PIECE_COLORS: [Color; 7] = [
    SKYBLUE, // I
    YELLOW,  // O
    PURPLE,  // T
    GREEN,   // S
    RED,     // Z
    BLUE,    // J
    ORANGE,  // L
];

#[derive(Clone, Copy)]
struct Piece {
    kind: usize,
    rot: usize,
    x: i32,
    y: i32,
}

struct Game {
    board: Vec<Vec<Option<usize>>>,
    active: Piece,
    drop_timer: f32,
    drop_interval: f32,
    horizontal_dir: i32,
    horizontal_hold_time: f32,
    horizontal_repeat_timer: f32,
    score: u32,
    lines: u32,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            board: vec![vec![None; BOARD_W as usize]; BOARD_H as usize],
            active: Piece {
                kind: 0,
                rot: 0,
                x: 3,
                y: -1,
            },
            drop_timer: 0.0,
            drop_interval: 0.5,
            horizontal_dir: 0,
            horizontal_hold_time: 0.0,
            horizontal_repeat_timer: 0.0,
            score: 0,
            lines: 0,
            game_over: false,
        };
        game.spawn_piece();
        game
    }

    fn spawn_piece(&mut self) {
        self.active = Piece {
            kind: gen_range(0, SHAPES.len() as i32) as usize,
            rot: 0,
            x: 3,
            y: -1,
        };

        if self.collides(self.active) {
            self.game_over = true;
        }
    }

    fn collides(&self, piece: Piece) -> bool {
        SHAPES[piece.kind][piece.rot].iter().any(|(dx, dy)| {
            let x = piece.x + dx;
            let y = piece.y + dy;
            x < 0
                || x >= BOARD_W
                || y >= BOARD_H
                || (y >= 0 && self.board[y as usize][x as usize].is_some())
        })
    }

    fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        let mut next = self.active;
        next.x += dx;
        next.y += dy;
        if self.collides(next) {
            return false;
        }
        self.active = next;
        true
    }

    fn try_rotate(&mut self) {
        let mut next = self.active;
        next.rot = (next.rot + 1) % 4;

        if !self.collides(next) {
            self.active = next;
            return;
        }

        // Very small wall-kick so pieces can rotate near borders.
        next.x -= 1;
        if !self.collides(next) {
            self.active = next;
            return;
        }

        next.x += 2;
        if !self.collides(next) {
            self.active = next;
        }
    }

    fn lock_piece(&mut self) {
        for (dx, dy) in SHAPES[self.active.kind][self.active.rot] {
            let x = self.active.x + dx;
            let y = self.active.y + dy;
            if y < 0 {
                self.game_over = true;
                continue;
            }
            self.board[y as usize][x as usize] = Some(self.active.kind);
        }

        if !self.game_over {
            self.clear_lines();
            self.spawn_piece();
        }
    }

    fn clear_lines(&mut self) {
        let mut new_board = vec![vec![None; BOARD_W as usize]; BOARD_H as usize];
        let mut write_row = BOARD_H - 1;
        let mut cleared = 0;

        for y in (0..BOARD_H as usize).rev() {
            if self.board[y].iter().all(Option::is_some) {
                cleared += 1;
            } else {
                new_board[write_row as usize] = self.board[y].clone();
                write_row -= 1;
            }
        }

        self.board = new_board;
        self.lines += cleared;
        self.score += match cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };

        self.drop_interval = (0.5 - self.lines as f32 * 0.01).max(0.1);
    }

    fn horizontal_repeat_interval(hold_time: f32) -> f32 {
        if hold_time >= 0.9 {
            HORIZONTAL_REPEAT_FAST
        } else if hold_time >= 0.4 {
            HORIZONTAL_REPEAT_MEDIUM
        } else {
            HORIZONTAL_REPEAT_SLOW
        }
    }

    fn update_horizontal_movement(&mut self, dt: f32) {
        let dir = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };

        if dir == 0 {
            self.horizontal_dir = 0;
            self.horizontal_hold_time = 0.0;
            self.horizontal_repeat_timer = 0.0;
            return;
        }

        if dir != self.horizontal_dir {
            self.horizontal_dir = dir;
            self.horizontal_hold_time = 0.0;
            self.horizontal_repeat_timer = 0.0;
            self.try_move(dir, 0);
            return;
        }

        self.horizontal_hold_time += dt;
        if self.horizontal_hold_time < HORIZONTAL_REPEAT_DELAY {
            return;
        }

        self.horizontal_repeat_timer += dt;
        let held_after_delay = self.horizontal_hold_time - HORIZONTAL_REPEAT_DELAY;
        let interval = Self::horizontal_repeat_interval(held_after_delay);

        while self.horizontal_repeat_timer >= interval {
            self.horizontal_repeat_timer -= interval;
            if !self.try_move(dir, 0) {
                break;
            }
        }
    }

    fn update(&mut self) {
        if self.game_over {
            if is_key_pressed(KeyCode::R) {
                *self = Game::new();
            }
            return;
        }

        let dt = get_frame_time();

        self.update_horizontal_movement(dt);

        if is_key_pressed(KeyCode::Up) {
            self.try_rotate();
        }

        if is_key_pressed(KeyCode::Space) {
            while self.try_move(0, 1) {}
            self.lock_piece();
            return;
        }

        self.drop_timer += if is_key_down(KeyCode::Down) {
            dt * 8.0
        } else {
            dt
        };

        if self.drop_timer >= self.drop_interval {
            self.drop_timer = 0.0;
            if !self.try_move(0, 1) {
                self.lock_piece();
            }
        }
    }

    fn draw_block(x: i32, y: i32, color: Color) {
        let px = OFFSET_X + x as f32 * CELL;
        let py = OFFSET_Y + y as f32 * CELL;
        draw_rectangle(px, py, CELL - 1.0, CELL - 1.0, color);
    }

    fn draw(&self) {
        clear_background(Color::from_rgba(20, 20, 26, 255));

        let board_px_w = BOARD_W as f32 * CELL;
        let board_px_h = BOARD_H as f32 * CELL;

        draw_rectangle_lines(
            OFFSET_X - 2.0,
            OFFSET_Y - 2.0,
            board_px_w + 4.0,
            board_px_h + 4.0,
            2.0,
            LIGHTGRAY,
        );

        for y in 0..BOARD_H as usize {
            for x in 0..BOARD_W as usize {
                if let Some(kind) = self.board[y][x] {
                    Self::draw_block(x as i32, y as i32, PIECE_COLORS[kind]);
                }
            }
        }

        for (dx, dy) in SHAPES[self.active.kind][self.active.rot] {
            let x = self.active.x + dx;
            let y = self.active.y + dy;
            if y >= 0 {
                Self::draw_block(x, y, PIECE_COLORS[self.active.kind]);
            }
        }

        let panel_x = OFFSET_X + board_px_w + 30.0;
        draw_text("Rust Tetris", panel_x, OFFSET_Y + 30.0, 34.0, WHITE);
        draw_text(
            &format!("Score: {}", self.score),
            panel_x,
            OFFSET_Y + 90.0,
            28.0,
            WHITE,
        );
        draw_text(
            &format!("Lines: {}", self.lines),
            panel_x,
            OFFSET_Y + 125.0,
            28.0,
            WHITE,
        );

        draw_text("Left/Right: move", panel_x, OFFSET_Y + 200.0, 22.0, GRAY);
        draw_text("Up: rotate", panel_x, OFFSET_Y + 228.0, 22.0, GRAY);
        draw_text("Down: soft drop", panel_x, OFFSET_Y + 256.0, 22.0, GRAY);
        draw_text("Space: hard drop", panel_x, OFFSET_Y + 284.0, 22.0, GRAY);

        if self.game_over {
            draw_rectangle(
                OFFSET_X,
                OFFSET_Y + board_px_h * 0.4,
                board_px_w,
                80.0,
                Color::from_rgba(0, 0, 0, 180),
            );
            draw_text(
                "Game Over",
                OFFSET_X + 56.0,
                OFFSET_Y + board_px_h * 0.47,
                44.0,
                RED,
            );
            draw_text(
                "Press R to restart",
                OFFSET_X + 40.0,
                OFFSET_Y + board_px_h * 0.53,
                30.0,
                WHITE,
            );
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Simple Rust Tetris".to_owned(),
        window_width: 560,
        window_height: 640,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
