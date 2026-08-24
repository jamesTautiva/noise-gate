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

/// Palm Mute Detector - Detecta riffs muteados con filtros reales
pub struct PalmMuteDetector {
    low_freq_energy: f32,
    high_freq_energy: f32,
    threshold: f32,
    smoothing: f32,
    // Filtros internos para separar bandas
    low_freq_filter: crate::dsp::filters::LowPassFilter,
    high_freq_filter: crate::dsp::filters::HighPassFilter,
}

impl Default for PalmMuteDetector {
    fn default() -> Self {
        Self {
            low_freq_energy: 0.0,
            high_freq_energy: 0.0,
            threshold: 2.0, // Ratio típico para palm mute: bajas > 2x altas
            smoothing: 0.95,
            low_freq_filter: crate::dsp::filters::LowPassFilter::new(),
            high_freq_filter: crate::dsp::filters::HighPassFilter::new(),
        }
    }
}

impl PalmMuteDetector {
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }

    pub fn set_sample_rate(&mut self, _sample_rate: f32) {
        // Configurar filtros: LPF a 200Hz para bajas, HPF a 1500Hz para altas
        let low_cutoff = 200.0;
        let high_cutoff = 1500.0;
        
        self.low_freq_filter.set_cutoff(low_cutoff);
        self.high_freq_filter.set_cutoff(high_cutoff);
    }

    pub fn process(&mut self, sample: f32, sample_rate: f32) -> bool {
        // Calcular alpha para filtros
        let low_rc = 1.0 / (2.0 * std::f32::consts::PI * 200.0);
        let low_alpha = 1.0 / (1.0 + sample_rate * low_rc);
        
        let high_rc = 1.0 / (2.0 * std::f32::consts::PI * 1500.0);
        let high_alpha = high_rc / (high_rc + 1.0 / sample_rate);
        
        // Filtrar en bandas separadas
        let low_band = self.low_freq_filter.process(sample, low_alpha);
        let high_band = self.high_freq_filter.process(sample, high_alpha);
        
        // Calcular energía en cada banda
        let low_energy = low_band.abs();
        let high_energy = high_band.abs();
        
        // Suavizar energía
        self.low_freq_energy = self.low_freq_energy * self.smoothing + low_energy * (1.0 - self.smoothing);
        self.high_freq_energy = self.high_freq_energy * self.smoothing + high_energy * (1.0 - self.smoothing);
        
        // Palm mute: mucho más energía en bajas que en altas
        if self.high_freq_energy > 0.001 {
            let ratio = self.low_freq_energy / self.high_freq_energy;
            ratio > self.threshold
        } else {
            false
        }
    }
}

/// Smart Gate - Modo automático que ajusta attack/release dinámicamente
pub struct SmartGate {
    avg_level: f32,
    smoothing: f32,
    dynamic_attack: f32,
    dynamic_release: f32,
}

impl Default for SmartGate {
    fn default() -> Self {
        Self {
            avg_level: 0.0,
            smoothing: 0.99,
            dynamic_attack: 10.0, // ms
            dynamic_release: 100.0, // ms
        }
    }
}

impl SmartGate {
    pub fn process(&mut self, level: f32, _sample_rate: f32) -> (f32, f32) {
        // Calcular nivel promedio del audio
        self.avg_level = self.avg_level * self.smoothing + level * (1.0 - self.smoothing);
        
        // Ajustar attack/release dinámicamente basado en nivel promedio
        // Nivel alto → attack más rápido, release más lento
        // Nivel bajo → attack más lento, release más rápido
        
        let level_factor = self.avg_level.clamp(0.0, 1.0);
        
        // Attack: 5ms a 50ms basado en nivel
        self.dynamic_attack = 5.0 + level_factor * 45.0;
        
        // Release: 50ms a 500ms basado en nivel
        self.dynamic_release = 50.0 + (1.0 - level_factor) * 450.0;
        
        (self.dynamic_attack, self.dynamic_release)
    }
}

/// Adaptive Threshold - Threshold que cambia dinámicamente en dB
pub struct AdaptiveThreshold {
    current_threshold_db: f32,
    speed: f32,
    min_threshold_db: f32,
    max_threshold_db: f32,
}

impl Default for AdaptiveThreshold {
    fn default() -> Self {
        Self {
            current_threshold_db: -30.0,
            speed: 0.1,
            min_threshold_db: -60.0,
            max_threshold_db: -10.0,
        }
    }
}

impl AdaptiveThreshold {
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0.01, 1.0);
    }

    pub fn set_base_threshold(&mut self, base_db: f32) {
        self.current_threshold_db = base_db;
    }

    pub fn process(&mut self, input_level_linear: f32, base_threshold_db: f32) -> f32 {
        // Convertir input level de lineal a dB
        let input_level_db = if input_level_linear > 1e-6 {
            20.0 * input_level_linear.log10()
        } else {
            -100.0
        };
        
        // Ajustar threshold dinámicamente basado en input level
        // Si input es más fuerte que base, subir threshold gradualmente
        // Si input es más débil que base, bajar threshold gradualmente
        
        let target_threshold = if input_level_db > base_threshold_db + 10.0 {
            // Input muy fuerte, subir threshold para evitar gating excesivo
            (base_threshold_db + (input_level_db - base_threshold_db) * 0.5).min(self.max_threshold_db)
        } else if input_level_db < base_threshold_db - 5.0 {
            // Input débil, bajar threshold para capturar señal débil
            (base_threshold_db - 5.0).max(self.min_threshold_db)
        } else {
            // Input en rango normal, mantener base threshold
            base_threshold_db
        };
        
        // Suavizar transición del threshold
        self.current_threshold_db = self.current_threshold_db * (1.0 - self.speed) 
            + target_threshold * self.speed;
        
        // Clamp a rango válido
        self.current_threshold_db = self.current_threshold_db.clamp(self.min_threshold_db, self.max_threshold_db);
        
        self.current_threshold_db
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
