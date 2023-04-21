use std::time::{Duration, Instant};

use crate::created::Created;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Stats {
    pub step: u64,
    pub simulated_secs: f32,
    pub time_used_simulating: f32,
    pub real_age: f32,
    pub created: Created,
}

impl Stats {
    pub fn reset(&mut self) {
        *self = Default::default();
    }

    // updates the original & then returns a copy
    pub fn update_real_age(&mut self) -> Self {
        self.real_age = self.created.elapsed().as_secs_f32();
        *self
    }

    pub fn track_step(&mut self, dt: f32, f: impl FnOnce()) {
        let step_start_time = Instant::now();
        f();
        self.step += 1;
        self.simulated_secs += dt;
        self.time_used_simulating += step_start_time.elapsed().as_secs_f32();
        self.real_age = self.created.elapsed().as_secs_f32();
    }

    fn relative_to(&self, baseline: Self) -> Self {
        Self {
            step: self.step - baseline.step,
            simulated_secs: self.simulated_secs - baseline.simulated_secs,
            time_used_simulating: self.time_used_simulating - baseline.time_used_simulating,
            real_age: self.real_age - baseline.real_age,
            created: self.created,
        }
    }

    pub fn lag(&self) -> f32 {
        self.real_age - self.simulated_secs
    }

    pub fn log(&self, baseline: Self) {
        let diff = self.relative_to(baseline);
        let work_per_step = Duration::from_secs_f32(diff.time_used_simulating / diff.step as f32);
        info!(
            "step {:6} lag: {:>4.1}s({:+4.1}), work time: {work_per_step:>10.3?}/step ({:3.0}% simulating)",
            self.step,
            self.lag(),
            diff.lag(),
            self.time_used_simulating / self.real_age * 100.0,
        )
    }
}
