use std::mem;

use nannou::color;
use nannou::event::Update;
use nannou::geom::Rect;
use nannou::Draw;

use particle::Particle;

use crate::drawing::{alpha, draw_rect};
use crate::geometry::{Point2, Positioned};
use crate::quad_tree::iterator::TreeIterator;
use crate::quad_tree::QuadTree;
use crate::view_state::ViewState;

mod particle;

type Universe = QuadTree<Particle>;

pub struct Simulation {
    universe: Universe,
}

impl Simulation {
    pub const INITIAL_PARTICLE_COUNT: usize = 1000;
    pub const UNIVERSE_SIZE: f32 = 1e6;

    pub fn new() -> Self {
        let particles = (0..Self::INITIAL_PARTICLE_COUNT).map(|_| Particle::new_random());
        Self {
            universe: Self::make_quad_tree(particles),
        }
    }

    fn new_universe() -> QuadTree<Particle> {
        QuadTree::new(Rect::from_w_h(Self::UNIVERSE_SIZE, Self::UNIVERSE_SIZE))
    }

    fn make_quad_tree(particles: impl IntoIterator<Item = Particle>) -> QuadTree<Particle> {
        let mut qt = QuadTree::new(Rect::from_w_h(Self::UNIVERSE_SIZE, Self::UNIVERSE_SIZE));
        qt.extend(particles);
        qt
    }

    pub fn add_particle(&mut self, position: Point2) {
        self.universe.insert(Particle::new(position));
    }

    pub fn update(&mut self, update: Update) {
        let old_universe = mem::replace(&mut self.universe, Self::new_universe());
        let particles = old_universe.into_iter();
        particles.for_each(|mut p| {
            p.update(update, &self.universe);
            self.universe.insert(p);
        });
    }

    pub fn draw(&self, draw: &Draw, bounds: Rect, view_state: &ViewState) {
        let bounded = self.universe.iter().bounded(bounds);
        bounded.clone().nodes().for_each(|node| {
            draw_rect(node.boundary(), draw, alpha(color::RED, 0.2));
        });
        if view_state.draw_particles {
            bounded.clone().leaves().for_each(|particle| {
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
