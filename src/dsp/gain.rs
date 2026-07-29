pub struct GainStage {
    range_gain: f32,
}

impl GainStage {
    pub fn new() -> Self {
        Self { range_gain: 0.0 }
    }

    pub fn set_range(&mut self, range_db: f32) {
        self.range_gain = nih_plug::util::db_to_gain(range_db);
    }

    pub fn process(&self, envelope: f32) -> f32 {
        self.range_gain + (1.0 - self.range_gain) * envelope
    }
}

impl Default for GainStage {
    fn default() -> Self {
        Self::new()
    }
}
