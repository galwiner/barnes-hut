use nannou::color::{Alpha, IntoLinSrgba};
use nannou::draw::primitive;
use nannou::draw::properties::ColorScalar;
use nannou::prelude::*;

use crate::view_state::ViewState;

pub trait Drawable {
    fn draw(&self, draw: &Draw, bounds: Rect, view_state: &ViewState);
}

pub fn draw_rect(rect: Rect, draw: &Draw, color: impl IntoLinSrgba<ColorScalar>) {
    draw.a(primitive::Rect::from(rect))
        .color(TRANSPARENT)
        .stroke(color.into_lin_srgba())
        .stroke_weight(0.5);
}

pub fn alpha(color: impl IntoLinSrgba<ColorScalar>, alpha: f32) -> LinSrgba {
    let mut color = color.into_lin_srgba();
    color.alpha = alpha;
    color
}

pub const TRANSPARENT: Alpha<Srgb<u8>, f32> = Alpha {
    color: BLACK,
    alpha: 0.0,
};
