use crate::physics::body::Body;
use super::gravity::calculate_body_gravity;

#[derive(Default)]
pub struct Naive {}

impl Naive {
    fn apply_gravity(&self, bodies: &mut Vec<Body>) {
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
