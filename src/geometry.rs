use std::borrow::Borrow;

pub use nannou::geom::Point2;
use nannou::geom::Rect;

pub trait Positioned {
    fn position(&self) -> Point2;
}

impl<T: Borrow<Point2>> Positioned for T {
    fn position(&self) -> Point2 {
        *self.borrow()
    }
}

pub type BoundingBox = Rect;

#[cfg(test)]
mod tests {
    use super::*;

    assert_impl_all!(Point2: Positioned);
    assert_impl_all!(&Point2: Positioned);
    assert_impl_all!(Box<Point2>: Positioned);
}
