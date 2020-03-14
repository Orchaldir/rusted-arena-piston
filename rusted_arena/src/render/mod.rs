pub mod tile;

use crate::math::Point;
use mockall::*;

#[automock]
pub trait Renderer {
    fn start(&mut self);
    fn render_char(&mut self, c: char, pos: &Point, size: u32, color: [f32; 4]);
    fn render_rectangle(&mut self, pos: &Point, size: &Point, color: [f32; 4]);
}
