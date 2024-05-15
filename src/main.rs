use std::env;

use glam::Vec2;

use ya_nbody::{
    config::Config, 
    physics::{integrators::{Integrator, Verlet}, BarnesHut,},
    rendering::{draw, ScreenState, ui::*}
};

#[macroquad::main("Yet Another N-Body Simulation")]
async fn main() -> Result<(), String> {
    let thread_count = std::thread::available_parallelism().expect("Couldn't get a thread count!");
    let mut bh: BarnesHut = BarnesHut::new(1.0, Vec2::ZERO, 20000.0, thread_count.get());
    let mut draw_bh = false;

    let integrator = Verlet::default();

    let mut screen_state = ScreenState::new(1.0);

    let mut paused = false;
    let mut time_passed = 0.0;

    let mut ke_points: Vec<f64> = vec![];

    let args: Vec<String> = env::args().collect();
    let file_path = &args.get(1).expect("No config file found!");

    let mut config = Config::from_toml(file_path);
    let mut bodies = config.bodies();

    loop {
        if !paused {
            for _ in 0..config.iterations {
                bh.apply_gravity(&mut bodies);
                integrator.integrate_bodies(&mut bodies, config.dt / (config.iterations as f64));
            }
            
            let total_energy: f32 = bodies.iter().map(|b| b.kinetic_energy()).sum();
            ke_points.push(total_energy as f64);

            time_passed += config.dt;
        }

        if draw_bh {
            draw::draw_bh(&bh.tree_root, &screen_state);
        }

        draw::draw_bodies(&mut bodies, &screen_state);
 
        draw_ui(|ui| {
            draw_sim_section(ui, &bodies, &config.dt, &time_passed, &mut paused);

            if ui.button("Reload").clicked() {
                config = Config::from_toml(file_path);
                bodies = config.bodies();
                time_passed = 0.0;
                ke_points.clear();
            }

            ui.separator();
            draw_energy_plot(ui, &ke_points, &config.dt);
            ui.separator();
            draw_debug_ui(ui, &config.iterations, &mut draw_bh);
        });

        egui_macroquad::draw();

        screen_state.handle_panning(-300.0);
        screen_state.handle_zoom(0.04);

        macroquad::prelude::next_frame().await
    }
}