use ggez::event::{self, EventHandler};
use ggez::graphics::{Color, Canvas};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};

mod map;
use map::Map;

struct Game {
    map: Map,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {
            map: Map::new(),
        }
    }

    fn check_events(&mut self, keycode: KeyCode, ctx: &mut Context) {
        if keycode == KeyCode::Escape {
            ctx.request_quit();
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        self.map.draw(ctx, &mut canvas)?;
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeat: bool,
    ) -> GameResult {
        if let Some(keycode) = input.keycode {
            self.check_events(keycode, ctx);
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("rusty_doom", "author")
        .window_setup(ggez::conf::WindowSetup::default().title("Rusty Doom"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let game = Game::new(&mut ctx);
    event::run(ctx, event_loop, game)
}
