use super::ScreenState;

use crate::physics::Body;
use crate::physics::barnes_hut::BHTreeNode;

use std::f32::consts::PI;

use glam::Vec2;
use macroquad::prelude::{draw_circle, draw_rectangle_lines, draw_line, draw_triangle, Color};

pub fn draw_bodies(bodies: &Vec<Body>, screen_state: &ScreenState) {
    for (_, body) in bodies.iter().enumerate() {        
        let color = Color::new(1., 1.,1., 0.2);
        let radius = (body.mass / 2.0).clamp(0.5, 10.0);

        let position = (body.position * screen_state.zoom) + screen_state.screen_center;
        
        if screen_state.screen_contains_position(position) {
            draw_circle(position.x, position.y, radius * screen_state.zoom, color);
        }
    }
}

pub fn draw_bh(node: &BHTreeNode, screen_state: &ScreenState) {
    let width = node.region_width * screen_state.zoom;

    if width < 0.75 { return; }

    let pos = screen_state.transform_position(node.region_center) - Vec2::ONE * width / 2.0;

    let color = Color::new(1.0, 1.0, 1.0, 0.1);
    let thickness = (1.5 * screen_state.zoom).clamp(2.0, 10.0);
    draw_rectangle_lines(pos.x, pos.y, width, width, thickness, color);

    for child in node.children.iter() {
        draw_bh(child, screen_state);
    }
}

pub fn draw_body_vectors(body: &Body, screen_state: &ScreenState) {    
    let position = screen_state.transform_position(body.position);

    let vel_color = Color::new(0.25, 1.0, 0.25, 0.15);
    draw_vector(position, position + body.velocity, 2.0, vel_color);

    let acc_color = Color::new(1.0, 0.25, 0.25, 0.15);
    draw_vector(position, position + body.acceleration, 2.0, acc_color);
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

