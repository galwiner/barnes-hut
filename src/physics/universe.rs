use nannou::color::Gradient;
use nannou::prelude::*;

use crate::drawing::Drawable;
use crate::physics::barnes_hut::{GravityField, GravityField2D, MassAggregate};
use crate::physics::point_mass::PointMass;
use crate::physics::space_2d::Space2D;
use crate::simulation;
use crate::view_state::ViewState;

use super::particle::Particle;

#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct Universe {
    particles: Vec<Particle>,
    bounding_boxes : Vec<Rect>,
    #[derivative(Default(value = "1e3"))]
    pub black_hole_mass: f32
}

impl Universe {
    pub(crate) fn set_black_hole_mass(&mut self, fac:f32) {
        self.black_hole_mass = fac;
        info!(
            "Blackhole mass is now: {}",
            self.black_hole_mass
        );
    }
}

impl Universe {
    pub(crate) fn multiply_black_hole_mass(&mut self, fac: f32) {
         self.black_hole_mass *= fac;
        info!(
            "Blackhole mass is now: {}",
            self.black_hole_mass
        );
    }
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

    fn get_bounding_box(&self) -> Rect {
        self
            .particles
            .iter()
            .fold(Rect::from_w_h(0.0, 0.0), |bounds, particle| {
                bounds.stretch_to(particle.position)
            })
    }

    fn gravity_field(&self) -> GravityField2D {
        let bounds = self.get_bounding_box();
        let size = bounds.w().max(bounds.h());

        GravityField2D::new(size)
    }
}

impl Drawable for Universe {
    fn draw(&self, draw: &Draw, bounds: Rect, view_state: &ViewState) {
        if view_state.draw_particles {
            let max_v = get_max_velocity(&self.particles);
            let gradient = get_gradient();
            for particle in &self.particles {
                if bounds.contains(particle.position) {
                    particle.draw(draw, view_state,&gradient,max_v);
                }
                particle.draw(draw, view_state,&gradient,max_v);
            }
        }
        if view_state.draw_quad_tree {
            self.bounding_boxes.iter().for_each(|bb| {
                draw.rect()
                    .xy(bb.xy())
                    .wh(bb.wh())
                    .stroke_weight(1.0)
                    .stroke_color(RED)
                    .no_fill();
            });
        }
    }
}

fn get_gradient() -> Gradient<LinSrgb> {
    Gradient::new(vec![
            LinSrgb::new(0.0, 0.0, 1.0),
            LinSrgb::new(1.0, 0.0, 0.0)
        ])
}

fn get_max_velocity(particles: &Vec<Particle>) -> f32 {
    particles
        .iter()
        .map(|p| p.velocity.length())
        .fold(0.0, |max, v| max.max(v as f64)) as f32
}

impl simulation::Model for Universe {
    fn step(&mut self, dt: f32) {
        let mut gravity_field = self.gravity_field();

        for particle in &self.particles {
            gravity_field += PointMass::new(particle.position, particle.mass);
        }

        gravity_field += PointMass::new(Point2::new(0.0, 0.0), self.black_hole_mass);

        self.bounding_boxes = gravity_field.get_bounding_boxes();
        let update_particle = |particle: &mut Particle| {
            let net_g = gravity_field.estimate_net_g(particle.position, Self::THETA, Self::G);
            particle.update(dt, net_g);
        };

        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            self.particles.par_iter_mut().for_each(update_particle);
        }
        #[cfg(not(feature = "rayon"))]
        {
            self.particles.iter_mut().for_each(update_particle);
        }
    }

    fn stats_string(&self) -> String {
        format!("p:{:6} ", self.particles.len())
    }
}
