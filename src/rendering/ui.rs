use egui_macroquad::egui::{self, Color32, Context, Ui, Window};
use egui_macroquad::egui::plot::{Line, Plot, PlotPoints};

use crate::physics::Body;

pub fn draw_ui_window<F>(mut f: F) where F: FnMut(&mut Ui) {
    egui_macroquad::ui(|egui_ctx| {
        Window::new("Info").show(egui_ctx, |ui: &mut Ui| {
            f(ui);
        });
    });
}

pub fn draw_bodies_section(ui: &mut Ui, bodies: &mut Vec<Body>) {
    ui.colored_label(Color32::WHITE, format!("Bodies: {}", bodies.len().to_string()));
}

pub fn draw_energy_plot(ui: &mut Ui, ke_points: &Vec<f64>, dt: &f64) {
    ui.colored_label(Color32::WHITE, "Kinetic Energy (J) Plot: ");

    let energy: PlotPoints = (0..ke_points.len()).map(|i| {
        [i as f64 * dt, ke_points[i]]
    }).collect();
    let energy_line = Line::new(energy);
    Plot::new("Kinetic Energy Plot").view_aspect(4.0).show(ui, |plot_ui| plot_ui.line(energy_line));
}

pub fn draw_sim_section(ui: &mut Ui, frame_time: f32, dt: &mut f64, time_passed: &f64, paused: &mut bool) {
    ui.colored_label(Color32::WHITE, format!("FPS: {:.1}", 1.0 / frame_time));
    ui.label( format!("dt: {:.3}", dt));
    ui.horizontal(|ui| {
        ui.label(format!("Simulation Time: {:.2}", time_passed));
        ui.toggle_value(paused, "Pause");
    });
}

pub fn draw_bh_visualize_toggle(ui: &mut Ui, draw_bh: &mut bool) {
    ui.checkbox(draw_bh, "Visualize Quadtree");
}