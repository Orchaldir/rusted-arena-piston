use crate::math::Point;

pub struct TileRenderer {
    start: Point,
    tile_size: Point,
}

impl TileRenderer {
    pub fn new(start: Point, tile_size: Point) -> TileRenderer {
        TileRenderer { start, tile_size }
    }

    pub fn render_char<R>(&self, renderer: &mut R, c: char, pos: &Point, color: [f32; 4])
    where
        R: super::Renderer,
    {
        let pos = &self.start + pos * &self.tile_size;

        renderer.render_char(c, &pos, self.tile_size.y as u32, color);
    }

    pub fn render_full<R>(&self, renderer: &mut R, pos: &Point, color: [f32; 4])
    where
        R: super::Renderer,
    {
        let pos = &self.start + pos * &self.tile_size;

        renderer.render_rectangle(&pos, &self.tile_size, color);
    }
}

#[cfg(test)]
mod tests {
    use super::super::MockRenderer;
    use super::*;

    #[test]
    fn test_render_full() {
        let start = Point { x: 1, y: 2 };
        let size = Point { x: 15, y: 25 };
        let pos = Point { x: 3, y: 4 };
        let color = [0.1, 0.2, 0.3, 0.4];

        let mut mock = MockRenderer::new();
        mock.expect_render_rectangle()
            .times(1)
            .withf(|p: &Point, s: &Point, c: &[f32; 4]| {
                p.x == 46
                    && p.y == 102
                    && s.x == 15
                    && s.y == 25
                    && c[0] == 0.1
                    && c[1] == 0.2
                    && c[2] == 0.3
                    && c[3] == 0.4
            })
            .return_const(());

        let tile_renderer = TileRenderer::new(start, size);

        tile_renderer.render_full(&mut mock, &pos, color);
    }
}
