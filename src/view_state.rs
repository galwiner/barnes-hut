use nannou::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct ViewState {
    /// The bounds of the inspector window in app coordinates.
    inspector: Option<Rect>,
    pub draw_particles: bool,

    pub pan: Point2,
    pub scale: f32,
}

const INSPECTOR_SIZE: f32 = 100.0;

impl Default for ViewState {
    fn default() -> Self {
        Self {
            inspector: None,
            draw_particles: true,
            pan: Point2::ZERO,
            scale: 1.0,
        }
    }
}

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

    pub fn inspect_at(&mut self, position: Point2) {
        self.inspector = Some(Rect::from_xy_wh(
            position,
            self.inspector
                .map(|r| r.wh())
                .unwrap_or(Point2::splat(INSPECTOR_SIZE)),
        ));
    }

    pub fn inspector_app_bounds(&self) -> Option<Rect> {
        self.inspector
    }

    /// in universe coordinates
    pub fn inspector_bounds(&self) -> Option<Rect> {
        self.inspector.map(|r| self.rect_to_universe(r))
    }

    pub fn rect_to_universe(&self, rect: Rect) -> Rect {
        Rect::from_xy_wh((rect.xy() - self.pan) / self.scale, rect.wh() / self.scale)
    }

    pub fn toggle_draw_particles(&mut self) {
        self.draw_particles ^= true;
        info!("draw_particles: {}", self.draw_particles);
    }
}
