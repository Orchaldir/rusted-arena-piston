
pub trait Renderer {
    fn start(&mut self);
    fn render_rectangle(&mut self, rect: [f64; 4], color: [f32; 4]);
}