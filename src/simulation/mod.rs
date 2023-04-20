use nannou::event::Update;
use nannou::geom::Rect;
use nannou::Draw;

use particle::Particle;
use stats::Stats;
use universe::Universe;

use crate::geometry::Point2;
use crate::view_state::ViewState;

mod particle;
mod stats;
mod universe;

#[derive(Debug, Default)]
pub struct Simulation {
    universe: Universe,
    stats: Stats,
}

const TARGET_FPS: f32 = 60.0;
const FRAME_INTERVAL: f32 = 1.0 / TARGET_FPS;
const MAX_UPDATE_DURATION: f32 = FRAME_INTERVAL * 0.5;
const DT: f32 = 0.001;
const CATCHUP_RATE: f32 = 1.1;

impl Simulation {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn reset_stats(&mut self) {
        self.stats = Default::default();
    }

    pub fn update(&mut self, _update: Update) {
        let update_start = self.stats.update_real_age();
        let target_sim_age_secs = (update_start.real_age + FRAME_INTERVAL)
            .min(update_start.simulated_secs + FRAME_INTERVAL * CATCHUP_RATE);

        loop {
            self.stats.track_step(DT, || {
                self.universe.step(DT);
            });

            if self.stats.simulated_secs > target_sim_age_secs
                || (self.stats.real_age - update_start.real_age) > MAX_UPDATE_DURATION
            {
                break;
            }
        }

        static_rate_limit!(Duration::from_secs(1), self.stats.log(update_start));
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
