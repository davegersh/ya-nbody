use super::ScreenState;

use crate::physics::Body;
use crate::physics::barnes_hut::BHTreeNode;

use glam::Vec2;
use macroquad::prelude::{draw_circle, draw_rectangle_lines, Color};

pub fn draw_bodies(bodies: &Vec<Body>, screen_state: &ScreenState) {
    for (_, body) in bodies.iter().enumerate() {        
        let color = Color::new(1., 1.,1., 0.1);
        let radius = (body.mass / 2.0).clamp(0.5, 10.0);

        let position = screen_state.transform_position(body.position);
        
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
