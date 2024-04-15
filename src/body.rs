use crate::force::{gravity::Gravity, Force, ForceType};
use crate::rk::solve_rk4_and_set;
use log::info;
use nannou::geom::Vec2;
use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

/// Struct representing the system state
/// * `s` - The position of the system as a vector
/// * `v` - The velocity of the system as a vector
#[derive(Debug)]
pub struct State {
    pub s: Vec2,
    pub v: Vec2,
}

#[derive(Clone, Debug)]
pub enum BodyType {
    GPoint,
    Point,
}

#[derive(Clone)]
pub struct Body {
    /// Type of the body, one of Point, GPoint
    body_type: BodyType,
    /// Represents the position vector in 2D cartesian space
    pub s: Vec2,
    /// Represents the velocity vector in 2D cartesian space
    pub v: Vec2,
    /// Mass of the body in kilograms
    pub m: f32,
    /// Array of all forces on the object
    forces: Vec<Box<dyn Force>>,
}

impl Body {
    /// Constructor for Body
    /// * `m` - The mass of the body in kilograms
    pub fn new(body_type: BodyType, m: f32) -> Self {
        Self {
            forces: match &body_type {
                BodyType::GPoint => vec![Box::<Gravity>::default()],
                _ => vec![],
            },

            body_type,
            m,
            s: Vec2::default(),
            v: Vec2::default(),
        }
    }

    pub fn calc(&mut self, d: f32, timestep: f32) {
        let state = State {
            s: self.s,
            v: self.v,
        };

        self.update(state, d, timestep);
    }

    /// Represents the F(t) where F is the forces acting on the given body.
    // TODO: replace the options
    pub fn sum_forces(&self, x: Option<f32>, dxdt: Option<f32>) -> f32 {
        if !self.forces.is_empty() {
            let mut sum: f32 = 0.0;

            for force in self.forces.iter() {
                sum += force.calc(x, dxdt);
            }

            sum
        } else {
            0.0
        }
    }

    pub fn update(&mut self, state: State, d: f32, timestep: f32) {
        solve_rk4_and_set(self, &state, d, timestep);
    }
    pub fn accn(&self, t: f32, _u: Option<f32>) -> f32 {
        self.sum_forces(None, None) / self.m
    }

    // TODO: implement a

    /// Gives the velocity of the system in the x direction
    pub fn vx(body: &Body, x: f32, u: f32) -> f32 {
        body.v.x
    }

    /// Gives the velocity of the system in the y direction
    pub fn vy(body: &Body, x: f32, u: f32) -> f32 {
        body.v.y
    }
}

impl Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Body")
            .field("BodyType", &self.body_type)
            .field("Mass (kg)", &self.m)
            .field("Position", &self.s)
            .finish()
    }
}
