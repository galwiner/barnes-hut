use std::time::{Duration, Instant};

use crate::created::Created;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Stats {
    pub frames: u64,
    pub steps: u64,
    pub simulated_secs: f32,
    pub time_used_simulating: f32,
    pub real_age: f32,
    pub created: Created,
}

impl Stats {
    /// Updates the stats to reflect the start of a new frame and returns a copy
    pub fn start_update(&mut self) -> Self {
        self.real_age = self.created.elapsed().as_secs_f32();
        *self
    }

    pub fn end_update(&mut self) {
        self.frames += 1;
    }

    pub fn track_step(&mut self, dt: f32, f: impl FnOnce()) {
        let step_start_time = Instant::now();
        f();
        self.steps += 1;
        self.simulated_secs += dt;
        self.time_used_simulating += step_start_time.elapsed().as_secs_f32();
        self.real_age = self.created.elapsed().as_secs_f32();
    }

    pub fn relative_to(&self, baseline: Self) -> Self {
        Self {
            frames: self.frames - baseline.frames,
            steps: self.steps - baseline.steps,
            simulated_secs: self.simulated_secs - baseline.simulated_secs,
            time_used_simulating: self.time_used_simulating - baseline.time_used_simulating,
            real_age: self.real_age - baseline.real_age,
            created: self.created,
        }
    }

    pub fn lag(&self) -> f32 {
        self.real_age - self.simulated_secs
    }

    pub fn mean_work_per_step(&self) -> Option<f32> {
        if self.steps == 0 {
            None
        } else {
            Some(self.time_used_simulating / self.steps as f32)
        }
    }

    pub fn log(&self, last_logged: Self) {
        let delta = self.relative_to(last_logged);

        let steps = self.steps;
        assert!(self.simulated_secs >= 0.0);
        let sim_time = Duration::from_secs_f32(self.simulated_secs);
        let real_time = Duration::from_secs_f32(self.real_age);
        let sim_percent = at_most!(self.simulated_secs / self.real_age * 100.0, 100.0);
        let d_lag_ms = delta.lag();
        let work_per_step = Duration::from_secs_f32(delta.mean_work_per_step().unwrap_or(0.0));
        let fps = delta.frames as f32 / delta.real_age;
        let hz = delta.steps as f32 / delta.real_age;
        info!(target:"barnes_hut::sim",
            "step {steps:6} simulated {sim_time:>6.1?} in {real_time:>6.1?} ({sim_percent:3.0}%), \
             lag{d_lag_ms:>+6.3?}ms, \
             spent:{work_per_step:>9.3?}/step {fps:3.0}FPS, {hz:3.0}Hz"
        );
    }
}
