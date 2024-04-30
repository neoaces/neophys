//! Implementation of forces to be used in a Body; generally not meant to be changed
//! - Gravity: represents the basic Newton gravitational force
//!
//! # Examples
//! ```
//! use neophys::force::gravity::Gravity;
//! // Instantiate a new force Gravity
//! let f = Gravity::new(0.0, 0.0);
//! ```

use self::{basic::Applied, gravity::Gravity};
use crate::constants::*;
use nannou::glam::Vec2;

pub mod basic;
pub mod gravity;

#[derive(Clone)]
pub enum ForceType {
    Gravity(Gravity),
    Basic(Applied),
}

pub trait Force {
    /// Returns the resulting acceleration
    fn calc(&self, x: Vec2, u: Vec2) -> Vec2;
    fn calc_x(&self, x: f32, u: f32) -> f32;
    fn calc_y(&self, y: f32, u: f32) -> f32;
    fn clone_dyn(&self) -> Box<dyn Force>;
}

impl Clone for Box<dyn Force> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
