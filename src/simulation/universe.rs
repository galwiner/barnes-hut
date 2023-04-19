use nannou::geom::Rect;
use std::mem;
use nannou::{color, Draw};
use crate::drawing::{alpha, draw_rect};
use crate::geometry::Positioned;
use crate::quad_tree::iterator::TreeIterator;
use crate::quad_tree::QuadTree;
use crate::simulation::particle::Particle;
use crate::view_state::ViewState;

pub(super) struct Universe {
    space: QuadTree<Particle>,
    time_elapsed: f32,
}

impl Universe {
    pub const SIZE: f32 = 1e6;
    pub const G: f32 = 1e3;
    pub const SCALE: f32 = 0.2;

    pub(super) fn new() -> Self {
        Self {
            space: Self::empty_space(),
            time_elapsed: 0.0,
        }
    }

    fn empty_space() -> QuadTree<Particle> {
        QuadTree::new(Rect::from_w_h(Self::SIZE, Self::SIZE))
    }

    pub(super) fn step(&mut self, dt: f32) {
        let new_universe = Self {
            space: Self::empty_space(),
            time_elapsed: self.time_elapsed + dt,
        };
        let old_universe = mem::replace(&mut *self, new_universe);
        for particle in old_universe.space.iter().leaves() {
            let mut particle = particle.clone();
            particle.update(dt, &old_universe);
            self.space.insert(particle);
        }
    }

    pub(super) fn insert(&mut self, particle: Particle) {
        if !self.space.insert(particle) {
            println!("Failed to insert out-of-bounds particle at {:?}", particle.position());
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