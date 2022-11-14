use core::time::Duration;
use std::time::Instant;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Time {
    pub v: Duration,
}

impl Time {
    pub fn new(v: Duration) -> Self {
        Self { v }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AdvanceableClock {
    start: Instant,
    now: Time,
}

impl AdvanceableClock {
    pub fn new(start: Instant) -> Self {
        Self {
            start,
            now: Time::new(Duration::from_nanos(0)),
        }
    }

    pub fn advance_to_real_now(&mut self) {
        self.now = Time::new(self.start.elapsed());
    }

    pub fn now(&self) -> Time {
        self.now
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Timer {
    start: Option<Time>,
    duration: Duration,
}

impl Timer {
    pub fn new() -> Self {
        Timer::default()
    }

    pub fn new_set(start: Time, duration: Duration) -> Self {
        Self {
            start: Some(start),
            duration,
        }
    }

    // TODO uncomment
    // pub fn elapsed(&self, t: Time) -> Option<Duration> {
    //     self.start.map(|start| t.v - start.v)
    // }

    pub fn is_expired_or_unset(&self, t: Time) -> bool {
        self.start
            .map_or(true, |start| Timer::is_expired(start, self.duration, t))
    }

    pub fn check_expired_then_unset_if_true_or_set_if_unset<F>(
        &mut self,
        t: Time,
        duration: F,
    ) -> bool
    where
        F: FnOnce() -> Duration,
    {
        match self.start {
            Some(start) if Timer::is_expired(start, self.duration, t) => {
                self.unset();
                true
            }
            None => {
                self.set(t, duration());
                false
            }
            _ => false,
        }
    }

    pub fn set(&mut self, start: Time, duration: Duration) {
        self.start = Some(start);
        self.duration = duration;
    }

    pub fn unset(&mut self) {
        self.start = None;
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    fn is_expired(start: Time, duration: Duration, t: Time) -> bool {
        t.v - start.v >= duration
    }
}
