extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::draw::to_coord_u32;
use crate::game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Load font for text rendering
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap_or_else(|_| std::path::PathBuf::from("."));

    // Try different font paths
    let font_paths = [
        assets.join("FiraSans-Regular.ttf"),
        assets.join("font.ttf"),
        std::path::PathBuf::from("/System/Library/Fonts/Helvetica.ttc"), // macOS
        std::path::PathBuf::from("/System/Library/Fonts/Arial.ttf"),     // macOS
        std::path::PathBuf::from("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"), // Linux
    ];

    let mut glyphs = None;
    for font_path in &font_paths {
        if font_path.exists() {
            if let Ok(g) = window.load_font(font_path) {
                glyphs = Some(g);
                break;
            }
        }
    }

    let mut glyphs =
        glyphs.expect("Could not load any font. Please ensure you have a font file available.");

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            game.draw(&c, g, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
