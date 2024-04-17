use crate::body::{Body, State};
use std::cell::RefCell;
use std::fmt::Debug;

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

    pub fn bodies(&self) -> &Vec<RefCell<Body>> {
        &self.bodies
    }

    pub fn body(&self, i: usize) -> Option<&RefCell<Body>> {
        self.bodies().get(i)
    }

    pub fn calc(&self, del: f32, timestep: f32) -> Result<(), NoBodyError> {
        // TODO: Implement the collision loops inside this function, not inside the body.
        if let None = self.count_bodies() {
            return Err(NoBodyError);
        }
        // Calculate state
        let states: Vec<State> = Vec::new();
        // if let Some(_a) = self.count_bodies() {
        //     for body in self.bodies.iter() {
        //         body.borrow_mut().calc(del, timestep);
        //     }

        //     Ok(())
        // } else {
        //     Err(NoBodyError)
        // }
        Ok(())
    }

    pub fn update_mass(&self, m: f32, i: usize) -> Result<(), NoBodyError> {
        if !self.bodies.is_empty() {
            let mut a = self.body(i).unwrap().borrow_mut();
            a.m = m;
            Ok(())
        } else {
            Err(NoBodyError)
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine::new(Vec::<RefCell<Body>>::new())
    }
}

impl Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("Iterations", &self.iterations)
            .field("Bodies", &self.bodies)
            .finish()
    }
}
