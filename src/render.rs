use super::body::Body;
use crate::barnes_hut::BHTreeNode;

use glam::Vec2;
use macroquad::prelude::{draw_circle, draw_rectangle_lines, Color};


pub fn draw_bodies(bodies: &Vec<Body>) {
    for body in bodies.iter() {
        let speed = body.velocity.length() / 5.0 + 0.5;
        let color = Color::new(speed, speed, speed, 0.5);
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