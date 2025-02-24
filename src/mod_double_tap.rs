use std::time::{Duration, Instant};

pub enum TapMode {
    QuickTap,
    Hold,
    DoubleTap,
}

pub struct DoubleTap {
    press_time: Instant,
    last_press_time: Option<Instant>,
    pressed: bool,
    threshold: Duration,
    double_tap_threshold: Duration,
    pub locked: bool,
}

impl DoubleTap {
    pub fn new(threshold_ms: u64) -> Self {
        Self {
            press_time: Instant::now(),
            last_press_time: None,
            pressed: false,
            threshold: Duration::from_millis(threshold_ms),
            double_tap_threshold: Duration::from_millis(300), // Configurable double tap window
            locked: false,
        }
    }

    pub fn init(&mut self) {
        if !self.pressed {
            self.last_press_time = Some(self.press_time);
            self.press_time = Instant::now();
            self.pressed = true;
        }
    }

    pub fn is(&mut self) -> TapMode {
        self.pressed = false;
        let elapsed = self.press_time.elapsed();

        // Check for double tap
        if let Some(last_press) = self.last_press_time {
            let time_between_presses = self.press_time - last_press;
            if time_between_presses <= self.double_tap_threshold {
                return TapMode::DoubleTap;
            }
        }

        // Normal hold check
        if elapsed < self.threshold {
            // Quick tap
            TapMode::QuickTap
        } else {
            // Hold duration greater than threshold
            TapMode::Hold
        }
    }
}
