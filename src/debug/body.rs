use nannou::{color::WHITE, geom::Rect, glam::Vec2, Draw};

#[derive(Debug, Clone)]
pub struct BoundingError;

pub fn draw_bounding_circle(d: &Draw, r: f32, c: Vec2) {
    d.rect()
        .w_h(r * 2.0, r * 2.0)
        .stroke(WHITE)
        .stroke_weight(1.5)
        .no_fill()
        .finish()
}
