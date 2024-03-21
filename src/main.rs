use std::f32::consts::PI;

use macroquad::prelude::{get_frame_time, next_frame, screen_height, screen_width};
use glam::Vec2;
use rand::Rng;

use ya_nbody::barnes_hut::BarnesHut;
use ya_nbody::body::Body;
use ya_nbody::integrators::{Integrator, Verlet};
use ya_nbody::gravity::G;
use ya_nbody::render;
use ya_nbody::ui::*;

fn screen_center() -> Vec2 {
    Vec2::new(screen_width() as f32 / 2.0, screen_height() as f32 / 2.0)
}

fn create_galaxy(count: i32, center: Vec2, center_mass: f32, min_distance: f32, max_distance: f32) -> Vec<Body> {
    let mut bodies = vec![];

    bodies.push(Body::new(center_mass, Vec2::ZERO, center));

    for _ in 0..count {
        let mut rng = rand::thread_rng();

        let distance = rng.gen_range(min_distance..max_distance) as f32;
        let angle = rng.gen_range(0.0..360.0) as f32 * PI / 180.0;

        let position = Vec2::new(angle.cos(), angle.sin()) * distance;

        let vel_dir = Vec2::new(-position.y, position.x).normalize();
        let vel_mag = (G * center_mass / distance).sqrt();

        let body = Body::new(0.75, vel_dir * vel_mag, position + center);

        bodies.push(body);
    }

    bodies
}

#[macroquad::main("Yet Another N-Body Simulation")]
async fn main() -> Result<(), String> {
    let integrator = Verlet::default();
    let mut bodies = create_galaxy(10000, screen_center(), 1e4, 15.0, 150.0);
    
    let mut draw_bh = false;

    loop {
        let dt = get_frame_time();
        
        let mut bh = BarnesHut::new(1.0, screen_center(), screen_width() * 2.0, 20);
        bh.apply_gravity(&mut bodies);

        integrator.integrate_bodies(&mut bodies, dt);

        if draw_bh {
            render::draw_bh(&bh.tree_root);
        }
        
        render::draw_bodies(&mut bodies);
 
        draw_ui_window(|ui| {
            draw_body_count(ui, &bodies.len());
            draw_fps(ui, dt);
            draw_clear_btn(ui, &mut bodies);
            draw_bh_visualize_toggle(ui, &mut draw_bh);
        });

        egui_macroquad::draw();
        next_frame().await
    }
}