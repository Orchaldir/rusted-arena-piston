pub mod tile;

pub trait Renderer {
    fn start(&mut self);
    fn render_rectangle(&mut self, pos: (u32, u32), size: (u32, u32), color: [f32; 4]);
}
