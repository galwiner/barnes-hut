use std::ops::AddAssign;

use crate::physics::space::Space;

#[derive(Debug, Clone, Copy, Default)]
pub struct PointMass<S: Space> {
    pub position: S::Vector,
    pub mass: S::Scalar,
}

impl<S: Space> PointMass<S> {
    pub fn new(position: S::Vector, mass: S::Scalar) -> Self {
        Self { position, mass }
    }

    pub fn g_at(&self, target: S::Vector, grav_constant: S::Scalar) -> S::Vector {
        let target_to_self: S::Vector = self.position - target;
        let distance_squared: S::Scalar = S::magnitude_squared(target_to_self);
        if distance_squared <= S::MIN_GRAVITY_DISTANCE_SQUARED {
            return S::VECTOR_ZERO;
        }
        let g: S::Scalar = self.mass * grav_constant / (distance_squared);
        S::normalize(target_to_self) * g
    }
}

impl<S: Space> AddAssign for PointMass<S> {
    fn add_assign(&mut self, rhs: Self) {
        let total_mass: S::Scalar = self.mass + rhs.mass;
        if total_mass > S::SCALAR_ZERO {
            self.position =
                self.position * (self.mass / total_mass) + rhs.position * (rhs.mass / total_mass);
            self.mass = total_mass;
        }
    }
}
