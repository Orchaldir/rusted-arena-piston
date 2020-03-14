extern crate piston_window;

mod render;

use piston_window::*;
use render::tile::TileRenderer;
use render::Renderer;

struct PistonRenderer<'a, 'b> {
    context: Context,
    graphics: &'a mut G2d<'b>,
}

impl<'a, 'b> Renderer for PistonRenderer<'a, 'b> {
    fn start(&mut self) {
        clear([0.0, 0.0, 0.0, 1.0], self.graphics);
    }

    fn render_rectangle(&mut self, pos: (u32, u32), size: (u32, u32), color: [f32; 4]) {
        let rect = [pos.0 as f64, pos.1 as f64, size.0 as f64, size.1 as f64];
        rectangle(color, rect, self.context.transform, self.graphics);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rusted Arena", [640, 480])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let tile_renderer = TileRenderer::new((0, 0), (100, 100));

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
    tile_renderer.render_full(renderer, (0, 0), [1.0, 0.0, 0.0, 1.0]);
    tile_renderer.render_full(renderer, (1, 0), [0.0, 0.0, 1.0, 1.0]);
}
