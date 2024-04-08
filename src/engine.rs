use crate::body::Body;
use std::fmt::Debug;
use std::{rc::Rc, time::Duration};

#[derive(Debug, Clone)]
pub struct NoBodyError;

pub struct Engine {
    bodies: Vec<Box<Body>>,
}

impl Engine {
    pub fn count_bodies(&self) -> Option<usize> {
        if self.bodies.len() != 0 {
            Some(self.bodies.len())
        } else {
            None
        }
    }

    pub fn add_body(&mut self, body: Box<Body>) {
        self.bodies.push(body)
    }

    pub fn new(bodies: Vec<Box<Body>>) -> Self {
        Self { bodies }
    }

    pub fn peek_bodies(&self) -> &Vec<Box<Body>> {
        &self.bodies
    }

    pub fn bodies(&self) -> &Vec<Box<Body>> {
        &self.bodies
    }

    pub fn calc(&self, del: f32) {
        if let Some(a) = self.count_bodies() {
            for body in self.bodies.iter() {
                body.calc()
            }
        }
    }

    pub fn update_mass(&mut self, m: f32, i: usize) -> Result<(), NoBodyError> {
        if let Some(a) = self.bodies.get_mut(i) {
            a.m = m;
            Ok(())
        } else {
            Err(NoBodyError)
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(Vec::<Box<Body>>::new())
    }
}

impl Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("Bodies", &self.bodies)
            .finish()
    }
}
