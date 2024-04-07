//! Implementation of forces to be used in a Body; generally not meant to be changed
//! - Gravity: represents the basic Newton gravitational force
//!
//! # Examples
//! ```
//! use rbt::force::gravity::Gravity;
//! // Instantiate a new force Gravity
//! let f = Gravity::new(0.0, 0.0);
//! ```

pub mod basic;
pub mod gravity;
use crate::constants::*;

use self::{basic::Basic, gravity::Gravity};

pub trait Force {
    /// Returns the resulting acceleration
    ///
    fn calc(&self) -> f32;
}
