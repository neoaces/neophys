use log::info;

use super::{Force, K_G, K_ME, K_RE};

#[derive(Clone)]
pub struct Gravity {
    // Gravitational constant without the objects mass
    k: f32,
}

impl Gravity {
    /// Create a gravity force with the mass of the surface and the distance from the center.
    pub fn new(d: f32) -> Self {
        Self {
            k: calculate_grav(d),
        }
    }

    pub fn new_k(k: f32) -> Self {
        Self { k }
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self::new(K_RE)
    }
}

impl Force for Gravity {
    fn calc(&self, _x: f32, _dxdt: f32) -> f32 {
        self.k
    }

    fn clone_dyn(&self) -> Box<dyn Force> {
        Box::new(self.clone())
    }
}

pub fn calculate_grav(d: f32) -> f32 {
    (K_G * K_ME) / f32::powi(d, 2)
}
