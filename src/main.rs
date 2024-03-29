use macroquad::prelude::{get_frame_time, next_frame, screen_height, screen_width};
use glam::Vec2;

use ya_nbody::physics::galaxy::Galaxy;
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
    let galaxy = Galaxy::new(50_000, screen_center() + Vec2::ONE, 9e3, 15.0..100.0);
    let mut bodies = galaxy.get_bodies();

    let mut bh = BarnesHut::new(1.0, screen_center(), screen_width() * 2.0, 20);
    let mut draw_bh = false;

    let mut steps = 1;
    let mut paused = false;

    let dt = 0.05;

    loop {
        if !paused {
            bh.tree_root.reset();
            bh.apply_gravity(&mut bodies);
            
            integrator.integrate_bodies(&mut bodies, dt);
        }

        if draw_bh {
            render::draw_bh(&bh.tree_root);
        }

        render::draw_bodies(&mut bodies);
 
        draw_ui_window(|ui| {
            draw_sim_section(ui, get_frame_time(), &mut paused, &mut steps);

            ui.separator();

            draw_bodies_section(ui, &mut bodies);
            if ui.button("Reset Galaxy").clicked() {
                bodies = galaxy.get_bodies();
            }

            ui.separator();

            draw_bh_visualize_toggle(ui, &mut draw_bh);
        });

        egui_macroquad::draw();
        next_frame().await
    }
}