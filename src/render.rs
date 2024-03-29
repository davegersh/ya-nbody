use crate::physics::body::Body;
use super::algorithms::barnes_hut::BHTreeNode;

use glam::Vec2;
use std::f32::consts::PI;
use macroquad::input::{self, is_mouse_button_down, mouse_delta_position, mouse_wheel};
use macroquad::prelude::{draw_circle, draw_rectangle_lines, draw_line, draw_triangle, Color};

#[derive(Default)]
pub struct ScreenState {
    pub screen_center: Vec2,
    pub zoom: f32
}


impl ScreenState {
    pub fn new(screen_center: Vec2, zoom: f32) -> Self {
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
        self.zoom += mouse_wheel().1 * speed;
        self.zoom = self.zoom.clamp(0.1, 100.0);
    }
}


pub fn draw_bodies(bodies: &Vec<Body>, screen_state: &ScreenState) {
    for (_, body) in bodies.iter().enumerate() {        
        let color = Color::new(1., 1.,1., 0.1);
        let radius = body.mass.clamp(0.5, 10.0);

        let position = (body.position * screen_state.zoom) + screen_state.screen_center;

        draw_circle(position.x, position.y, radius * screen_state.zoom, color);
    }
}

pub fn draw_bh(node: &BHTreeNode, screen_state: &ScreenState) {
    let width = node.region_width * screen_state.zoom;

    if width < 0.75 { return; }

    let mut pos = (node.region_center * screen_state.zoom) - Vec2::new(width / 2.0, width / 2.0);
    pos += screen_state.screen_center;

    let color = Color::new(1.0, 1.0, 1.0, 0.1);
    let thickness = (1.5 * screen_state.zoom).clamp(1.5, 10.0);
    draw_rectangle_lines(pos.x, pos.y, width, width, thickness, color);

    for child in node.children.iter() {
        draw_bh(child, screen_state);
    }
}

fn draw_vector(start: Vec2, end: Vec2, thickness: f32, color: Color) {
    todo!("should take in a screen state!");

    draw_line(start.x, start.y, end.x, end.y, thickness, color);

    let dir = (end - start).normalize();
    let t1 = end + Vec2::from_angle(PI / 2.0).rotate(dir).normalize() * thickness * 1.5;
    let t2 = end + Vec2::from_angle(-PI / 2.0).rotate(dir).normalize() * thickness * 1.5;
    let t3 = end + dir * thickness * 2.0;

    draw_triangle(
        macroquad::math::Vec2::new(t1.x, t1.y),
        macroquad::math::Vec2::new(t2.x, t2.y), 
        macroquad::math::Vec2::new(t3.x, t3.y), 
        color)
}

pub fn draw_body_vectors(body: &Body) {
    todo!("should take in a screen state!");
    
    let position = body.position;

    let vel_color = Color::new(0.25, 1.0, 0.25, 0.15);
    draw_vector(position, position + body.velocity, 2.0, vel_color);

    let acc_color = Color::new(1.0, 0.25, 0.25, 0.15);
    draw_vector(position, position + body.acceleration, 2.0, acc_color);
}