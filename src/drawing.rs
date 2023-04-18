use nannou::color::{IntoLinSrgba, LinSrgba};
use nannou::draw::primitive::Rect as RectPrimitive;
use nannou::draw::properties::ColorScalar;
use nannou::Draw;

use crate::geometry::BoundingBox;

pub fn draw_rect(rect: BoundingBox, draw: &Draw, color: impl IntoLinSrgba<ColorScalar>) {
    draw.a(RectPrimitive::from(rect))
        .rgba(0.0, 0.0, 0.0, 0.0)
        .stroke(color.into_lin_srgba())
        .stroke_weight(0.5);
}

pub fn alpha(color: impl IntoLinSrgba<ColorScalar>, alpha: f32) -> LinSrgba {
    let mut color = color.into_lin_srgba();
    color.alpha = alpha;
    color
}
