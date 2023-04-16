use std::ops::Range;

use nannou::geom::Point2;

pub struct BoundingBox {
    pub x_range: Range<f32>,
    pub y_range: Range<f32>,
}

impl BoundingBox {
    pub fn new(x_range: Range<f32>, y_range: Range<f32>) -> Self {
        Self { x_range, y_range }
    }

    pub fn about_point(center: &Point2, radius: f32) -> Self {
        Self::new(
            (center.x - radius)..(center.x + radius),
            (center.y - radius)..(center.y + radius),
        )
    }

    pub fn contains_point(&self, point: &Point2) -> bool {
        self.x_range.contains(&point.x) && self.y_range.contains(&point.y)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x_range.start < other.x_range.end
            && self.x_range.end > other.x_range.start
            && self.y_range.start < other.y_range.end
            && self.y_range.end > other.y_range.start
    }

    pub fn center(&self) -> Point2 {
        Point2::new(
            self.x_range.start + (self.x_range.end - self.x_range.start) / 2.0,
            self.y_range.start + (self.y_range.end - self.y_range.start) / 2.0,
        )
    }

    pub fn size(&self) -> Point2 {
        Point2::new(
            self.x_range.end - self.x_range.start,
            self.y_range.end - self.y_range.start,
        )
    }

    pub fn subdivide(&self) -> [Self; 4] {
        let center = self.center();
        [
            Self::new(self.x_range.start..center.x, self.y_range.start..center.y),
            Self::new(center.x..self.x_range.end, self.y_range.start..center.y),
            Self::new(self.x_range.start..center.x, center.y..self.y_range.end),
            Self::new(center.x..self.x_range.end, center.y..self.y_range.end),
        ]
    }
}
