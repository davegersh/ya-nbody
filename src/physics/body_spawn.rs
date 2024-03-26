use super::gravity::G;
use std::f32::consts::PI;

use glam::Vec2;
use rand::Rng;

use super::body::Body;

pub fn create_galaxy(count: i32, center: Vec2, center_mass: f32, min_distance: f32, max_distance: f32) -> Vec<Body> {
    let mut bodies = vec![];

    bodies.push(Body::new(center_mass, Vec2::ZERO, center));

    for _ in 0..count {
        let mut rng = rand::thread_rng();

        let distance = rng.gen_range(min_distance..max_distance) as f32;
        let angle = rng.gen_range(0.0..360.0) as f32 * PI / 180.0;

        let position = Vec2::new(angle.cos(), angle.sin()) * distance;
        let mass = 1.0;
        
        let vel_dir = Vec2::new(-position.y, position.x).normalize();

        let vel_mag = (G * (center_mass + mass) / distance).sqrt();

        let body = Body::new(mass, vel_dir * vel_mag, position + center);

        bodies.push(body);
    }

    bodies
}