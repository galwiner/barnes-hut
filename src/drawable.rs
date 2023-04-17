use nannou::color::IntoLinSrgba;
use nannou::draw::primitive::Rect as RectPrimitive;
use nannou::draw::properties::ColorScalar;
use nannou::Draw;

use crate::geometry::BoundingBox;
use crate::Model;

pub trait Drawable {
    fn draw(&self, draw: &Draw, model: &Model);
}

pub fn draw_bounding_box(
    boundary: BoundingBox,
    draw: &Draw,
    color: impl IntoLinSrgba<ColorScalar>,
) {
    draw.a(RectPrimitive::from(boundary))
        .rgba(0.0, 0.0, 0.0, 0.0)
        .stroke(color)
        .stroke_weight(0.5);
}
