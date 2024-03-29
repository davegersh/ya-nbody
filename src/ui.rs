use egui_macroquad::egui::{Color32, Ui, Window};

use crate::physics::body::Body;

pub fn draw_ui_window<F>(mut f: F) where F: FnMut(&mut Ui) {
    egui_macroquad::ui(|egui_ctx| {
        Window::new("Info").show(egui_ctx, |ui: &mut Ui| {
            f(ui);
        });
    });
}

pub fn draw_bodies_section(ui: &mut Ui, bodies: &mut Vec<Body>) {
    ui.colored_label(Color32::WHITE, format!("Bodies: {}", bodies.len().to_string()));

    if ui.button("Clear Bodies").clicked() {
        bodies.clear();
    }
}

pub fn draw_sim_section(ui: &mut Ui, dt: f32, paused: &mut bool) {
    ui.colored_label(Color32::WHITE, format!("FPS: {:.1}", 1.0 / dt));
    ui.toggle_value(paused, "Paused");
}

pub fn draw_bh_visualize_toggle(ui: &mut Ui, draw_bh: &mut bool) {
    ui.toggle_value(draw_bh, "Visualize Quadtree");
}