use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

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
    const VECTOR_ZERO: Self::Vector;
    const SCALAR_ZERO: Self::Scalar;
    const TWO: Self::Scalar;
    const EPSILON: Self::Scalar;
    const EPSILON_SQUARED: Self::Scalar;
    const MIN_GRAVITY_DISTANCE_SQUARED: Self::Scalar;

    fn magnitude_squared(vector: Self::Vector) -> Self::Scalar;
    fn normalize(vector: Self::Vector) -> Self::Vector;
}

pub trait DivisibleSpace<const NUM_SUBDIVISIONS: usize>: Space {
    fn subdivisions_array_default<T: Default>() -> [T; NUM_SUBDIVISIONS];
    fn max_abs_dimension(vector: Self::Vector) -> Self::Scalar;
    fn subdivision_index(pivot: Self::Vector, point: Self::Vector) -> usize;

    fn subtree_width_pivot(
        i: usize,
        width: Self::Scalar,
        pivot: Self::Vector,
    ) -> (Self::Scalar, Self::Vector);
}
