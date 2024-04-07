use crate::body::Body;

pub struct Engine {
    bodies: Vec<Body>,
}

impl Engine {
    pub fn count_bodies(&self) -> usize {
        self.bodies.len()
    }
}
