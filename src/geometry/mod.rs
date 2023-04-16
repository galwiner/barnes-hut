mod bounding_box;

use nannou::prelude::Point2;
pub use bounding_box::BoundingBox;

pub trait Positioned {
    fn position(&self) -> Point2;
}