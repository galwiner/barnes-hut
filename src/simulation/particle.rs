use nannou::geom::{vec2, Point2, Vec2};
use nannou::rand::{thread_rng, Rng};
use nannou::{color, Draw};
use rand_distr::Normal;

use ParticleTag::*;

use crate::drawing::alpha;
use crate::geometry::Positioned;
use crate::simulation::universe::Universe;

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
            velocity: vec2(50.0, 0.0),
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
            position: normal_random_pt2() * 200.0,
            velocity: normal_random_pt2() * 100.0,
            acceleration: normal_random_pt2() * 0.0,
            mass: uniform() * 10.0,
            radius: 3.0,
            tag: Default,
        }
    }
    pub fn update(&mut self, dt: f32, _universe: &Universe) {
        let p = self.position;
        let distance = p.length();
        let g = Universe::G / (distance * distance);
        self.acceleration = g * -p.normalize();
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
    }

    pub fn draw(&self, draw: &Draw, in_inspector: bool) {
        let color = match (self.tag, in_inspector) {
            (Placed, _) => alpha(color::YELLOW, 1.0),
            (_, true) => alpha(color::YELLOW, 0.3),
            _ => alpha(color::GREEN, 0.5),
        };

        draw.ellipse()
            .xy(self.position)
            .w_h(self.radius * 2.0, self.radius * 2.0)
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
