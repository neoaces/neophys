use nannou::glam::Vec2;

use super::{Force, K_G, K_ME, K_RE};

#[derive(Clone)]
pub struct Gravity {
    // Gravitational constant without the objects mass
    k: f32,
}

impl Gravity {
    /// Create a gravity force with the mass of the surface and the distance from the center.
    pub fn new(d: f32, m: f32) -> Self {
        Self {
            k: calculate_grav(d, m),
        }
    }

    pub fn new_k(k: f32) -> Self {
        Self { k }
    }
}

impl Force for Gravity {
    fn calc(&self, s: Vec2, u: Vec2) -> Vec2 {
        Vec2::new(self.calc_x(s.x, u.x), self.calc_y(s.y, u.y))
    }

    fn calc_x(&self, _: f32, _: f32) -> f32 {
        0.0
    }

    fn calc_y(&self, _: f32, _: f32) -> f32 {
        self.k
    }

    fn clone_dyn(&self) -> Box<dyn Force> {
        Box::new(self.clone())
    }
}

pub fn calculate_grav(d: f32, m: f32) -> f32 {
    (-K_G * K_ME * m) / f32::powi(d, 2)
}
