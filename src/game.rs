use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle, draw_text};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];
const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

const BASE_MOVING_PERIOD: f64 = 0.3;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    difficulty: i32,
    difficulty_selected: bool,
    game_started: bool,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            difficulty: 5, // Default difficulty level (1-10)
            difficulty_selected: false,
            game_started: false,
        }
    }

    fn get_moving_period(&self) -> f64 {
        // Higher difficulty = faster speed = lower period
        // Difficulty 1: slowest (0.5s), Difficulty 10: fastest (0.05s)
        BASE_MOVING_PERIOD * (11.0 - self.difficulty as f64) / 10.0
    }

    pub fn key_pressed(&mut self, key: Key) {
        // If difficulty hasn't been selected yet, handle difficulty selection
        if !self.difficulty_selected {
            match key {
                Key::NumPad1 | Key::D1 => {
                    self.difficulty = 1;
                    self.difficulty_selected = true;
                    self.game_started = true;
                    return;
                }
                Key::NumPad2 | Key::D2 => {
                    self.difficulty = 2;
                    self.difficulty_selected = true;
                    self.game_started = true;
                    return;
                }
                Key::NumPad3 | Key::D3 => {
                    self.difficulty = 3;
                    self.difficulty_selected = true;
                    self.game_started = true;
                    return;
                }
                Key::NumPad4 | Key::D4 => {
                    self.difficulty = 4;
                    self.difficulty_selected = true;
                    self.game_started = true;
                    return;
                }
                Key::NumPad5 | Key::D5 => {
                    self.difficulty = 5;
                    self.difficulty_selected = true;
                    self.game_started = true;
                    return;
                }
                Key::NumPad6 | Key::D6 => {
                    self.difficulty = 6;
                    self.difficulty_selected = true;
                    self.game_started = true;
                    return;
                }
                Key::NumPad7 | Key::D7 => {
                    self.difficulty = 7;
                    self.difficulty_selected = true;
                    self.game_started = true;
                    return;
                }
                Key::NumPad8 | Key::D8 => {
                    self.difficulty = 8;
                    self.difficulty_selected = true;
                    self.game_started = true;
                    return;
                }
                Key::NumPad9 | Key::D9 => {
                    self.difficulty = 9;
                    self.difficulty_selected = true;
                    self.game_started = true;
                    return;
                }
                Key::NumPad0 | Key::D0 => {
                    self.difficulty = 10;
                    self.difficulty_selected = true;
                    self.game_started = true;
                    return;
                }
                _ => return, // Ignore other keys during difficulty selection
            }
        }

        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };

        if let Some(dir) = dir {
            if dir == self.snake.head_direction().opposite() {
                return;
            }
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        // If difficulty hasn't been selected, show difficulty selection screen
        if !self.difficulty_selected {
            self.draw_difficulty_selection(con, g, glyphs);
            return;
        }

        // Draw the game
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        // Display current difficulty (smaller, less intrusive)
        let difficulty_text = format!("Difficulty: {}", self.difficulty);
        draw_text(TEXT_COLOR, &difficulty_text, 10.0, 25.0, 16, glyphs, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
            draw_text(
                TEXT_COLOR,
                "GAME OVER - Press any key to restart",
                150.0,
                350.0,
                24,
                glyphs,
                con,
                g,
            );
        }
    }

    fn draw_difficulty_selection(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        // Draw a semi-transparent background
        draw_rectangle([0.0, 0.0, 0.0, 0.8], 0, 0, self.width, self.height, con, g);

        // Title
        draw_text(TEXT_COLOR, "SNAKE GAME", 250.0, 200.0, 48, glyphs, con, g);

        // Instructions
        draw_text(
            TEXT_COLOR,
            "Select Difficulty Level (1-10):",
            200.0,
            280.0,
            32,
            glyphs,
            con,
            g,
        );

        // Difficulty options
        draw_text(
            TEXT_COLOR,
            "1-3: Easy (Slow)",
            220.0,
            330.0,
            24,
            glyphs,
            con,
            g,
        );

        draw_text(TEXT_COLOR, "4-6: Medium", 220.0, 360.0, 24, glyphs, con, g);

        draw_text(
            TEXT_COLOR,
            "7-9: Hard (Fast)",
            220.0,
            390.0,
            24,
            glyphs,
            con,
            g,
        );

        draw_text(
            TEXT_COLOR,
            "0: Expert (Very Fast)",
            220.0,
            420.0,
            24,
            glyphs,
            con,
            g,
        );

        // Current selection hint
        draw_text(
            [1.0, 1.0, 0.0, 1.0], // Yellow color
            "Press a number key to start!",
            200.0,
            480.0,
            28,
            glyphs,
            con,
            g,
        );
    }

    pub fn update(&mut self, delta_time: f64) {
        // Don't update game logic until difficulty is selected and game has started
        if !self.difficulty_selected || !self.game_started {
            return;
        }

        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > self.get_moving_period() {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
        // Keep the current difficulty level and selection state when restarting
        // self.difficulty_selected and self.game_started remain true
    }
}
