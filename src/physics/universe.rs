use std::mem;

use nannou::draw::primitive;
use nannou::prelude::*;

use crate::drawing::{alpha, Drawable, TRANSPARENT};
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

    fn particles(&self) -> impl Iterator<Item = &Particle> {
        self.space.iter().leaves()
    }

    pub(super) fn net_force_on(&self, particle: &Particle) -> Point2 {
        const SUN_POSITION: Point2 = Point2::ZERO;
        let from_sun = particle.position() - SUN_POSITION;
        let distance = from_sun.length() / Self::SCALE;
        let g = particle.mass * Self::G / (distance * distance);
        g * -from_sun.normalize()
    }

    pub(super) fn insert(&mut self, particle: Particle) {
        self.space.insert(particle);
    }
}

impl Drawable for Universe {
    fn draw(&self, draw: &Draw, bounds: Rect, view_state: &ViewState) {
        let iter = || self.space.iter().bounded(bounds);
        if view_state.draw_particles {
            iter().leaves().for_each(|particle| {
                particle.draw(draw, view_state);
            });
        }
        if view_state.draw_quad_tree {
            iter().nodes().for_each(|node| {
                draw.a(primitive::Rect::from(node.boundary()))
                    .color(TRANSPARENT)
                    .stroke(alpha(RED, 0.2))
                    .stroke_weight(0.5 / view_state.scale);
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

        let update_particle = |particle: &Particle| {
            let mut particle = particle.clone();
            particle.update(dt, &old_universe);
            particle
        };

        #[cfg(feature = "parallel")]
        self.space.extend({
            use itertools::Itertools;
            use rayon::prelude::*;

            old_universe
                .particles()
                .collect_vec()
                .par_iter()
                .cloned()
                .map(update_particle)
                .collect::<Vec<_>>()
        });

        #[cfg(not(feature = "parallel"))]
        self.space
            .extend(old_universe.particles().map(update_particle));
    }
}
