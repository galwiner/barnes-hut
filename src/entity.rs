use crate::drawable::Drawable;
use crate::geometry::Positioned;

pub trait Entity: Positioned + Drawable {
    fn update(&mut self);
}
