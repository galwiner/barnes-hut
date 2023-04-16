use nannou::color::{rgba, GREEN, RED};
use nannou::geom::{Point2, Vec2};
use nannou::rand::distributions::Distribution;
use nannou::{rand, Draw};
use rand_distr::Normal;

use crate::constants::WINDOW_SIZE;
use crate::drawable::Drawable;
use crate::entity::Entity;
use crate::geometry::Positioned;
use crate::Model;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
    radius: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        let position = Point2::new(x, y);
        let velocity = Vec2::new(1.0, 0.0);
        let acceleration = Vec2::new(0.0, 0.0);
        let mass = 10.0;
        let radius = 0.1;
        Self {
            position,
            velocity,
            acceleration,
            mass,
            radius,
        }
    }
    pub fn new_random() -> Self {
        let normal = Normal::new(0.0, WINDOW_SIZE / 10.0).unwrap();
        let random_sample = || normal.sample(&mut rand::thread_rng());
        let position = Point2::new(random_sample(), random_sample());
        let velocity = Vec2::new(1.0, 0.0);
        let acceleration = Vec2::new(0.0, 0.0);
        let mass = 10.0;
        let radius = 0.1;
        Self {
            position,
            velocity,
            acceleration,
            mass,
            radius,
        }
    }
}

impl Positioned for Particle {
    fn position(&self) -> Point2 {
        self.position
    }
}

impl Drawable for Particle {
    fn draw(&self, draw: &Draw, model: &Model) {
        let color = if model.inspector.contains_point(&self.position) {
            RED
        } else {
            GREEN
        };
        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .w_h(self.radius * 20.0, self.radius * 20.0)
            .stroke(rgba(0.0, 0.0, 0.0, 1.0))
            .color(color);
    }
}

impl Entity for Particle {
    fn update(&mut self) {
        // self.position += self.velocity;
    }
}
