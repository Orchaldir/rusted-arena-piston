#[macro_use]
extern crate impl_ops;
extern crate piston_window;

mod math;
mod render;

use math::Point;
use piston_window::*;
use render::tile::TileRenderer;
use render::Renderer;

struct PistonRenderer<'a, 'b> {
    context: Context,
    graphics: &'a mut G2d<'b>,
}

impl<'a, 'b> Renderer for PistonRenderer<'a, 'b> {
    fn start(&mut self) {
        let black = [0.0, 0.0, 0.0, 1.0];
        clear(black, self.graphics);
    }

    fn render_rectangle(&mut self, pos: &Point, size: &Point, color: [f32; 4]) {
        let rect = [pos.x as f64, pos.y as f64, size.x as f64, size.y as f64];
        rectangle(color, rect, self.context.transform, self.graphics);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rusted Arena", [640, 480])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let start = Point { x: 0, y: 0 };
    let tile_size = Point { x: 100, y: 100 };

    let tile_renderer = TileRenderer::new(start, tile_size);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            let mut renderer = PistonRenderer {
                context: c,
                graphics: g,
            };

            render(&mut renderer, &tile_renderer);
        });
    }
}

fn render<R: Renderer>(renderer: &mut R, tile_renderer: &TileRenderer) {
    renderer.start();
    tile_renderer.render_full(renderer, &Point { x: 0, y: 0 }, [1.0, 0.0, 0.0, 1.0]);
    tile_renderer.render_full(renderer, &Point { x: 1, y: 0 }, [0.0, 0.0, 1.0, 1.0]);
}
