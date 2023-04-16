use nannou::color::IntoLinSrgba;
use nannou::Draw;
use nannou::draw::properties::ColorScalar;
use nannou::prelude::RED;
use crate::geometry::BoundingBox;
use crate::Model;
use crate::quad_tree::{QuadTree, QuadTreeChildren};

pub trait Drawable {
    fn draw(&self, draw: &Draw, model: &Model);
}

impl<Leaf: Drawable> Drawable for QuadTree<Leaf> {
    fn draw(&self, draw: &Draw, model: &Model) {
        draw_bounding_box(&self.boundary, draw, RED);
        self.children.draw(draw, model);
    }
}

pub fn draw_bounding_box(boundary: &BoundingBox, draw: &Draw, color: impl IntoLinSrgba<ColorScalar>) {
    let center = boundary.center();
    let size = boundary.size();
    draw.rect()
        .x_y(center.x, center.y)
        .w_h(size.x, size.y)
        .rgba(0.0, 0.0, 0.0, 0.0)
        .stroke(color)
        .stroke_weight(0.5);
}

impl<Leaf: Drawable> Drawable for QuadTreeChildren<Leaf> {
    fn draw(&self, draw: &Draw, model: &Model) {
        match self {
            QuadTreeChildren::Leaves(leaves) => {
                leaves.iter().for_each(|leaf| leaf.draw(draw, model));
            }
            QuadTreeChildren::Nodes(nodes) => {
                nodes.iter().for_each(|node| node.draw(draw, model));
            }
        }
    }
}
