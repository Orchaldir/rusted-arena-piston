extern crate piston_window;

use piston_window::*;

struct Renderer<'a, 'b> {
    context: Context,
    graphics: &'a mut G2d<'b>
}

impl<'a,'b> Renderer<'a,'b> {
    fn start(&mut self) {
        clear([0.0, 0.0, 0.0, 1.0], self.graphics);
    }

    fn render_rectangle(&mut self, rect: [f64; 4], color: [f32; 4]) {
        rectangle(color, rect, self.context.transform, self.graphics);
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Rusted Arena", [640, 480])
            .exit_on_esc(true)
            .resizable(false)
            .build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            let mut renderer = Renderer { context: c, graphics: g };

            renderer.start();
            renderer.render_rectangle([0.0, 0.0, 100.0, 100.0], [1.0, 0.0, 0.0, 1.0]);
            renderer.render_rectangle([100.0, 0.0, 100.0, 100.0], [0.0, 0.0, 1.0, 1.0]);
        });
    }
}