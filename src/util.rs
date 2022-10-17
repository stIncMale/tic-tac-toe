use core::time::Duration;
use std::time::Instant;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AdvanceableClockTime {
    pub v: Duration,
}

impl AdvanceableClockTime {
    pub fn new(v: Duration) -> Self {
        Self { v }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AdvanceableClock {
    start: Instant,
    now: AdvanceableClockTime,
}

impl AdvanceableClock {
    pub fn new(start: Instant) -> Self {
        let mut new = Self {
            start,
            now: AdvanceableClockTime::new(Duration::from_nanos(0)),
        };
        new.advance();
        new
    }

    pub fn advance(&mut self) {
        self.now = AdvanceableClockTime::new(self.start.elapsed());
    }

    pub fn now(&self) -> AdvanceableClockTime {
        self.now
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Timer {
    start: Option<AdvanceableClockTime>,
    duration: Duration,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: None,
            ..Default::default()
        }
    }

    pub fn new_set(start: AdvanceableClockTime, duration: Duration) -> Self {
        Self {
            start: Some(start),
            duration,
        }
    }

    pub fn is_expired_or_unset(&self, t: AdvanceableClockTime) -> bool {
        self.start
            .map_or(true, |start| Timer::is_expired(start, self.duration, t))
    }

    pub fn check_expired_then_unset_if_true_or_set_if_unset<F>(
        &mut self,
        t: AdvanceableClockTime,
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

    pub fn set(&mut self, start: AdvanceableClockTime, duration: Duration) {
        self.start = Some(start);
        self.duration = duration;
    }

    pub fn unset(&mut self) {
        self.start = None;
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    fn is_expired(
        start: AdvanceableClockTime,
        duration: Duration,
        t: AdvanceableClockTime,
    ) -> bool {
        t.v - start.v >= duration
    }
}