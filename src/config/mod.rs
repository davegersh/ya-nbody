use crate::physics::Galaxy;
use crate::physics::Body;

pub struct Config {
    pub dt: f32,
    pub iterations: u32,
    pub galaxies: Vec<Galaxy>,
    pub bodies: Vec<Body>
}

impl Config {

}