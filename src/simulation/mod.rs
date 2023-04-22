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
    stats_at_prev_update_start: Stats,
    stats_last_logged: Stats,
}

const TARGET_FPS: f32 = 60.0;
const TARGET_FRAME_INTERVAL: f32 = 1.0 / TARGET_FPS;
const MAX_DT: f32 = 0.01;
const CATCHUP_RATE: f32 = 1.1;

impl<M: Model> Simulation<M> {
    pub fn new(model: M) -> Self {
        Self {
            model,
            stats: Stats::default(),
            stats_at_prev_update_start: Stats::default(),
            stats_last_logged: Stats::default(),
        }
    }

    pub fn update(&mut self) {
        let update_start = self.stats.start_update();

        let frame_interval = update_start.real_age - self.stats_at_prev_update_start.real_age;
        let prev_frame_overrun = at_least!(frame_interval - TARGET_FRAME_INTERVAL, 0.0);

        let real_age_deadline = update_start.real_age + TARGET_FRAME_INTERVAL - prev_frame_overrun;
        let target_sim_age_secs = at_most!(
            update_start.real_age + frame_interval,
            update_start.simulated_secs + frame_interval * CATCHUP_RATE
        );

        loop {
            let since_last_update = self.stats.relative_to(self.stats_at_prev_update_start);

            let dt = match since_last_update.mean_work_per_step() {
                Some(estimated_step_cost) => {
                    let sim_time_remaining = target_sim_age_secs - self.stats.simulated_secs;
                    let update_time_remaining = real_age_deadline - self.stats.real_age;

                    let expected_steps_remaining =
                        at_least!((update_time_remaining / estimated_step_cost).trunc(), 1.0);
                    at_most!(sim_time_remaining / expected_steps_remaining, MAX_DT)
                }
                None => 0.0, // Avoid unnecessary quantization error due to overestimating step cost
            };

            self.step(dt);

            if self.stats.simulated_secs > target_sim_age_secs
                || self.stats.real_age > real_age_deadline
            {
                break;
            }
        }

        self.stats_at_prev_update_start = update_start;
        self.stats.end_update();

        static_rate_limit!(Duration::from_secs(1), {
            self.stats.log(self.stats_last_logged);
            self.stats_last_logged = self.stats;
        });
    }

    fn step(&mut self, dt: f32) {
        self.stats.track_step(dt, || {
            self.model.step(dt);
        });
    }

    pub fn reset_stats(&mut self) {
        self.stats = Stats::default();
        self.stats_at_prev_update_start = Stats::default();
        self.stats_last_logged = Stats::default();
    }
}
