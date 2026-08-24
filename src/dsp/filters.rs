use crate::dsp::utils::flush_to_zero;

pub struct HighPassFilter {
    prev_input: f32,
    prev_output: f32,
    cutoff: f32,
}

impl HighPassFilter {
    pub fn new() -> Self {
        Self {
            prev_input: 0.0,
            prev_output: 0.0,
            cutoff: 0.0,
        }
    }
    
    pub fn get_cutoff(&self) -> f32 {
        self.cutoff
    }

    pub fn process(&mut self, input: f32, alpha: f32) -> f32 {
        // Filtro pasa-altas RC de primer orden correcto
        // y[n] = alpha * (x[n] - x[n-1] + y[n-1])
        let output = alpha * (input - self.prev_input + self.prev_output);
        self.prev_input = flush_to_zero(input);
        self.prev_output = flush_to_zero(output);
        flush_to_zero(output)
    }
    
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.prev_input = 0.0;
        self.prev_output = 0.0;
    }
}

impl Default for HighPassFilter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LowPassFilter {
    prev_output: f32,
    cutoff: f32,
}

impl LowPassFilter {
    pub fn new() -> Self {
        Self {
            prev_output: 0.0,
            cutoff: 0.0,
        }
    }
    
    pub fn get_cutoff(&self) -> f32 {
        self.cutoff
    }

    pub fn process(&mut self, input: f32, alpha: f32) -> f32 {
        // Filtro pasa-bajas RC de primer orden correcto
        // y[n] = alpha * x[n] + (1 - alpha) * y[n-1]
        let output = alpha * input + (1.0 - alpha) * self.prev_output;
        self.prev_output = flush_to_zero(output);
        flush_to_zero(output)
    }
    
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.prev_output = 0.0;
    }
}

impl Default for LowPassFilter {
    fn default() -> Self {
        Self::new()
    }
}
