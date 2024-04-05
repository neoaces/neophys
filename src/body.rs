use crate::{force::Force, vector::vec2::Vec2};

pub struct Body {
    /// Mass of the body in kilograms
    m: f32,
    /// Represents the position vector in 2D cartesian space
    s: Vec2,
    /// Represents the position vector in 2D cartesian space
    v: Vec2,
    ///
    a: Vec<Force>,
}
