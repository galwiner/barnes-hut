use nannou::prelude::*;

use crate::drawing::Drawable;
use crate::physics::barnes_hut::GravityField2D;
use crate::physics::point_mass::PointMass;
use crate::simulation;
use crate::view_state::ViewState;

use super::particle::Particle;

#[derive(Debug, Clone, Default)]
pub struct Universe {
    particles: Vec<Particle>,
}

impl Universe {
    pub const G: f32 = 1e2;
    pub const THETA: f32 = 0.7;

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

    pub(super) fn insert(&mut self, particle: Particle) {
        self.particles.push(particle);
    }
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
            // TODO?
        }
    }
}

impl simulation::Model for Universe {
    fn step(&mut self, dt: f32) {
        let bounds: Rect = self
            .particles
            .iter()
            .fold(Rect::from_w_h(0.0, 0.0), |bounds, particle| {
                bounds.stretch_to(particle.position)
            });
        let size = bounds.w().max(bounds.h());

        let mut gravity_field = GravityField2D::new(size);

        for particle in &self.particles {
            gravity_field += PointMass::new(particle.position, particle.mass);
        }
        gravity_field += PointMass::new(Point2::new(0.0, 0.0), 1e3);
        let update_particle = |particle: &mut Particle| {
            let net_g = gravity_field.estimate_net_g(particle.position, Self::THETA, Self::G);
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

    fn stats_string(&self) -> String {
        format!("p:{:6} ", self.particles.len())
    }
}
