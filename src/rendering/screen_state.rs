use glam::Vec2;

use macroquad::window::{screen_height, screen_width};
use macroquad::input::{self, is_mouse_button_down, mouse_delta_position, mouse_wheel};

#[derive(Default)]
pub struct ScreenState {
    pub screen_center: Vec2,
    pub zoom: f32
}

impl ScreenState {
    pub fn new(zoom: f32) -> Self {
        let screen_center = Vec2::new(screen_width(), screen_height()) / 2.0;

        Self { screen_center, zoom }
    }
    
    pub fn handle_panning(&mut self, speed: f32) {
        let mouse_delta = mouse_delta_position();
    
        if is_mouse_button_down(input::MouseButton::Right) {
            let delta = Vec2::new(mouse_delta.x, mouse_delta.y);
            self.screen_center += delta * speed;
        }
    }
    
    pub fn handle_zoom(&mut self, speed: f32) {
        let prev_zoom = self.zoom;
        self.zoom += mouse_wheel().1 * speed * self.zoom;
        self.zoom = self.zoom.clamp(0.01, 100.0);

        let diff = prev_zoom - self.zoom;
        if diff != 0.0 {
            self.screen_center += Vec2::ONE * self.zoom;
        }
    }

    pub fn screen_contains_position(&self, position: Vec2) -> bool {
        if position.x > screen_width() || position.x < 0.0 {
            return false;
        }

        if position.y > screen_height() || position.y < 0.0 {
            return false;
        }
    
        return true;
    }

    pub fn transform_position(&self, position: Vec2) -> Vec2 {
        position * self.zoom + self.screen_center
    }
}