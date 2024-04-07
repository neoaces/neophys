use crate::body::Body;

pub struct Engine {
    bodies: Vec<Body>,
}

impl Engine {
    pub fn count_bodies(&self) -> usize {
        self.bodies.len()
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body)
    }

    pub fn new(bodies: Vec<Body>) -> Self {
        Self { bodies }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(Vec::<Body>::new())
    }
}
