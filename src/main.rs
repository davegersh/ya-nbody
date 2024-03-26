
use macroquad::prelude::{get_frame_time, next_frame, screen_height, screen_width};
use glam::Vec2;

use ya_nbody::physics::body_spawn;
use ya_nbody::algorithms::barnes_hut::BarnesHut;
use ya_nbody::physics::integrators::{Integrator, Verlet};
use ya_nbody::render;
use ya_nbody::ui::*;

fn screen_center() -> Vec2 {
    Vec2::new(screen_width() as f32 / 2.0, screen_height() as f32 / 2.0)
}

#[macroquad::main("Yet Another N-Body Simulation")]
async fn main() -> Result<(), String> {
    let integrator = Verlet::default();
    let mut bodies = body_spawn::create_galaxy(3000, screen_center() + Vec2::ONE, 1e4, 15.0, 100.0);
    
    let mut bh = BarnesHut::new(1.0, screen_center(), screen_width() * 2.0, 20);
    let mut draw_bh = false;

    loop {
        let dt = get_frame_time();

        for _ in 0..1 {
            bh.tree_root.reset();
            bh.apply_gravity(&mut bodies);

            integrator.integrate_bodies(&mut bodies, dt);
        }

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