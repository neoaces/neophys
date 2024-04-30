use crate::body::State;
use nannou::glam::Vec2;

type TempState = Vec<(usize, State)>;

pub enum CollisionError {
    Intersecting,
    Disjoint,
}

pub fn detect_collisions(v: TempState, ground: Option<f32>) -> Result<TempState, CollisionError> {
    if let Some(g) = ground {
        
    }

    Ok(v)
}
