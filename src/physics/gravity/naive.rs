use crate::physics::Body;
use super::{calculate_body_gravity, GravitySolver};

#[derive(Default)]
pub struct Naive;

impl GravitySolver for Naive {
    fn apply_gravity(&mut self, bodies: &mut Vec<Body>) {
        let n = bodies.len();
        
        for i in 0..n {
            for j in (i+1)..n {
                let f = calculate_body_gravity(&bodies[i], &bodies[j]);
                bodies[i].apply_force(f);
                bodies[j].apply_force(-f);
            }
        }
    }
}
