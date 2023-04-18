use itertools::Itertools;
use nannou::color;
use nannou::color::IntoLinSrgba;
use nannou::event::Update;
use nannou::geom::Rect;
use nannou::Draw;

pub use particle::Particle;

use crate::drawing::{alpha, draw_rect};
use crate::geometry::{Point2, Positioned};
use crate::quad_tree::iterator::TreeIterator;
use crate::quad_tree::QuadTree;
use crate::view_state::ViewState;

mod particle;

pub struct Simulation {
    universe: QuadTree<Particle>,
}

impl Simulation {
    const INITIAL_PARTICLE_COUNT: usize = 1000;
    const UNIVERSE_SIZE: f32 = 1e6;

    pub fn new() -> Self {
        let particles = (0..Self::INITIAL_PARTICLE_COUNT).map(|_| Particle::new_random());
        Self {
            universe: Self::make_quad_tree(particles),
        }
    }

    fn make_quad_tree(particles: impl IntoIterator<Item = Particle>) -> QuadTree<Particle> {
        let mut qt = QuadTree::new(Rect::from_w_h(Self::UNIVERSE_SIZE, Self::UNIVERSE_SIZE));
        qt.extend(particles);
        qt
    }

    pub fn add_particle(&mut self, position: Point2) {
        self.universe.insert(Particle::new(position));
    }

    pub fn update(&mut self, view_state: &ViewState, update: Update) {
        // TODO
    }

    pub fn draw(&self, draw: &Draw, bounds: Rect, view_state: &ViewState) {
        let bounded = self.universe.iter().bounded(bounds);
        bounded.clone().nodes().for_each(|node| {
            draw_rect(node.boundary, draw, alpha(color::RED, 0.5));
        });
        bounded.clone().leaves().for_each(|particle| {
            Self::draw_particle(draw, view_state, particle);
        });
    }

    fn draw_particle(draw: &Draw, view_state: &ViewState, particle: &Particle) {
        let position = particle.position;
        let in_inspector = view_state
            .inspector
            .iter()
            .any(|inspector| inspector.contains(particle.position));
        let color = if in_inspector {
            alpha(color::YELLOW, 0.5)
        } else {
            alpha(color::GREEN, 0.5)
        };
        if view_state.draw_particles {
            draw.ellipse()
                .xy(position)
                .w_h(10.0, 10.0)
                .stroke(color::BLACK)
                .color(color);
        }
    }
}
