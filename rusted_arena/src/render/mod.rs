pub mod tile;

use crate::math::Point;

pub trait Renderer {
    fn start(&mut self);
    fn render_rectangle(&mut self, pos: &Point, size: &Point, color: [f32; 4]);
}
