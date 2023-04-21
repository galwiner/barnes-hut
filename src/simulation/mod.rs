use std::default::Default;
use std::time::Duration;

use stats::Stats;

mod stats;

pub trait Model: Sized {
    fn step(&mut self, dt: f32);
}

#[derive(Debug, Default)]
pub struct Simulation<M> {
    pub model: M,
    stats: Stats,
    last_logged_stats: Stats,
}

const TARGET_FPS: f32 = 60.0;
const FRAME_INTERVAL: f32 = 1.0 / TARGET_FPS;
const MAX_UPDATE_DURATION: f32 = FRAME_INTERVAL * 0.5;
const DT: f32 = 0.001;
const CATCHUP_RATE: f32 = 1.1;

impl<M: Model> Simulation<M> {
    pub fn new(model: M) -> Self {
        Self {
            model,
            stats: Stats::default(),
            last_logged_stats: Stats::default(),
        }
    }

    pub fn update(&mut self) {
        let update_start = self.stats.start_update();
        let target_sim_age_secs = at_least!(
            (update_start.real_age + FRAME_INTERVAL),
            update_start.simulated_secs + FRAME_INTERVAL * CATCHUP_RATE
        );

        loop {
            self.stats.track_step(DT, || {
                self.model.step(DT);
            });

            if self.stats.simulated_secs > target_sim_age_secs
                || (self.stats.real_age - update_start.real_age) > MAX_UPDATE_DURATION
            {
                break;
            }
        }

        self.stats.end_update();

        static_rate_limit!(Duration::from_secs(1), {
            self.stats.log(self.last_logged_stats);
            self.last_logged_stats = self.stats;
        });
    }

    pub fn reset_stats(&mut self) {
        self.stats = Stats::default();
        self.last_logged_stats = Stats::default();
    }
}
