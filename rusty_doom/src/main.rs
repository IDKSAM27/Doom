mod settings;
mod map;

use sfml::graphics::{RenderWindow, RenderTarget, Color};
use sfml::window::{Event, Style, Key, VideoMode};
use sfml::system::Clock;
use map::Map;

fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new(settings::WIDTH, settings::HEIGHT, 32),
        "Rust SFML Game",
        Style::CLOSE,
        &Default::default(),
    );
    // window.set_vertical_sync_enabled(true);
    window.expect("REASON").set_vertical_sync_enabled(true);

    let mut clock = Clock::start();
    let mut map = Map::new();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed |
                Event::KeyPressed { code: Key::ESCAPE, .. } => return,
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        map.draw(&mut window);
        window.display();

        let _elapsed = clock.restart();
    }
}