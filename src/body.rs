use crate::force::{gravity::Gravity, Force, ForceType};
use nannou::geom::Vec2;
use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug)]
pub enum BodyType {
    GPoint,
    Point,
}

#[derive(Clone)]
pub struct Body {
    /// Type of the body, one of Point, GPoint
    body_type: BodyType,
    /// Mass of the body in kilograms
    pub m: f32,
    /// Represents the position vector in 2D cartesian space
    s: Vec2,
    /// Represents the velocity vector in 2D cartesian space
    v: f32,
    v_dir: Vec2,
    /// Array of all forces on the object
    forces: Vec<Rc<RefCell<dyn Force>>>,
}

impl Body {
    /// Constructor for Body
    /// * `m` - The mass of the body in kilograms
    pub fn new(body_type: BodyType, m: f32) -> Self {
        Self {
            forces: match &body_type {
                BodyType::GPoint => vec![Rc::new(RefCell::new(Gravity::default()))],
                _ => vec![],
            },

            body_type,
            m,
            s: Vec2::default(),
            v: 0.0,
            v_dir: Vec2::default(),
        }
    }

    pub fn calc(&self) {
        // Δd = v average * a
    }

    pub fn mass(&self) -> f32 {
        self.m
    }

    pub fn sum_forces(&self) -> f32 {
        if !self.forces.is_empty() {
            let mut sum: f32 = 0.0;

            for force in self.forces.iter() {
                sum += force.borrow().calc();
            }

            sum
        } else {
            0.0
        }
    }
}

impl Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Body")
            .field("BodyType", &self.body_type)
            .field("Mass (kg)", &self.m)
            .finish()
    }
}
