pub mod bounding;
use crate::constants::K_RE;
use crate::force::{gravity::Gravity, Force};
use crate::rk::solve_rk4;
use log::info;
use nannou::geom::Vec2;
use std::fmt::Debug;

/// Struct representing the system state
/// * `s` - The position of the system as a vector
/// * `v` - The velocity of the system as a vector
#[derive(Debug, Clone, Copy)]
pub struct State {
    pub s: Vec2,
    pub v: Vec2,
}

#[derive(Clone, Debug)]
pub enum BodyType {
    Ball(f32),
    Box(f32),
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
                BodyType::Ball(_) | BodyType::Box(_) => vec![Box::new(Gravity::new(K_RE, m))],
                _ => vec![],
            },
            body_type,
            m,
            s: Vec2::default(),
            v: Vec2::default(),
        }
    }

    // Returns the acc
    pub fn a<'a>(
        &'a self,
        v: Vec2,
        u: Vec2,
    ) -> (impl Fn(f32, f32) -> f32 + 'a, impl Fn(f32, f32) -> f32 + 'a) {
        (self.a_x(), self.a_y())
    }

    /// Represents the F(t) where F is the forces acting on the given body.
    pub fn a_x<'a>(&'a self) -> impl Fn(f32, f32) -> f32 + 'a {
        move |v, u| {
            let mut sum: f32 = 0.0;

            if !self.forces.is_empty() {
                for force in self.forces.iter() {
                    let f = force.calc_x(v, u);
                    sum += f;
                }
            }

            info!("Summed force in x: {}N", sum);
            sum / self.m
        }
    }

    /// Represents the F(t) where F is the forces acting on the given body.
    pub fn a_y<'a>(&'a self) -> impl Fn(f32, f32) -> f32 + 'a {
        move |v, u| {
            let mut sum: f32 = 0.0;

            if !self.forces.is_empty() {
                for force in self.forces.iter() {
                    let f = force.calc_y(v, u);
                    sum += f;
                }
            }
            info!("Summed force in y: {}N", sum);

            sum / self.m
        }
    }

    pub fn update(&mut self, state: State) {
        self.v = state.v;
        self.s = state.s;
    }

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
