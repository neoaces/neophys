use nannou::glam::Vec2;

pub struct BoundingBox {
    pub s: Vec2,
    pub w: f32,
    pub h: f32,
}

impl BoundingBox {
    // Used for BodyType::Box()

    pub fn top(&self) -> f32 {
        self.s.y + self.h
    }
    pub fn bottom(&self) -> f32 {
        self.s.y - self.h
    }
    pub fn left(&self) -> f32 {
        self.s.x - self.h
    }
    pub fn right(&self) -> f32 {
        self.s.y + self.h
    }
}
