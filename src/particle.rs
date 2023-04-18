use nannou::geom::{Point2, Vec2};
use nannou::rand;
use nannou::rand::distributions::Distribution;
use rand_distr::Normal;

use crate::geometry::Positioned;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub position: Point2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub radius: f32,
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
        let normal = Normal::new(0.0, 100.0).unwrap();
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
