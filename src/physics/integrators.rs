use super::Body;

pub trait Integrator {
    fn integrate(&self, body: &mut Body, dt: f32);

    fn integrate_bodies(&self, bodies: &mut Vec<Body>, dt: f64) where Self: Sync {
        for body in bodies {
            self.integrate(body, dt as f32);
            body.reset_force();
        }
    }
}

#[derive(Default)]
pub struct Verlet {}

impl Integrator for Verlet {
    fn integrate(&self, body: &mut Body, dt: f32) {
        let new_acc = body.force / body.mass;

        body.velocity += (new_acc + body.acceleration) / 2.0 * dt;
        body.position += body.velocity * dt + body.acceleration * dt * dt * 0.5;
    
        body.acceleration = new_acc;
    }
}

#[derive(Default)]
pub struct Euler {}

impl Integrator for Euler {
    fn integrate(&self, body: &mut Body, dt: f32) {
        body.acceleration = body.force / body.mass;
        
        body.velocity += body.acceleration * dt;
        body.position += body.velocity * dt;
    }
}
