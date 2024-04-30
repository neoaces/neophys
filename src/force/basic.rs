use super::Force;
use nannou::glam::Vec2;

#[derive(Clone)]
pub struct Applied {
    pub mag_x: f32,
    pub mag_y: f32,
    m: f32,
}

impl Force for Applied {
    fn calc(&self, _x: Vec2, _dxdt: Vec2) -> Vec2 {
        Vec2::new(self.mag_x, self.mag_y)
    }

    fn calc_x(&self, x: f32, u: f32) -> f32 {
        self.mag_x
    }

    fn calc_y(&self, y: f32, u: f32) -> f32 {
        self.mag_y
    }

    fn clone_dyn(&self) -> Box<dyn Force> {
        Box::new(self.clone())
    }
}
