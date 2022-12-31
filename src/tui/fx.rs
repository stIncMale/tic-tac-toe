use core::time::Duration;

use cursive::{
    theme::{ColorStyle, ColorType, PaletteColor},
    Printer,
};

use crate::util::time::{Time, Timer};

pub const HIGHLIGHTED_COLOR_STYLE: ColorStyle = ColorStyle {
    front: ColorType::Palette(PaletteColor::Primary),
    back: ColorType::Palette(PaletteColor::Tertiary),
};

#[derive(Debug)]
pub struct BlinkingAnimation {
    start: Time,
    period: Duration,
    timer: Option<Timer>,
}

impl BlinkingAnimation {
    pub fn new(start: Time, period: Duration, duration_periods: Option<u32>) -> Self {
        Self {
            start,
            period,
            timer: duration_periods
                .map(|duration_periods| period * duration_periods)
                .map(|duration| Timer::new_set(start, duration)),
        }
    }

    pub fn draw<F>(&self, now: Time, printer: &Printer, f: F)
    where
        F: FnOnce(&Printer),
    {
        let elapsed = now.v - self.start.v;
        if self
            .timer
            .as_ref()
            .map_or(false, |timer| timer.is_expired_or_unset(now))
        {
            f(printer);
        } else {
            let even_period = (elapsed.as_nanos() / self.period.as_nanos()) & 1 == 0;
            if even_period {
                printer.with_color(HIGHLIGHTED_COLOR_STYLE, f);
            } else {
                f(printer);
            }
        }
    }
}
