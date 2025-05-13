use sfml::graphics::{Color, RenderTarget, RenderWindow, RenderWindowExt, Style};
use sfml::window::{Event, Key};
use sfml::system::Clock;

mod map;

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Rusty Doom",
        Style::CLOSE,
        &Default::default(),
    )
    .expect("Failed to create window");

    window.set_vertical_sync_enabled(true);

    let mut clock = Clock::start();

    let map = map::TileMap::new();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::ESCAPE, .. } => {
                    window.close();
                }
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        map.draw(&mut window);
        window.display();

        let _elapsed = clock.restart();
    }
}
