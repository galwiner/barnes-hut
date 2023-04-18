use nannou::geom::{pt2, Point2};

use crate::geometry::BoundingBox;

pub struct ViewState {
    pub bounds: BoundingBox,
    pub inspector: Option<BoundingBox>,
    pub draw_particles: bool,
}

impl ViewState {
    pub fn new() -> Self {
        Self {
            bounds: BoundingBox::from_w_h(0.0, 0.0),
            inspector: None,
            draw_particles: true,
        }
    }

    pub fn inspect_at(&mut self, position: Point2) {
        self.inspector = Some(BoundingBox::from_xy_wh(position, pt2(100.0, 100.0)));
    }

    pub fn toggle_draw_particles(&mut self) {
        self.draw_particles ^= true;
        println!("draw_particles: {}", self.draw_particles);
    }
}
