use std::fs;
use serde_derive::Deserialize;

use crate::physics::Galaxy;
use crate::physics::Body;

use glam::Vec2;

#[derive(Deserialize)]
pub struct Config {
    pub dt: f64,
    pub iterations: u32,
    galaxies: Option<Vec<SerializableGalaxy>>,
    bodies: Option<Vec<SerializableBody>>
}

impl Config {
    pub fn from_toml(file_path: &str) -> Config {
        let contents = fs::read_to_string(file_path).unwrap();
        toml::from_str(&contents).unwrap()
    }

    pub fn bodies(&self) -> Vec<Body> {
        let mut bodies = vec![];

        if let Some(ser_galaxies) = &self.galaxies {
            for g in ser_galaxies.iter() {
                let position = Vec2::new(g.position_x, g.position_y);
                let velocity = Vec2::new(g.velocity_x, g.velocity_y);
                let galaxy = Galaxy::new(g.num_bodies, g.size, g.center_mass, position, velocity);

                bodies.append(&mut galaxy.get_bodies());
            }
        }

        if let Some(ser_bodies) = &self.bodies {
            for b in ser_bodies.iter() {
                let position = Vec2::new(b.position_x, b.position_y);
                let velocity = Vec2::new(b.velocity_x, b.velocity_y);
                let body = Body::new(b.mass, position, velocity);

                bodies.push(body);
            }
        }

        bodies
    }
}

#[derive(Deserialize)]
pub struct SerializableGalaxy {
    num_bodies: u32,
    size: f32,
    center_mass: f32,
    position_x: f32,
    position_y: f32,
    velocity_x: f32,
    velocity_y: f32,
}

#[derive(Deserialize)]
pub struct SerializableBody {
    mass: f32,
    position_x: f32,
    position_y: f32,
    velocity_x: f32,
    velocity_y: f32,
}