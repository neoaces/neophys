pub mod bounding;
use log::{error, info};
use nannou::geom::Rect;

// type TempState = Vec<(usize, State)>;

pub enum Collision {
    Intersecting,
    Disjoint,
}

pub fn detect_ground_collisions(state: Rect, window: Rect) -> Result<(), Collision> {
    info!("State: {:?}\nWindow: {:?}", state, window);
    if state.left() < window.left()
        || state.right() > window.right()
        || state.bottom() < window.bottom()
    {
        error!("Intersecting!");
        return Err(Collision::Intersecting);
    }

    Ok(())
}
