extern crate piston_window;

mod render;

use piston_window::*;
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

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            let mut renderer = PistonRenderer {
                context: c,
                graphics: g,
            };

            render(&mut renderer);
        });
    }
}

fn render<R: Renderer>(renderer: &mut R) {
    let size = (100, 100);
    renderer.start();
    renderer.render_rectangle((0, 0), size, [1.0, 0.0, 0.0, 1.0]);
    renderer.render_rectangle((100, 0), size, [0.0, 0.0, 1.0, 1.0]);
}
