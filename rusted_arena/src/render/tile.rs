pub struct TileRenderer {
    start: (u32, u32),
    tile_size: (u32, u32),
}

impl TileRenderer {
    pub fn new(start: (u32, u32), tile_size: (u32, u32)) -> TileRenderer {
        TileRenderer { start, tile_size }
    }

    pub fn render_full<R: super::Renderer>(
        &self,
        renderer: &mut R,
        pos: (u32, u32),
        color: [f32; 4],
    ) {
        let pos = (
            self.start.0 + pos.0 * self.tile_size.0,
            self.start.1 + pos.1 * self.tile_size.1,
        );

        renderer.render_rectangle(pos, self.tile_size, color);
    }
}
