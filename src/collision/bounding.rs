use log::debug;
use nannou::geom::Rect;

pub struct BoundingBox {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

pub fn construct_rect(x: f32, y: f32, width: f32) -> Rect {
    let x = Rect::from_x_y_w_h(x, y, width, width);
    debug!("Rect created: {:?}", x);
    x
}

impl BoundingBox {
    // Used for BodyType::Box()

    // pub fn top(&self) -> f32 {
    //     self.s.y + self.h
    // }
    // pub fn bottom(&self) -> f32 {
    //     self.s.y - self.h
    // }
    // pub fn left(&self) -> f32 {
    //     self.s.x - self.h
    // }
    // pub fn right(&self) -> f32 {
    //     self.s.y + self.h
    // }
}
