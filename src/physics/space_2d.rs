use nannou::geom::{pt2, Point2};

use crate::physics::space::{DivisibleSpace, Space};

#[derive(Debug, Default, Clone, Copy)]
pub struct Space2D;

impl Space for Space2D {
    type Scalar = f32;
    type Vector = Point2;
    const VECTOR_ZERO: Self::Vector = Point2::ZERO;
    const SCALAR_ZERO: Self::Scalar = 0.0;
    const TWO: Self::Scalar = 2.0;
    const EPSILON: Self::Scalar = 1e-6;
    const EPSILON_SQUARED: Self::Scalar = Self::EPSILON * Self::EPSILON;
    const MIN_GRAVITY_DISTANCE_SQUARED: Self::Scalar = 1.0;

    fn magnitude_squared(vector: Self::Vector) -> Self::Scalar {
        vector.length_squared()
    }

    fn normalize(vector: Self::Vector) -> Self::Vector {
        vector.normalize_or_zero()
    }
}

impl DivisibleSpace<4> for Space2D {
    fn subdivisions_array_default<T: Default>() -> [T; 4] {
        Default::default()
    }

    fn max_abs_dimension(vector: Self::Vector) -> Self::Scalar {
        vector.abs().max_element()
    }

    fn subdivision_index(pivot: Self::Vector, point: Self::Vector) -> usize {
        match (point.x >= pivot.x, point.y >= pivot.y) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        }
    }

    fn subtree_width_pivot(
        i: usize,
        width: Self::Scalar,
        pivot: Self::Vector,
    ) -> (Self::Scalar, Self::Vector) {
        let width: Self::Scalar = width / Self::TWO;
        let pivot_offset = match i {
            0 => pt2(width, width),
            1 => pt2(width, -width),
            2 => pt2(-width, width),
            3 => pt2(-width, -width),
            _ => panic!("Invalid subdivision index: {}", i),
        };
        (width, pivot + pivot_offset)
    }
}
