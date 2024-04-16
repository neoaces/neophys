use crate::body::{Body, State};
use log::debug;

/// Implementation of the fourth order Runge Kutta method to update the position and velocity of a Body
/// * `body` - The body to be updated
/// * `init_state` - The initial state of the Body
/// * `t` - The initial time of the system
/// * `timestep` - The timestep wanted for the simulation
pub fn solve_rk4_and_set(body: &mut Body, init_state: &State, t: f32, timestep: f32) {
    // We start with the function x** = F(x, t) where F(t) represents the force
    debug!(
        "Inital conditions: {:?}, time: {}s, timestep: {}s",
        init_state, t, timestep
    );

    let x = init_state.s.x;
    let y = init_state.s.y;
    let dxdt = init_state.v.x;
    let dydt = init_state.v.y;
    let a = body.sum_forces();

    debug!("Acceleration: {}", body.sum_forces());

    body.v.y += solve_rk4(x, None, timestep, &|_: f32, _: f32| a);

    debug!("Calculated vy: {}", body.v.y);
    let s = body.v.y * timestep;
    body.s.y += s;
    debug!("Calculated y: {}, Sy: {}", s, body.s.y);
}

/// Implementation of the fourth order Runge Kutta method.
/// Returns the solution to the problem dx/dt = F(x, u) where u is held constant
/// * `x` - The initial state of the system
/// * `dxdt` - The initial ROF of the system
/// * `t` - The initial time of the system
/// * `f` - The function f(t, u) = dx/dt

pub fn solve_rk4(x: f32, u: Option<f32>, t: f32, f: &dyn Fn(f32, f32) -> f32) -> f32 {
    let init_x = x;
    // The constant u, used to control, is assumed to be constant across the time interval.
    let init_u = u.unwrap_or_default();

    let k1 = f(init_x, init_u);
    let k2 = f(init_x + t * 0.5 * k1, init_u);
    let k3 = f(init_x + t * 0.5 * k2, init_u);
    let k4 = f(init_x + t * k3, init_u);

    init_x + t * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}
