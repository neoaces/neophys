use crate::body::Body;

pub struct Engine {
    bodies: Vec<Box<Body>>,
}

impl Engine {
    pub fn count_bodies(&self) -> usize {
        self.bodies.len()
    }
}
