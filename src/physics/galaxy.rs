use super::gravity::G;
use super::Body;

use std::f32::consts::PI;

use glam::Vec2;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub struct Galaxy {
    num_bodies: u32,
    size: f32,
    center_mass: f32,
    position: Vec2,
    velocity: Vec2,
}

impl Galaxy {
    pub fn new(num_bodies: u32, size: f32, center_mass: f32, position: Vec2, velocity: Vec2) -> Self {
        Self { num_bodies, size, position, center_mass, velocity }
    }

    pub fn get_bodies(&self) -> Vec<Body> {
        let mut bodies = vec![];

        bodies.push(Body::new(self.center_mass, self.position, self.velocity));
    
        for i in 0..self.num_bodies - 1 {
            let mut rng = ChaCha8Rng::seed_from_u64(i as u64);
            
            let distance = rng.gen_range(15.0..self.size) as f32;
            let angle = rng.gen_range(0.0..360.0) as f32 * PI / 180.0;
    
            let position = Vec2::new(angle.cos(), angle.sin()) * distance;
            let mass = rng.gen_range(1.0..5.0);
            
            let vel_dir = Vec2::new(-position.y, position.x).normalize();
    
            let vel_mag = (G * (self.center_mass + mass) / distance).sqrt();
    
            let body = Body::new(mass, position + self.position, vel_dir * vel_mag);
    
            bodies.push(body);
        }
    
        bodies
    }
}