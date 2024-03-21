use egui_macroquad::{draw, egui::{Color32, Ui, Window}};

use crate::body::Body;

pub fn draw_ui_window<F>(mut f: F) where F: FnMut(&mut Ui) {
    egui_macroquad::ui(|egui_ctx| {
        Window::new("Info").show(egui_ctx, |ui: &mut Ui| {
            f(ui);
        });
    });
}

pub fn draw_body_count(ui: &mut Ui, count: &usize) {
    ui.colored_label(Color32::WHITE, format!("Bodies: {}", count.to_string()));
}

pub fn draw_fps(ui: &mut Ui, dt: f32) {
    ui.colored_label(Color32::WHITE, format!("FPS: {:.1}", 1.0 / dt));
}

pub fn draw_clear_btn(ui: &mut Ui, bodies: &mut Vec<Body>) {
    if ui.button("Clear Bodies").clicked() {
        bodies.clear();
    }
}

pub fn draw_bh_visualize_toggle(ui: &mut Ui, draw_bh: &mut bool) {
    ui.toggle_value(draw_bh, "Visualize Quadtree");
}