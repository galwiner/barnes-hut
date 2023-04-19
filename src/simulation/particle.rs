use nannou::event::Update;
use nannou::geom::{vec2, Point2, Vec2};
use nannou::rand::{thread_rng, Rng};
use nannou::{color, Draw};
use rand_distr::Normal;

use ParticleTag::*;

use crate::drawing::alpha;
use crate::geometry::Positioned;
use crate::simulation::{Simulation, Universe};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParticleTag {
    Default,
    Placed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Particle {
    tag: ParticleTag,
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
    radius: f32,
}

impl Particle {
    pub fn new(position: Point2) -> Self {
        Self {
            position,
            velocity: vec2(10.0, 0.0),
            acceleration: vec2(0.0, 0.0),
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
            position: normal_random_pt2() * 100.0,
            velocity: normal_random_pt2() * 10.0,
            acceleration: normal_random_pt2() * 0.0,
            mass: uniform() * 10.0,
            radius: 2.0,
            tag: Default,
        }
    }
    pub fn update(&mut self, update: Update, _universe: &Universe) {
        const SIM_SPEED: f32 = 10.0;
        const G: f32 = 1e3;
        const SCALE: f32 = 0.5;

        let p = self.position;
        let distance = p.length() * SCALE;
        let gravity = G * self.mass / (distance * distance);
        self.acceleration = (-gravity * p.normalize()).clamp_length_max(10.0);
        let dt = update.since_last.as_secs_f32() * SIM_SPEED;
        self.velocity += (self.acceleration * dt).clamp_length_max(10.0);
        self.position += self.velocity * dt;
        self.position.clamp_length_max(Simulation::UNIVERSE_SIZE);
    }

    pub fn draw(&self, draw: &Draw, in_inspector: bool) {
        let color = match (self.tag, in_inspector) {
            (Placed, _) => alpha(color::YELLOW, 1.0),
            (_, true) => alpha(color::YELLOW, 0.3),
            _ => alpha(color::GREEN, 0.3),
        };

        draw.ellipse()
            .xy(self.position)
            .w_h(self.radius * 2.0, self.radius * 2.0)
            .stroke(alpha(color::BLACK, 0.5))
            .stroke_weight(0.1)
            .color(color);
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
