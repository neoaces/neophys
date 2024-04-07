use super::{k_G, k_mE, k_rE, Force};
pub struct Gravity {
    // Gravitational constant without the objects mass
    k: f32
}

impl Gravity {
    /// Create a gravity force with the mass of the surface and the distance from the center.
    pub fn new(m: f32, d: f32) -> Self {
        Self {
            k: calculate_grav(m, d)
        }
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self::new(k_mE, k_rE)
    }
}

impl Force for Gravity {
    fn calc(&self) -> f32 {
        self.k
    }
}

pub fn calculate_grav(m: f32, d: f32) -> f32 {
    (k_G * m) / f32::powi(d, 2)
}