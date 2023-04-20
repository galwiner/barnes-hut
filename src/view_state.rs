use nannou::geom::Rect;
use nannou::geom::{pt2, Point2};

pub struct ViewState {
    pub bounds: Rect,
    pub inspector: Option<Rect>,
    pub draw_particles: bool,
}

impl ViewState {
    pub fn new() -> Self {
        Self {
            bounds: Rect::from_w_h(0.0, 0.0),
            inspector: None,
            draw_particles: true,
        }
    }

    pub fn inspect_at(&mut self, position: Point2) {
        self.inspector = Some(Rect::from_xy_wh(position, pt2(100.0, 100.0)));
    }

    pub fn toggle_draw_particles(&mut self) {
        self.draw_particles ^= true;
        info!("draw_particles: {}", self.draw_particles);
    }
}
