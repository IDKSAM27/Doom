use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{Event, Key, Style};
use sfml::system::Clock;

mod map;

fn main() {
    let mut window = RenderWindow::new(
        (1600, 900),
        "Rusty Doom",
        Style::CLOSE,
        &Default::default(),
    )
    .expect("Failed to create window");

    window.set_vertical_sync_enabled(true);

    let mut clock = Clock::start().expect("Failed to start clock"); // ✅ FIXED

    let map = map::TileMap::new();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => {
                    window.close();
                }
                _ => {}
            }
        }

        window.clear(Color::WHITE);
        map.draw(&mut window);
        window.display();

        let _elapsed = clock.restart(); // ✅ Works now
    }
}
