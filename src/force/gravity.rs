use log::info;

use super::{Force, K_G, K_ME, K_RE};

#[derive(Clone)]
pub struct Gravity {
    // Gravitational constant without the objects mass
    k: f32,
}

impl Gravity {
    /// Create a gravity force with the mass of the surface and the distance from the center.
    pub fn new(m: f32, d: f32) -> Self {
        Self {
            k: calculate_grav(m, d),
        }
    }

    pub fn new_k(k: f32) -> Self {
        Self { k }
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self::new(10.0, K_RE)
    }
}

impl Force for Gravity {
    fn calc(&self) -> f32 {
        self.k
    }

    fn clone_dyn(&self) -> Box<dyn Force> {
        Box::new(self.clone())
    }
}

pub fn calculate_grav(m: f32, d: f32) -> f32 {
    (K_G * K_ME) / f32::powi(d, 2)
}
