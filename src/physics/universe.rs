use nannou::prelude::*;

use crate::drawing::Drawable;
use crate::physics::barnes_hut::GravityField2D;
use crate::physics::point_mass::PointMass;
use crate::quad_tree::Positioned;
use crate::simulation;
use crate::view_state::ViewState;

use super::particle::Particle;

#[derive(Debug, Clone, Default)]
pub struct Universe {
    particles: Vec<Particle>,
    // space: QuadTree<Particle>,
}

impl Universe {
    pub const SIZE: f32 = 1e6;
    pub const SCALE: f32 = 20.0;
    pub const G: f32 = 800.0;

    pub fn new(num_particles: usize) -> Self {
        let mut new = Self::default();
        new.add_random_particles(num_particles);
        new
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }

    pub fn add_particle_at(&mut self, position: Point2) {
        self.insert(Particle::new(position));
    }

    pub fn add_random_particles(&mut self, num_particles: usize) {
        for _ in 0..num_particles {
            self.insert(Particle::new_random());
        }
    }

    // pub(super) fn net_force_on(&self, particle: &Particle) -> Point2 {
    //     const SUN_POSITION: Point2 = Point2::ZERO;
    //     let from_sun = particle.position() - SUN_POSITION;
    //     let distance = from_sun.length() / Self::SCALE;
    //     let g = particle.mass * Self::G / (distance * distance);
    //     g * -from_sun.normalize()
    // }

    pub(super) fn insert(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    /*
    pub fn approx_g_at(
        &self,
        point: S::Vector,
        theta: S::Scalar,
        grav_constant: S::Scalar,
    ) -> S::Vector {
        // Barnes-hut approximation:

        match &self.subdivisions {
            None => {
                // If we're a leaf node, calculate the acceleration directly.
                let distance_squared = S::magnitude_squared(self.total.position - point);
                if distance_squared == S::ZERO {
                    return S::ORIGIN;
                }
                let magnitude = grav_constant * self.total.mass / distance_squared;
                let direction = S::normalize(self.total.position - point);
                direction * magnitude
            }
            Some(subtrees) => {
                let sum = S::ORIGIN;
                for subtree in subtrees.iter() {
                    todo!()
                }
                todo!()
            }
        }
    }

    fn single_mass_g(
        &mut self,
        body: PointMass<S>,
        position: S::Vector,
        grav_constant: S::Scalar,
    ) -> S::Vector {
        if (body.mass == S::ZERO) || (self.total.mass == S::ZERO) {
            return S::ORIGIN;
        }
        let distance_squared = S::magnitude_squared(body.position - position);
        if distance_squared == S::ZERO {
            return S::ORIGIN;
        }
        let magnitude = grav_constant * self.total.mass / distance_squared;
        let direction = S::normalize(body.position - position);
        return direction * magnitude;
    }
     */
}

impl Drawable for Universe {
    fn draw(&self, draw: &Draw, bounds: Rect, view_state: &ViewState) {
        if view_state.draw_particles {
            for particle in &self.particles {
                if bounds.contains(particle.position) {
                    particle.draw(draw, view_state);
                }
                particle.draw(draw, view_state);
            }
        }
        if view_state.draw_quad_tree {
            // TODO
        }
    }
}

impl simulation::Model for Universe {
    fn step(&mut self, dt: f32) {
        let mut gravity_field = GravityField2D::default();
        for particle in &self.particles {
            gravity_field += PointMass::new(particle.position(), particle.mass);
        }
        let update_particle = |particle: &mut Particle| {
            let net_g = gravity_field.estimate_net_g(particle.position, 0.5, Self::G);
            particle.update(dt, net_g);
        };

        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;
            self.particles.par_iter_mut().for_each(update_particle);
        }
        #[cfg(not(feature = "parallel"))]
        {
            self.particles.iter().for_each(update_particle);
        }
    }
}
