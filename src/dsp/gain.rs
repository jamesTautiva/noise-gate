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
    
    pub fn get_range_gain(&self) -> f32 {
        self.range_gain
    }

    pub fn process(&self, envelope: f32) -> f32 {
        // Fórmula corregida para evitar distorsión con ranges negativos grandes
        // Cuando range es -80dB, range_gain es muy pequeño (~0.0001)
        // La ganancia debe interpolarse entre range_gain (gate cerrado) y 1.0 (gate abierto)
        let clamped_envelope = envelope.clamp(0.0, 1.0);
        self.range_gain + (1.0 - self.range_gain) * clamped_envelope
    }
}

impl Default for GainStage {
    fn default() -> Self {
        Self::new()
    }
}
