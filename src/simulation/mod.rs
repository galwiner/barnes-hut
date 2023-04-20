use std::time::{Duration, Instant};

use nannou::event::Update;
use nannou::geom::Rect;
use nannou::Draw;

use particle::Particle;
use universe::Universe;

use crate::geometry::Point2;
use crate::view_state::ViewState;

mod particle;
mod universe;

#[derive(Debug, Default)]
pub struct Simulation {
    universe: Universe,
    step: u64,
    simulated_time_elapsed: Duration,
    time_used_simulating: Duration,
    last_logged_at: Option<Instant>,
}

impl Simulation {
    const LOG_INTERVAL_SECS: f32 = 1.0;
    const TARGET_FPS: f32 = 60.0;
    const MAX_SECS_PER_UPDATE: f32 = 1.0 / Self::TARGET_FPS;
    const DT_PER_STEP: f32 = 0.01;

    pub fn new() -> Self {
        Self {
            universe: Universe::new(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, update: Update) {
        let time_budget = Duration::from_secs_f32(Self::MAX_SECS_PER_UPDATE);
        let update_time = Instant::now();
        let dt_duration = Duration::from_secs_f32(Self::DT_PER_STEP);

        loop {
            let step_started_at = Instant::now();
            self.universe.step(Self::DT_PER_STEP);
            self.simulated_time_elapsed += dt_duration;
            self.time_used_simulating += step_started_at.elapsed();
            self.step += 1;
            if update_time.elapsed() > time_budget
                || self.simulated_time_elapsed > update.since_start
            {
                break;
            }
        }

        self.log_update(update);
    }

    fn log_update(&mut self, update: Update) {
        let last_logged_at = self.last_logged_at.get_or_insert_with(Instant::now);
        if last_logged_at.elapsed().as_secs_f32() > Self::LOG_INTERVAL_SECS || self.step == 0 {
            let time_percent = |d: Duration| {
                format!(
                    "{:6.2}%",
                    d.as_secs_f32() / update.since_start.as_secs_f32() * 100.0
                )
            };
            println!(
                "s: {:6}{:>8.2?}, lag: {:>8.2?}, work time: {}",
                self.step,
                update.since_start,
                update.since_start - self.simulated_time_elapsed.min(update.since_start),
                time_percent(self.time_used_simulating),
            );
            self.last_logged_at = Some(Instant::now());
        }
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
