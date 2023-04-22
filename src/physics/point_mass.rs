use std::ops::AddAssign;

use crate::physics::space::Space;

#[derive(Debug, Clone, Copy, Default)]
pub struct PointMass<S: Space> {
    pub position: S::Vector,
    pub mass: S::Scalar,
}

impl<S: Space> AddAssign for PointMass<S> {
    fn add_assign(&mut self, rhs: Self) {
        let total_mass: S::Scalar = self.mass + rhs.mass;
        if total_mass > S::ZERO {
            self.position = (self.position * self.mass + rhs.position * rhs.mass) / total_mass;
            self.mass = total_mass;
        }
    }
}
