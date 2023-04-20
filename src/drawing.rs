use nannou::color::{IntoLinSrgba, LinSrgba};
use nannou::draw::primitive::Rect as RectPrimitive;
use nannou::draw::properties::ColorScalar;
use nannou::geom::Rect;
use nannou::Draw;

use crate::view_state::ViewState;

pub trait Drawable {
    fn draw(&self, draw: &Draw, bounds: Rect, view_state: &ViewState);
}

pub fn draw_rect(rect: Rect, draw: &Draw, color: impl IntoLinSrgba<ColorScalar>) {
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
