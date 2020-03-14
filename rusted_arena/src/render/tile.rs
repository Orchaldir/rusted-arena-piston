use crate::math::Point;

pub struct TileRenderer {
    start: Point,
    tile_size: Point,
}

impl TileRenderer {
    pub fn new(start: Point, tile_size: Point) -> TileRenderer {
        TileRenderer { start, tile_size }
    }

    pub fn render_full<R: super::Renderer>(&self, renderer: &mut R, pos: &Point, color: [f32; 4]) {
        let pos = &self.start + pos * &self.tile_size;

        renderer.render_rectangle(&pos, &self.tile_size, color);
    }
}
