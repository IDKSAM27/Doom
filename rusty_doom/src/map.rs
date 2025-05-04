use std::collections::HashMap;

use ggez::{Context, GameResult, graphics};

pub struct Map {
    mini_map: Vec<Vec<bool>>,
    world_map: HashMap<(usize, usize), bool>
}

impl Map {
    pub fn new() -> Self {
        let mini_map = vec![
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true],
            vec![true, false, false, true, true, true, true, false, false, false, true, true, true, false, false, true],
            vec![true; 16],
            vec![true; 16],
            vec![true; 16],
            vec![true; 16],
            vec![true; 16],
            vec![true; 16],
        ];

        let mut map = Map {
            mini_map,
            world_map: HashMap::new(),
        };
        map.get_map();
        map
    }

    fn get_map(&mut self){
        for (j, row) in self.mini_map.iter().enumerate() {
            for (i, &value) in row.iter().enumerate() {
                if value {
                    self.world_map.insert((i, j), value);
                }
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let tile_size = 100.0;
        let color = graphics::Color::from_rgb(169, 169, 169); // dark gray

        for &(x, y) in self.world_map.keys() {
            let rect = graphics::Rect::new(
                x as f32 * tile_size,
                y as f32 * tile_size,
                tile_size,
                tile_size,
            );
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(2.0),
                rect,
                color,
            )?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        Ok(())?
    }
}