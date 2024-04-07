use crate::{
    force::{
        gravity::{self, Gravity},
        Force,
    },
    vector::vec2::Vec2,
};

pub enum BodyType {
    GPoint,
    Point,
}

pub struct Body {
    /// Type of the body, one of Point, GPoint
    body_type: BodyType,
    /// Mass of the body in kilograms
    m: f32,
    /// Represents the position vector in 2D cartesian space
    s: Vec2,
    /// Represents the velocity vector in 2D cartesian space
    v: f32,
    v_dir: Vec2,
    /// Array of all forces on the object
    forces: Vec<Box<dyn Force>>,
}

impl Body {
    pub fn new(body_type: BodyType, m: f32) -> Self {
        Self {
            forces: match &body_type {
                BodyType::GPoint => vec![Box::<Gravity>::default()],
                _ => vec![],
            },

            body_type,
            m,
            s: Vec2::default(),
            v: 0.0,
            v_dir: Vec2::default(),
        }
    }
}
