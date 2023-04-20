use std::mem;

use nannou::geom::{Point2, Rect};
use nannou::{color, Draw};

use crate::drawing::{alpha, draw_rect, Drawable};
use crate::quad_tree::iterator::TreeIterator;
use crate::quad_tree::Positioned;
use crate::quad_tree::QuadTree;
use crate::simulation;
use crate::view_state::ViewState;

use super::particle::Particle;

#[derive(Debug, Clone)]
pub struct Universe {
    space: QuadTree<Particle>,
}

impl Universe {
    pub const SIZE: f32 = 1e6;
    pub const SCALE: f32 = 20.0;
    pub const G: f32 = 800.0;

    pub fn new(num_particles: usize) -> Self {
        let mut new = Self {
            space: Self::empty_space(),
        };
        new.add_random_particles(num_particles);
        new
    }

    pub fn clear(&mut self) {
        self.space = Self::empty_space();
    }

    fn empty_space() -> QuadTree<Particle> {
        QuadTree::new(Rect::from_w_h(Self::SIZE, Self::SIZE))
    }

    pub fn add_particle_at(&mut self, position: Point2) {
        self.insert(Particle::new(position));
    }

    pub fn add_random_particles(&mut self, num_particles: usize) {
        for _ in 0..num_particles {
            self.insert(Particle::new_random());
        }
    }

    pub(super) fn net_force_on(&self, particle: &Particle) -> Point2 {
        const SUN_POSITION: Point2 = Point2::ZERO;
        let from_sun = particle.position() - SUN_POSITION;
        let distance = from_sun.length() / Self::SCALE;
        let g = particle.mass * Self::G / (distance * distance);
        g * -from_sun.normalize()
    }

    pub(super) fn insert(&mut self, particle: Particle) {
        if let Err(err) = self.space.insert(particle) {
            error!(
                "Failed to insert particle at {:?}: {err:?}",
                particle.position()
            );
        }
    }

    fn draw_particle(draw: &Draw, p: &Particle, view_state: &ViewState) {
        let position = p.position();
        let in_inspector = view_state
            .inspector
            .iter()
            .any(|inspector| inspector.contains(position));
        p.draw(draw, in_inspector);
    }
}

impl Drawable for Universe {
    fn draw(&self, draw: &Draw, bounds: Rect, view_state: &ViewState) {
        let bounded = self.space.iter().bounded(bounds);
        bounded.clone().nodes().for_each(|node| {
            draw_rect(node.boundary(), draw, alpha(color::RED, 0.2));
        });
        if view_state.draw_particles {
            bounded.leaves().for_each(|particle| {
                Self::draw_particle(draw, particle, view_state);
            });
        }
    }
}

impl simulation::Model for Universe {
    fn step(&mut self, dt: f32) {
        let old_universe = mem::replace(
            &mut *self,
            Self {
                space: Self::empty_space(),
            },
        );
        for particle in old_universe.space.iter().leaves() {
            let mut particle = particle.clone();
            particle.update(dt, &old_universe);
            self.insert(particle);
        }
    }
}
