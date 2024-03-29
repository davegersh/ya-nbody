use super::gravity::G;
use std::{f32::consts::PI, ops::Range};

use glam::Vec2;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::body::Body;

pub struct Galaxy {
    num_bodies: u32,
    center: Vec2,
    center_mass: f32,
    spawn_range: Range<f32>,
}

impl Galaxy {
    pub fn new(num_bodies: u32, center: Vec2, center_mass: f32, spawn_range: Range<f32>) -> Self {
        Self { num_bodies, center, center_mass, spawn_range }
    }

    pub fn get_bodies(&self) -> Vec<Body> {
        let mut bodies = vec![];

        bodies.push(Body::new(self.center_mass, Vec2::ZERO, self.center));
    
        for i in 0..self.num_bodies {
            let mut rng = ChaCha8Rng::seed_from_u64(i as u64);
            
            let distance = rng.gen_range(self.spawn_range.clone()) as f32;
            let angle = rng.gen_range(0.0..360.0) as f32 * PI / 180.0;
    
            let position = Vec2::new(angle.cos(), angle.sin()) * distance;
            let mass = 1.0;
            
            let vel_dir = Vec2::new(-position.y, position.x).normalize();
    
            let vel_mag = (G * (self.center_mass + mass) / distance).sqrt();
    
            let body = Body::new(mass, vel_dir * vel_mag, position + self.center);
    
            bodies.push(body);
        }
    
        bodies
    }
}