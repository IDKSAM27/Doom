use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use std::time::Duration;
use ggez::context::quit;

mod settings;
mod map;
use map::Map;

struct Game {
    map:Map,
}


impl Game {
    fn new(_ctx: &mut Context) -> Game {
        let map = Map::new();
        Game { map }
    }

    fn new_game(&mut self) {
        self.map = Map::new();
    }

    fn check_events(&mut self, ctx: &mut Context) {
        // ggez handles events in update/input callbacks, not in a loop like pygame
        // so we won't be implementing it here but use key_down_event instead
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Typically you'd update game state here
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.map.draw(ctx)?;
        graphics::present(ctx)?;
        ggez::timer::sleep(Duration::from_secs_f64(1.0 / settings::FPS as f64));
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymodes: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            ggez::event::quit(ctx);
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("game", "author")
        .window_setup(ggez::conf::WindowSetup::default().title("ggez Game"))
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(settings::RES.0, settings::RES.1),
        )
        .build()?;

    let game = Game::new(&mut ctx);
    event::run(ctx, event_loop, game)
}
