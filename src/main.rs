use macroquad::prelude::{get_frame_time, next_frame};
use glam::Vec2;

use ya_nbody::physics::{barnes_hut::BarnesHut, galaxy::Galaxy};
use ya_nbody::physics::integrators::{Integrator, Euler, Verlet};

use ya_nbody::rendering::draw;
use ya_nbody::rendering::screen_state::ScreenState;
use ya_nbody::rendering::ui::*;

#[macroquad::main("Yet Another N-Body Simulation")]
async fn main() -> Result<(), String> {
    let integrator = Euler::default();

    let galaxy1 = Galaxy::new(30_000, Vec2::ONE * 100.0, 7e3, Vec2::new(0.0, 3.0), 15.0..100.0);
    let galaxy2 = Galaxy::new(30_000, Vec2::ONE * -100.0, 7e3, Vec2::new(0.0, -3.0), 15.0..100.0);
    
    let mut bodies = galaxy1.get_bodies();
    bodies.append(&mut galaxy2.get_bodies());

    let mut bh = BarnesHut::new(1.0, Vec2::ZERO, 2000.0, 20);
    let mut draw_bh = false;

    let mut screen_state = ScreenState::new(1.0);

    let mut paused = false;
    let dt = 0.05;

    loop {
        screen_state.handle_panning(-200.0);
        screen_state.handle_zoom(0.1);

        if !paused {
            bh.apply_gravity(&mut bodies);
            integrator.integrate_bodies(&mut bodies, dt);
        }

        if draw_bh {
            draw::draw_bh(&bh.tree_root, &screen_state);
        }

        draw::draw_bodies(&mut bodies, &screen_state);
 
        draw_ui_window(|ui| {
            draw_sim_section(ui, get_frame_time(), &mut paused);
            ui.separator();
            draw_bodies_section(ui, &mut bodies);
            ui.separator();
            draw_bh_visualize_toggle(ui, &mut draw_bh);
        });

        egui_macroquad::draw();
        next_frame().await
    }
}