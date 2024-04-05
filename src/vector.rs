pub mod vec2;

pub trait Vector {
    fn new(a: f32, b: f32) -> Self;
    fn scale(&mut self, f: f32);
}
