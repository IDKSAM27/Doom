use sfml::graphics::{RenderWindow, Color, RectangleShape, RenderTarget, Transformable};
use sfml::system::Vector2f;
use std::collections::HashMap;

const TILE_SIZE: f32 = 100.0;

pub struct Map {
    pub world_map: HashMap<(i32, i32), i32>,
}

impl Map {
    pub fn new() -> Self {
        let mini_map = vec![
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,1,1,1,1,0,0,0,1,1,1,0,0,1],
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
        ];

        let mut world_map = HashMap::new();
        for (j, row) in mini_map.iter().enumerate() {
            for (i, &value) in row.iter().enumerate() {
                if value != 0 {
                    world_map.insert((i as i32, j as i32), value);
                }
            }
        }

        Map { world_map }
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        for (&(x, y), _) in &self.world_map {
            let mut rect = RectangleShape::new();
            rect.set_size(Vector2f::new(TILE_SIZE, TILE_SIZE));
            rect.set_position(Vector2f::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE));
            rect.set_fill_color(Color::TRANSPARENT);
            rect.set_outline_thickness(2.0);
            rect.set_outline_color(Color::DARK_GRAY);
            window.draw(&rect);
        }
    }
}