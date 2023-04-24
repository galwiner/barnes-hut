use nannou::prelude::*;

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(Default)]
pub struct ViewState {
    /// The bounds of the inspector window in app coordinates.
    inspector: Option<Rect>,

    #[derivative(Default(value = "true"))]
    pub draw_particles: bool,

    pub draw_quad_tree: bool,

    pub pan: Point2,

    #[derivative(Default(value = "1.0"))]
    pub scale: f32,

    mouse_pan_prev_position: Option<Point2>,
}

const INSPECTOR_SIZE: f32 = 100.0;

impl ViewState {
    pub fn universe_to_app_transform(&self) -> Mat4 {
        let pan = self.pan.extend(0.0);
        let scale = Vec3::splat(self.scale);
        Mat4::from_translation(pan) * Mat4::from_scale(scale)
    }

    pub fn zoom_at(&mut self, zoom_center_app: Point2, scale_factor: f32) {
        self.scale *= scale_factor;
        self.pan = self.pan + (zoom_center_app - self.pan) * (1.0 - scale_factor);
    }

    pub fn reset_zoom(&mut self) {
        self.scale = 1.0;
    }

    pub fn reset_pan(&mut self) {
        self.pan = Point2::ZERO;
    }

    pub fn mouse_pan(&mut self, position: Point2) {
        if let Some(prev_position) = self.mouse_pan_prev_position {
            self.pan += position - prev_position;
        }
        self.mouse_pan_prev_position = Some(position);
    }

    pub fn end_mouse_pan(&mut self) {
        self.mouse_pan_prev_position = None;
    }

    pub fn min_universe_feature_size(&self) -> f32 {
        2.0 / self.scale
    }

    pub fn inspect_at(&mut self, position: Point2) {
        self.inspector = Some(Rect::from_xy_wh(
            position,
            self.inspector
                .map(|r| r.wh())
                .unwrap_or(Point2::splat(INSPECTOR_SIZE)),
        ));
    }

    pub fn is_inspecting(&self, point: Point2) -> bool {
        self.inspector_bounds()
            .map(|r| r.contains(point))
            .unwrap_or(false)
    }

    pub fn inspector_app_bounds(&self) -> Option<Rect> {
        self.inspector
    }

    /// in universe coordinates
    pub fn inspector_bounds(&self) -> Option<Rect> {
        self.inspector.map(|r| self.to_universe_rect(r))
    }

    pub fn to_universe_point(&self, point: Point2) -> Point2 {
        (point - self.pan) / self.scale
    }

    pub fn to_universe_rect(&self, rect: Rect) -> Rect {
        Rect::from_xy_wh(self.to_universe_point(rect.xy()), rect.wh() / self.scale)
    }

    pub fn cycle_drawn_stuff(&mut self) {
        (self.draw_particles, self.draw_quad_tree) =
            match (self.draw_particles, self.draw_quad_tree) {
                (true, false) => (true, true),
                (true, true) => (false, true),
                _ => (true, false),
            };
        info!(
            "drawing particles: {}, quad tree: {}",
            self.draw_particles, self.draw_quad_tree
        );
    }
}
