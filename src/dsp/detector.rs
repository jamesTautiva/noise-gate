pub struct PeakDetector {
    is_open: bool,
    open_threshold: f32,
    close_threshold: f32,
}

impl PeakDetector {
    pub fn new() -> Self {
        Self {
            is_open: false,
            open_threshold: 0.0,
            close_threshold: 0.0,
        }
    }

    pub fn set_thresholds(&mut self, open_threshold: f32, close_threshold: f32) {
        self.open_threshold = open_threshold;
        self.close_threshold = close_threshold;
    }

    pub fn process(&mut self, input: f32) -> bool {
        if self.is_open {
            if input < self.close_threshold {
                self.is_open = false;
            }
        } else {
            if input >= self.open_threshold {
                self.is_open = true;
            }
        }
        self.is_open
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.is_open = false;
    }
}

impl Default for PeakDetector {
    fn default() -> Self {
        Self::new()
    }
}
