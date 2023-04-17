pub use nannou::geom::Point2;
use nannou::geom::Rect;

pub trait Positioned {
    fn position(&self) -> Point2;
}

impl Positioned for Point2 {
    fn position(&self) -> Point2 {
        *self
    }
}

pub type BoundingBox = Rect;

fn _x() {
    let rect = BoundingBox::from_wh(Point2::ZERO);
    rect.xy();
}
