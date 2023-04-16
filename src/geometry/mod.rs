use nannou::prelude::Point2;

pub use bounding_box::BoundingBox;

mod bounding_box;

pub trait Positioned {
    fn position(&self) -> Point2;
}
