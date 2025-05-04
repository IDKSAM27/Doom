use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::{Context, GameResult};

const TILE_SIZE: f32 = 100.0;

pub struct Map {
    world_map: Vec<(usize, usize)>,
}

impl Map {
    pub fn new() -> Map {
        let mini_map = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let mut world_map = vec![];
        for (j, row) in mini_map.iter().enumerate() {
            for (i, &val) in row.iter().enumerate() {
                if val == 1 {
                    world_map.push((i, j));
                }
            }
        }

        Map { world_map }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut ggez::graphics::Canvas) -> GameResult {
        let mut mesh_builder = MeshBuilder::new();
        for (i, j) in &self.world_map {
            let rect = graphics::Rect::new(
                *i as f32 * TILE_SIZE,
                *j as f32 * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
            );
            mesh_builder.rectangle(DrawMode::stroke(2.0), rect, Color::from_rgb(169, 169, 169));
        }

        let mesh_data = mesh_builder.build(); // build mesh data without context
        let mesh = graphics::Mesh::from_data(ctx, mesh_data); // convert to drawable mesh
        canvas.draw(&mesh, DrawParam::default());
        Ok(())
    }
}
