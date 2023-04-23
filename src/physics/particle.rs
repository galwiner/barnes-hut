use nannou::prelude::*;
use nannou::rand::{thread_rng, Rng};
use rand_distr::Normal;

use ParticleType::*;

use crate::drawing::alpha;
use crate::view_state::ViewState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParticleType {
    Default,
    Placed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Particle {
    tag: ParticleType,
    pub mass: f32,
    pub position: Point2,
    velocity: Vec2,
    radius: f32,
}

impl Particle {
    pub fn new(position: Point2) -> Self {
        Self {
            position,
            velocity: vec2(0.0, 0.0),
            mass: 1000.0,
            radius: 5.0,
            tag: Placed,
        }
    }

    pub fn new_random() -> Self {
        let normal_dist = Normal::new(0.0, 1.0).unwrap();
        let uniform = || thread_rng().gen::<f32>();
        let normal_random_pt2 = || {
            Point2::new(
                thread_rng().sample(normal_dist),
                thread_rng().sample(normal_dist),
            )
        };

        let size = 0.5 + (uniform() * 3.0);

        let position = normal_random_pt2() * 200.0;
        let velocity = position.rotate(-PI / 2.0).normalize() * position.length().powf(0.5) * 5.0;

        Self {
            position,
            velocity,
            mass: size * size * size,
            radius: size,
            tag: Default,
        }
    }

    pub fn update(&mut self, dt: f32, acceleration: Vec2) {
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
    }

    pub fn draw(&self, draw: &Draw, view_state: &ViewState) {
        let color = match (self.tag, view_state.is_inspecting(self.position)) {
            (Placed, _) => alpha(TURQUOISE, 0.5),
            (_, true) => alpha(YELLOW, 0.2),
            _ => alpha(GREEN, 0.2),
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
