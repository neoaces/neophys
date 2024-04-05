#[derive(Debug)]
pub struct Vec2 {
    i: f32,
    j: f32,
}

impl Vec2 {
    pub fn x(&self) -> f32 {
        self.i
    }

    pub fn y(&self) -> f32 {
        self.j
    }

    pub fn scale_x(&mut self, f: f32) {
        self.i *= f;
    }

    pub fn scale_y(&mut self, f: f32) {
        self.j *= f;
    }
}

impl super::Vector for Vec2 {
    fn new(a: f32, b: f32) -> Self {
        Self { i: a, j: b }
    }

    fn scale(&mut self, f: f32) {
        self.scale_x(f);
        self.scale_y(f);
    }
}
