use std::borrow::Borrow;

use nannou::geom::{vec2, Point2, Vec2};
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
    pub fn new(p: Point2) -> Self {
        Self {
            position: p,
            velocity: vec2(1.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            mass: 10.0,
            radius: 0.1,
        }
    }
    pub fn new_random() -> Self {
        let normal = Normal::new(0.0, 100.0).unwrap();
        let random_sample = || normal.sample(&mut rand::thread_rng());
        let position = Point2::new(random_sample(), random_sample());
        let velocity = vec2(1.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
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

impl Positioned for &Particle {
    fn position(&self) -> Point2 {
        self.position
    }
}
