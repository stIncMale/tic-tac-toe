use core::time::Duration;
use std::time::Instant;

#[derive(Debug)]
pub struct AdvanceableClock {
    t_start: Instant,
    now: Duration,
}

impl AdvanceableClock {
    pub fn new(t_start: Instant) -> Self {
        let mut new = Self {
            t_start,
            now: Duration::from_nanos(0),
        };
        new.advance();
        new
    }

    pub fn advance(&mut self) {
        self.now = self.t_start.elapsed();
    }

    pub fn now(&self) -> Duration {
        self.now
    }
}
