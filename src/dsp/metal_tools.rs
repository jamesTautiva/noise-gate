/// Herramientas especializadas para guitarra metal
/// Incluye: Pick Detector, Palm Mute Detector, Smart Gate, Adaptive Threshold, Noise Classification

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoiseType {
    Silence,
    Noise,
    String,
    PalmMute,
    Harmonic,
}

/// Pick Detector - Detecta ataques de púa rápidos
pub struct PickDetector {
    prev_sample: f32,
    attack_threshold: f32,
}

impl Default for PickDetector {
    fn default() -> Self {
        Self {
            prev_sample: 0.0,
            attack_threshold: 0.3,
        }
    }
}

impl PickDetector {
    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        // Sensitivity 0.0-1.0, invertido para threshold (menor sensibilidad = threshold más alto)
        self.attack_threshold = 1.0 - sensitivity;
    }

    pub fn process(&mut self, sample: f32) -> bool {
        let delta = (sample - self.prev_sample).abs();
        self.prev_sample = sample;
        
        // Detectar ataque rápido
        delta > self.attack_threshold
    }
}

/// Palm Mute Detector - Detecta riffs muteados
pub struct PalmMuteDetector {
    low_freq_energy: f32,
    high_freq_energy: f32,
    threshold: f32,
    smoothing: f32,
}

impl Default for PalmMuteDetector {
    fn default() -> Self {
        Self {
            low_freq_energy: 0.0,
            high_freq_energy: 0.0,
            threshold: 0.3,
            smoothing: 0.95,
        }
    }
}

impl PalmMuteDetector {
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }

    pub fn process(&mut self, sample: f32) -> bool {
        // Palm mute tiene más energía en bajas frecuencias que en altas
        // Esta es una simplificación - en producción usar filtros reales
        
        let low_energy = sample.abs(); // Simplificación: asume sample ya filtrado
        let high_energy = (sample * 0.5).abs(); // Simplificación para altas
        
        self.low_freq_energy = self.low_freq_energy * self.smoothing + low_energy * (1.0 - self.smoothing);
        self.high_freq_energy = self.high_freq_energy * self.smoothing + high_energy * (1.0 - self.smoothing);
        
        if self.high_freq_energy > 0.001 {
            let ratio = self.low_freq_energy / self.high_freq_energy;
            ratio > self.threshold
        } else {
            false
        }
    }
}

/// Smart Gate - Modo automático que ajusta parámetros
pub struct SmartGate {
    avg_level: f32,
    target_threshold: f32,
    smoothing: f32,
}

impl Default for SmartGate {
    fn default() -> Self {
        Self {
            avg_level: 0.0,
            target_threshold: -30.0,
            smoothing: 0.99,
        }
    }
}

impl SmartGate {
    pub fn process(&mut self, level: f32) -> f32 {
        // Calcular nivel promedio del audio
        self.avg_level = self.avg_level * self.smoothing + level * (1.0 - self.smoothing);
        
        // Threshold automático basado en nivel promedio
        let db_level = 20.0 * self.avg_level.max(0.0001).log10();
        self.target_threshold = db_level - 15.0; // 15dB por debajo del promedio
        
        self.target_threshold.clamp(-60.0, -10.0)
    }
}

/// Adaptive Threshold - Threshold que cambia dinámicamente
pub struct AdaptiveThreshold {
    current_threshold: f32,
    target_threshold: f32,
    speed: f32,
    min_threshold: f32,
    max_threshold: f32,
}

impl Default for AdaptiveThreshold {
    fn default() -> Self {
        Self {
            current_threshold: -30.0,
            target_threshold: -30.0,
            speed: 0.1,
            min_threshold: -60.0,
            max_threshold: 0.0,
        }
    }
}

impl AdaptiveThreshold {
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn process(&mut self, input_level: f32, base_threshold: f32) -> f32 {
        let db_input = 20.0 * input_level.max(0.0001).log10();
        
        // Ajustar target threshold basado en input
        if db_input > base_threshold + 10.0 {
            // Input fuerte, subir threshold
            self.target_threshold = (self.target_threshold + 1.0).min(self.max_threshold);
        } else if db_input < base_threshold - 5.0 {
            // Input débil, bajar threshold
            self.target_threshold = (self.target_threshold - 0.5).max(self.min_threshold);
        }
        
        // Suavizar transición
        self.current_threshold = self.current_threshold * (1.0 - self.speed) 
            + self.target_threshold * self.speed;
        
        self.current_threshold
    }
}

/// Noise Classification - Clasifica el tipo de señal
pub struct NoiseClassifier {
    rms_buffer: [f32; 32],
    buffer_index: usize,
    zero_crossings: usize,
    prev_sample: f32,
}

impl Default for NoiseClassifier {
    fn default() -> Self {
        Self {
            rms_buffer: [0.0; 32],
            buffer_index: 0,
            zero_crossings: 0,
            prev_sample: 0.0,
        }
    }
}

impl NoiseClassifier {
    pub fn process(&mut self, sample: f32) -> NoiseType {
        // Contar zero crossings
        if (sample > 0.0 && self.prev_sample <= 0.0) || (sample < 0.0 && self.prev_sample >= 0.0) {
            self.zero_crossings += 1;
        }
        self.prev_sample = sample;
        
        // Almacenar en buffer RMS
        self.rms_buffer[self.buffer_index] = sample.abs();
        self.buffer_index = (self.buffer_index + 1) % 32;
        
        // Calcular RMS
        let rms: f32 = self.rms_buffer.iter().map(|&x| x * x).sum::<f32>() / 32.0;
        let rms = rms.sqrt();
        
        // Clasificar basado en RMS y zero crossings
        if rms < 0.01 {
            self.zero_crossings = 0;
            NoiseType::Silence
        } else if rms < 0.05 {
            self.zero_crossings = 0;
            NoiseType::Noise
        } else if self.zero_crossings > 20 {
            self.zero_crossings = 0;
            NoiseType::Harmonic
        } else if rms > 0.3 && self.zero_crossings < 10 {
            self.zero_crossings = 0;
            NoiseType::PalmMute
        } else {
            self.zero_crossings = 0;
            NoiseType::String
        }
    }
}
