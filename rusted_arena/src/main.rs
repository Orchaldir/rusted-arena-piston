extern crate find_folder;
#[macro_use]
extern crate impl_ops;
extern crate piston_window;

mod math;
mod render;

use crate::piston_window::character::CharacterCache;
use math::Point;
use piston_window::*;
use render::tile::TileRenderer;
use render::Renderer;

struct PistonRenderer<'a, 'b> {
    context: Context,
    graphics: &'a mut G2d<'b>,
    glyphs: &'a mut Glyphs,
}

impl<'a, 'b> Renderer for PistonRenderer<'a, 'b> {
    fn start(&mut self) {
        let black = [0.0, 0.0, 0.0, 1.0];
        clear(black, self.graphics);
    }

    fn render_char(&mut self, c: char, pos: &Point, size: u32, color: [f32; 4]) {
        let image = Image::new_color(color);
        let character = self.glyphs.character(size, c).unwrap();
        let center = pos + (size / 2) as i32;
        let start_x = center.x as f64 - character.advance_size[0] * 0.5;
        let start_y = center.y as f64 - character.offset[1] * 0.5;

        image.draw(
            character.texture,
            &self.context.draw_state,
            self.context.transform.trans(start_x, start_y),
            self.graphics,
        );
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

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .expect("Could not find assets folder!");

    let mut glyphs = window
        .load_font(assets.join("Cascadia.ttf"))
        .expect("Could not load font!");

    let start = Point { x: 0, y: 0 };
    let tile_size = Point { x: 100, y: 100 };

    let tile_renderer = TileRenderer::new(start, tile_size);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            let mut renderer = PistonRenderer {
                context: c,
                graphics: g,
                glyphs: &mut glyphs,
            };

            render(&mut renderer, &tile_renderer);
        });
    }
}

fn render<R: Renderer>(renderer: &mut R, tile_renderer: &TileRenderer) {
    renderer.start();
    tile_renderer.render_full(renderer, &Point { x: 0, y: 0 }, [1.0, 0.0, 0.0, 1.0]);
    tile_renderer.render_full(renderer, &Point { x: 1, y: 0 }, [0.0, 0.0, 1.0, 1.0]);

    renderer.render_char('#', &Point { x: 0, y: 0 }, 100, [1.0, 1.0, 1.0, 1.0]);
}
