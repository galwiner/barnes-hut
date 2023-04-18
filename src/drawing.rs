use nannou::color;
use nannou::color::IntoLinSrgba;
use nannou::draw::primitive::Rect as RectPrimitive;
use nannou::draw::properties::ColorScalar;
use nannou::Draw;

use crate::geometry::{BoundingBox, Positioned};
use crate::particle::Particle;
use crate::quad_tree::iterator::{TreeIterator, TreePosition};
use crate::quad_tree::QuadTree;
use crate::Model;

pub fn draw_all(model: &Model, draw: &Draw, frame: BoundingBox) {
    draw_tree(&model.qt, &draw, &model, frame);
    draw_rect(model.inspector, &draw, color::LIGHTCORAL);
}

fn draw_tree(tree: &QuadTree<Particle>, draw: &Draw, model: &Model, frame: BoundingBox) {
    tree.iter()
        .bounded(frame)
        .for_each(|position| match position {
            TreePosition::Leaf(leaf) => {
                let position = leaf.position();
                let color = if model.inspector.contains(position) {
                    color::RED
                } else {
                    color::GREEN
                };
                draw.ellipse()
                    .xy(position)
                    .w_h(2.0, 2.0)
                    .stroke(color::BLACK)
                    .color(color);
            }
            TreePosition::Node(node) => draw_rect(node.boundary, draw, color::RED),
        });
}

pub fn draw_rect(boundary: BoundingBox, draw: &Draw, color: impl IntoLinSrgba<ColorScalar>) {
    draw.a(RectPrimitive::from(boundary))
        .rgba(0.0, 0.0, 0.0, 0.0)
        .stroke(color)
        .stroke_weight(0.5);
}
