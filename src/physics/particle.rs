use nannou::geom::{vec2, Point2, Vec2};
use nannou::rand::{thread_rng, Rng};
use nannou::{color, Draw};
use rand_distr::Normal;

use ParticleTag::*;

use crate::drawing::alpha;
use crate::quad_tree::Positioned;
use crate::view_state::ViewState;

use super::Universe;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParticleTag {
    Default,
    Placed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Particle {
    tag: ParticleTag,
    pub mass: f32,
    pub position: Point2,
    velocity: Vec2,
    radius: f32,
}

impl Particle {
    pub fn new(position: Point2) -> Self {
        Self {
            position,
            velocity: vec2(50.0, 0.0),
            mass: 10.0,
            radius: 5.0,
            tag: Placed,
        }
    }

    pub fn new_random() -> Self {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let uniform = || thread_rng().gen::<f32>();
        let normal_random_pt2 =
            || Point2::new(thread_rng().sample(normal), thread_rng().sample(normal));

        Self {
            position: normal_random_pt2() * 200.0,
            velocity: normal_random_pt2() * 50.0,
            mass: uniform() * 10.0,
            radius: 3.0,
            tag: Default,
        }
    }

    pub fn update(&mut self, dt: f32, universe: &Universe) {
        let acceleration = universe.net_force_on(self) / self.mass;
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
    }

    pub fn draw(&self, draw: &Draw, view_state: &ViewState) {
        let color = match (self.tag, view_state.is_inspecting(self.position)) {
            (Placed, _) => alpha(color::YELLOW, 1.0),
            (_, true) => alpha(color::YELLOW, 0.3),
            _ => alpha(color::GREEN, 0.5),
        };
        let diameter = self.radius * 2.0;
        if diameter > view_state.min_universe_feature_size() {
            draw.ellipse()
                .x_y(self.position.x, self.position.y)
                .w_h(diameter, diameter)
                .color(color);
        } else {
            let diameter = view_state.min_universe_feature_size();
            draw.rect()
                .x_y(self.position.x, self.position.y)
                .w_h(diameter, diameter)
                .color(color);
        }
    }
}

impl Positioned for Particle {
    fn position(&self) -> Point2 {
        self.position
    }
}

impl Positioned for &Particle {
    fn position(&self) -> Point2 {
        self.position
    }
}
