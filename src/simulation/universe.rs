use std::mem;

use nannou::geom::{Point2, Rect};
use nannou::{color, Draw};

use crate::drawing::{alpha, draw_rect};
use crate::geometry::Positioned;
use crate::quad_tree::iterator::TreeIterator;
use crate::quad_tree::QuadTree;
use crate::simulation::particle::Particle;
use crate::view_state::ViewState;

#[derive(Debug, Clone)]
pub(super) struct Universe {
    space: QuadTree<Particle>,
}

impl Default for Universe {
    fn default() -> Self {
        Self {
            space: Self::empty_space(),
        }
    }
}

impl Universe {
    pub const SIZE: f32 = 1e6;
    pub const SCALE: f32 = 20.0;
    pub const G: f32 = 800.0;

    pub(super) fn new() -> Self {
        Self::default()
    }

    fn empty_space() -> QuadTree<Particle> {
        QuadTree::new(Rect::from_w_h(Self::SIZE, Self::SIZE))
    }

    pub(super) fn step(&mut self, dt: f32) {
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

    pub(super) fn force_on(&self, particle: &Particle) -> Point2 {
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

    pub(super) fn draw(&self, draw: &Draw, bounds: Rect, view_state: &ViewState) {
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

    fn draw_particle(draw: &Draw, p: &Particle, view_state: &ViewState) {
        let position = p.position();
        let in_inspector = view_state
            .inspector
            .iter()
            .any(|inspector| inspector.contains(position));
        p.draw(draw, in_inspector);
    }
}
