use super::body::Body;
use crate::barnes_hut::BHTreeNode;

use glam::Vec2;
use std::f32::consts::PI;
use macroquad::prelude::{draw_circle, draw_rectangle_lines, draw_line, draw_triangle, Color};


pub fn draw_bodies(bodies: &Vec<Body>) {
    for body in bodies.iter() {
        let speed = body.velocity.length() / 5.0 + 0.5;
        let color = Color::new(speed, speed, speed, 0.3);
        let mass = body.mass.clamp(0.5, 10.0);
        draw_circle(body.position.x, body.position.y, mass, color);
    }
}

pub fn draw_bh(node: &BHTreeNode) {
    let width = node.region_width;

    if width < 0.75 {
        return;
    }

    let pos = node.region_center - Vec2::new(width / 2.0, width / 2.0);
    let color = Color::new(1.0, 1.0, 1.0, 0.1);
    draw_rectangle_lines(pos.x, pos.y, width, width, 1.0, color);

    for child in node.children.iter() {
        draw_bh(child);
    }
}

fn draw_vector(start: Vec2, end: Vec2, thickness: f32, color: Color) {
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
    let position = body.position;

    let vel_color = Color::new(0.25, 1.0, 0.25, 0.15);
    draw_vector(position, position + body.velocity, 2.0, vel_color);

    let acc_color = Color::new(1.0, 0.25, 0.25, 0.15);
    draw_vector(position, position + body.acceleration, 2.0, acc_color);
}