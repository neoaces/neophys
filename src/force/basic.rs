use nannou::glam::Vec2;

use super::Force;

#[derive(Clone)]
pub struct Basic {
    m: f32,
}

impl Force for Basic {
    fn calc(&self, _x: Vec2, _dxdt: Vec2) -> f32 {
        self.m
    }

    fn clone_dyn(&self) -> Box<dyn Force> {
        Box::new(self.clone())
    }
}
