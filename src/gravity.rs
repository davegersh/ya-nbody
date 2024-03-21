use glam::Vec2;
use super::body::Body;

pub const G: f32 = 1.0;
const MIN_DISTANCE: f32 = 1.0;

pub fn calculate_gravity(pos1: Vec2, mass1: f32, pos2: Vec2, mass2: f32) -> Vec2 {
    let r = pos2 - pos1;

    let r_len = r.length();

    if r_len < MIN_DISTANCE {
        return Vec2::ZERO;
    }

    G * mass1 * mass2 / r_len.powi(3) * r
}

pub fn calculate_body_gravity(b1: &Body, b2: &Body) -> Vec2 {
    calculate_gravity(b1.position, b1.mass, b2.position, b2.mass)
}