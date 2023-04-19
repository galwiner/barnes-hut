use std::time::{Duration, Instant};

use nannou::Draw;
use nannou::event::Update;
use nannou::geom::Rect;

use particle::Particle;
use universe::Universe;

use crate::geometry::Point2;
use crate::view_state::ViewState;

mod particle;
mod universe;

pub struct Simulation {
    universe: Universe,
}

impl Simulation {
    pub const MAX_DT: Duration = Duration::from_millis(50);

    pub fn new() -> Self {
        Self {
            universe: Universe::new(),
        }
    }

    pub fn update(&mut self, update: Update) {
        let dt = update.since_last.min(Self::MAX_DT);
        let step_started_at = Instant::now();
        self.universe.step(dt.as_secs_f32());
        let time_spent = step_started_at.elapsed();
        println!("Since last: {:?} dt: {dt:?} Time spent: {time_spent:?}", update.since_last);
    }

    pub fn add_particle_at(&mut self, position: Point2) {
        self.universe.insert(Particle::new(position));
    }

    pub fn add_random_particles(&mut self, num_particles: usize) {
        for _ in 0..num_particles {
            self.universe.insert(Particle::new_random());
        }
    }

    pub fn draw(&self, draw: &Draw, bounds: Rect, view_state: &ViewState) {
        self.universe.draw(draw, bounds, view_state);
    }
}
