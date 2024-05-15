use egui_macroquad::egui::{self, Color32, Ui, Window};
use egui_macroquad::egui::plot::{Line, Plot, PlotPoints};

use macroquad::prelude::get_frame_time;

use crate::physics::Body;

pub fn draw_ui<F>(mut f: F) where F: FnMut(&mut Ui) {
    egui_macroquad::ui(|egui_ctx| {
        Window::new("Info")
        .fixed_size(egui::vec2(200.0, 0.0))
        .show(egui_ctx, |ui: &mut Ui| {
            f(ui);
        });
    });
}

pub fn draw_energy_plot(ui: &mut Ui, ke_points: &Vec<f64>, dt: &f64) {
    ui.colored_label(Color32::WHITE, "Kinetic Energy (J) Plot: ");

    let energy: PlotPoints = (0..ke_points.len()).map(|i| {
        [i as f64 * dt, ke_points[i]]
    }).collect();
    let energy_line = Line::new(energy);
    Plot::new("Kinetic Energy Plot").view_aspect(4.0).show(ui, |plot_ui| plot_ui.line(energy_line));
}

pub fn draw_sim_section(ui: &mut Ui, bodies: &Vec<Body>, dt: &f64, time_passed: &f64, paused: &mut bool) {
    ui.colored_label(Color32::WHITE, "Simulation:");

    ui.label(format!("Bodies: {:?}", bodies.len()));
    ui.label( format!("Time per frame: {:.3} s", dt));

    ui.horizontal(|ui| {
        ui.label(format!("Simulated time: {:.2} s", time_passed));
        ui.toggle_value(paused, "Pause");
    });
}

pub fn draw_debug_ui(ui: &mut Ui, iterations: &u32, draw_bh: &mut bool) {
    ui.colored_label(Color32::WHITE, "Debug:");

    ui.label(format!("FPS: {:.1}", 1.0 / get_frame_time()));
    ui.label(format!("Iterations per frame: {:?}", iterations));
    ui.checkbox(draw_bh, "Visualize Barnes-Hut Tree");
}