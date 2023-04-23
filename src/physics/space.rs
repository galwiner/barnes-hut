use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

use nannou::geom::{rect, Point2, Range, Rect};

pub trait Space: Default + Copy + Debug {
    type Scalar: Copy
        + Debug
        + PartialEq
        + PartialOrd
        + Default
        + Add<Output = Self::Scalar>
        + Sub<Output = Self::Scalar>
        + Mul<Output = Self::Scalar>
        + Div<Output = Self::Scalar>;
    type Vector: Copy
        + Debug
        + PartialEq
        + Default
        + Add<Output = Self::Vector>
        + Sub<Output = Self::Vector>
        + AddAssign<Self::Vector>
        + Mul<Self::Scalar, Output = Self::Vector>
        + Div<Self::Scalar, Output = Self::Vector>;
    const ZERO_VECTOR: Self::Vector;
    const ZERO: Self::Scalar;

    fn magnitude(vector: Self::Vector) -> Self::Scalar;
    fn magnitude_squared(vector: Self::Vector) -> Self::Scalar {
        let magnitude = Self::magnitude(vector);
        magnitude * magnitude
    }
    fn normalize(vector: Self::Vector) -> Self::Vector {
        let magnitude = Self::magnitude(vector);
        if magnitude > Self::ZERO {
            vector / magnitude
        } else {
            Self::ZERO_VECTOR
        }
    }
}

pub trait DivisibleSpace<const NUM_SUBDIVISIONS: usize>: Space {
    type Bounds: Copy + PartialEq;
    const EMPTY_BOUNDS: Self::Bounds;

    fn point_bounds(point: Self::Vector) -> Self::Bounds;
    fn expand_bounds(bounds: Self::Bounds, point: Self::Vector) -> Self::Bounds;
    fn midpoint(bounds: Self::Bounds) -> Self::Vector;
    fn max_dimension(bounds: Self::Bounds) -> Self::Scalar;
    fn contains(bounds: Self::Bounds, point: Self::Vector) -> bool;

    fn subdivision_index(pivot: Self::Vector, point: Self::Vector) -> usize;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Space2D;

impl Space for Space2D {
    type Scalar = f32;
    type Vector = Point2;
    const ZERO_VECTOR: Self::Vector = Point2::ZERO;
    const ZERO: Self::Scalar = 0.0;

    fn normalize(vector: Self::Vector) -> Self::Vector {
        vector.normalize()
    }

    fn magnitude(vector: Self::Vector) -> Self::Scalar {
        vector.length()
    }

    fn magnitude_squared(vector: Self::Vector) -> Self::Scalar {
        vector.length_squared()
    }
}

impl DivisibleSpace<{ rect::NUM_SUBDIVISIONS as usize }> for Space2D {
    type Bounds = Rect;
    const EMPTY_BOUNDS: Rect = Rect {
        x: Range {
            start: 0.0,
            end: 0.0,
        },
        y: Range {
            start: 0.0,
            end: 0.0,
        },
    };
    fn point_bounds(point: Self::Vector) -> Self::Bounds {
        Rect::from_corners(point, point)
    }

    fn expand_bounds(bounds: Self::Bounds, point: Self::Vector) -> Self::Bounds {
        bounds.stretch_to(point)
    }

    fn midpoint(bounds: Self::Bounds) -> Self::Vector {
        bounds.xy()
    }

    fn max_dimension(bounds: Self::Bounds) -> Self::Scalar {
        bounds.wh().max_element()
    }

    fn contains(bounds: Self::Bounds, point: Self::Vector) -> bool {
        bounds.contains(point)
    }

    fn subdivision_index(pivot: Self::Vector, point: Self::Vector) -> usize {
        match (point.x >= pivot.x, point.y >= pivot.y) {
            (true, true) => 0,
            (false, true) => 1,
            (false, false) => 2,
            (true, false) => 3,
        }
    }
}
