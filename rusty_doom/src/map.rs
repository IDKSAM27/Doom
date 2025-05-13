use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::system::Vector2f;

pub struct TileMap {
    tiles: Vec<RectangleShape<'static>>,
}

impl TileMap {
    pub fn new() -> Self {
        let mut tiles = Vec::new();

        for y in 0..10 {
            for x in 0..10 {
                let mut tile = RectangleShape::new();
                tile.set_size(Vector2f::new(50.0, 50.0));
                tile.set_fill_color(Color::GREEN);
                tile.set_position(Vector2f::new(x as f32 * 50.0, y as f32 * 50.0));
                tiles.push(tile);
            }
        }

        TileMap { tiles }
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        for tile in &self.tiles {
            window.draw(tile);
        }
    }
}
