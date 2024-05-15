use macroquad::prelude::{get_frame_time, next_frame};
use glam::Vec2;

use ya_nbody::physics::{
    BarnesHut, Naive, Galaxy, GravitySolver,
    integrators::{Integrator, Euler, Verlet}
};

use ya_nbody::rendering::draw;
use ya_nbody::rendering::ScreenState;
use ya_nbody::rendering::ui::*;

#[macroquad::main("Yet Another N-Body Simulation")]
async fn main() -> Result<(), String> {
    let integrator = Verlet::default();

    let galaxy1 = Galaxy::new(15_000, Vec2::ONE * 200.0, 5e5, Vec2::new(0.0, -0.0), 150.0);
    let galaxy2 = Galaxy::new(5_000, Vec2::ONE * -200.0,5e4, Vec2::new(20.0, 0.0), 50.0);
    
    let mut bodies = galaxy1.get_bodies();
    bodies.append(&mut galaxy2.get_bodies());

    let mut bh: BarnesHut = BarnesHut::new(1.0, Vec2::ZERO, 20000.0, 20);
    let mut draw_bh = false;

    let mut screen_state = ScreenState::new(1.0);

    let mut paused = false;
    let mut dt = 0.025;
    let mut time_passed = 0.0;

    let mut ke_points: Vec<f64> = vec![];

    loop {
        screen_state.handle_panning(-200.0);
        screen_state.handle_zoom(0.1);

        if !paused {
            for _ in 0..4 {
                bh.apply_gravity(&mut bodies);
                integrator.integrate_bodies(&mut bodies, dt / 4.0);
            }
            
            let mut total_energy = 0.0;
            for body in bodies.iter() {
                total_energy += body.get_kinetic_energy();
            }
            ke_points.push(total_energy as f64);

            time_passed += dt;
        }

        if draw_bh {
            draw::draw_bh(&bh.tree_root, &screen_state);
        }

        draw::draw_bodies(&mut bodies, &screen_state);
 
        draw_ui_window(|ui| {
            draw_sim_section(ui, get_frame_time(), &mut dt, &time_passed, &mut paused);
            
            ui.separator();
            draw_energy_plot(ui, &ke_points, &dt);

            ui.separator();
            draw_bodies_section(ui, &mut bodies);

            ui.separator();
            draw_bh_visualize_toggle(ui, &mut draw_bh);
        });

        egui_macroquad::draw();
        next_frame().await
    }
}