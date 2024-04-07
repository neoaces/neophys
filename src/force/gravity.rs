use super::{k_mE, k_rE, Force};
pub struct Gravity {
    m: f32,
    d: f32,
}

impl Gravity {
    /// Create a gravity force with the mass of the surface and the distance from the center.
    pub fn new(m: f32, d: f32) -> Self {
        Self { m, d }
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self::new(k_mE, k_rE)
    }
}

impl Force for Gravity {
    fn calc(&self) -> f32 {
        todo!() // Implement calculation for gravitational force
    }
}

