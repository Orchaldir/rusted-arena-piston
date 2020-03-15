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
        let pos = self.calculate_start(pos);

        renderer.render_char(c, &pos, self.tile_size.y as u32, color);
    }

    pub fn render_full<R>(&self, renderer: &mut R, pos: &Point, color: [f32; 4])
    where
        R: super::Renderer,
    {
        let pos = self.calculate_start(pos);

        renderer.render_rectangle(&pos, &self.tile_size, color);
    }

    fn calculate_start(&self, pos: &Point) -> Point {
        &self.start + pos * &self.tile_size
    }
}

#[cfg(test)]
mod tests {
    use super::super::MockRenderer;
    use super::*;

    const C: char = 'T';
    const START: Point = Point { x: 1, y: 2 };
    const SIZE: Point = Point { x: 15, y: 25 };
    const POS: Point = Point { x: 3, y: 4 };
    const COLOR: [f32; 4] = [0.1, 0.2, 0.3, 0.4];

    #[test]
    fn test_render_char() {
        let mut mock = MockRenderer::new();
        mock.expect_render_char()
            .times(1)
            .withf(|t: &char, p: &Point, s: &u32, c: &[f32; 4]| {
                *t == C
                    && p.x == 46
                    && p.y == 102
                    && *s == 25
                    && c[0] == 0.1
                    && c[1] == 0.2
                    && c[2] == 0.3
                    && c[3] == 0.4
            })
            .return_const(());

        let tile_renderer = TileRenderer::new(START, SIZE);

        tile_renderer.render_char(&mut mock, C, &POS, COLOR);
    }

    #[test]
    fn test_render_full() {
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

        let tile_renderer = TileRenderer::new(START, SIZE);

        tile_renderer.render_full(&mut mock, &POS, COLOR);
    }
}
