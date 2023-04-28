use instant::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Created {
    pub at: Instant,
}

impl Created {
    pub fn elapsed(&self) -> Duration {
        self.at.elapsed()
    }
}

impl Default for Created {
    fn default() -> Self {
        Self { at: Instant::now() }
    }
}
