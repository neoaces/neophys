use crate::body::Body;
use core::time;
use std::cell::RefCell;
use std::fmt::Debug;
use std::{rc::Rc, time::Duration};

#[derive(Debug, Clone)]
pub struct NoBodyError;

pub struct Engine {
    bodies: Vec<RefCell<Body>>,
    pub iterations: u32,
}

impl Engine {
    pub fn count_bodies(&self) -> Option<usize> {
        if !self.bodies.is_empty() {
            Some(self.bodies.len())
        } else {
            None
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(RefCell::new(body));
    }

    pub fn new(bodies: Vec<RefCell<Body>>) -> Self {
        Self {
            bodies,
            iterations: 0,
        }
    }

    pub fn peek_bodies(&self) -> &Vec<RefCell<Body>> {
        &self.bodies
    }

    pub fn bodies(&self) -> &Vec<RefCell<Body>> {
        &self.bodies
    }

    #[allow(clippy::borrowed_box)]
    pub fn body(&self, i: usize) -> Option<&RefCell<Body>> {
        self.bodies().get(i)
    }

    pub fn calc(&self, del: f32, timestep: f32) {
        if let Some(_a) = self.count_bodies() {
            for body in self.bodies.iter() {
                body.borrow_mut().calc(del, timestep);
            }
        }
    }

    pub fn update_mass(&mut self, m: f32, i: usize) -> Result<(), NoBodyError> {
        if let Some(a) = self.bodies.get_mut(i) {
            a.borrow_mut().m = m;
            Ok(())
        } else {
            Err(NoBodyError)
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(Vec::<RefCell<Body>>::new())
    }
}

impl Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("Bodies", &self.bodies)
            .finish()
    }
}
