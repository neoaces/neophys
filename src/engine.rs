use log::{debug, info};
use nannou::glam::Vec2;

use crate::body::{Body, State};
use crate::rk::solve_rk4;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct NoBodyError;

pub struct Engine {
    pub bodies: Vec<Body>,
    pub iterations: u32,
}

impl Engine {
    pub fn new(bodies: Vec<Body>) -> Self {
        Self {
            bodies,
            iterations: 0,
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn body(&self, i: usize) -> Option<&Body> {
        self.bodies().get(i)
    }

    pub fn bodies(&self) -> &Vec<Body> {
        &self.bodies
    }

    pub fn count_bodies(&self) -> Option<usize> {
        if !self.bodies.is_empty() {
            Some(self.bodies.len())
        } else {
            None
        }
    }

    pub fn calc(&mut self, t: f32) -> Result<(), NoBodyError> {
        if self.count_bodies().is_none() {
            return Err(NoBodyError);
        }

        // Calculate state
        let mut states: Vec<(usize, State)> = Vec::new();

        loop {
            // Get velocity and acceleration for each object through RK4
            for (i, body) in self.bodies.iter().enumerate() {
                debug!("Current body: {}, {:?}", i, body);
                let temp_body = body.clone();
                let (a_x, a_y) = temp_body.a(temp_body.v, temp_body.s);

                let t_vx = solve_rk4(temp_body.v.x, temp_body.s.x, t, a_x);
                let t_vy = solve_rk4(temp_body.v.x, temp_body.s.x, t, a_y);
                info!("{}m/s, {}m/s", t_vx, t_vy);

                let v = Vec2::new(temp_body.v.x + t_vx, temp_body.v.y + t_vy);
                let s = Vec2::new(temp_body.s.x + v.x * t, temp_body.s.y + v.y * t);
                states.push((i, State { v, s }));
            }

            // No need to advance to object collision detection if there is only one body
            if self.count_bodies().unwrap() == 1 {
                debug!("Only one body present in engine. Not performing collision detection.");
                break;
            }
        }

        info!("{:#?}", states);

        // When valid state is reached, update all the bodies.
        for state in states.iter() {
            self.bodies[0].v = state.1.v;
            self.bodies[0].s = state.1.s;
        }

        Ok(())
    }

    pub fn update_mass(&mut self, m: f32, i: usize) -> Result<(), NoBodyError> {
        if !self.bodies.is_empty() {
            self.bodies[i].m = m;
            Ok(())
        } else {
            Err(NoBodyError)
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine::new(Vec::<Body>::new())
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
