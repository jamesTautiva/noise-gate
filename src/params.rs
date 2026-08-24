use nih_plug::prelude::*;

#[derive(Params)]
pub struct NoiseGateParams {

    #[id = "threshold"]
    pub threshold_db: FloatParam,

    #[id = "attack"]
    pub attack_ms: FloatParam,

    #[id = "hold"]
    pub hold_ms: FloatParam,

    #[id = "release"]
    pub release_ms: FloatParam,

    #[id = "range"]
    pub range_db: FloatParam,

    #[id = "sidechain_hpf"]
    pub sidechain_hpf_hz: FloatParam,

    #[id = "sidechain_lpf"]
    pub sidechain_lpf_hz: FloatParam,

    #[id = "lookahead"]
    pub lookahead_ms: FloatParam,

    #[id = "display_mode"]
    pub display_mode: FloatParam,

    #[id = "pick_detector"]
    pub pick_detector_enabled: BoolParam,

    #[id = "pick_sensitivity"]
    pub pick_sensitivity: FloatParam,

    #[id = "palm_mute_detector"]
    pub palm_mute_enabled: BoolParam,

    #[id = "palm_mute_threshold"]
    pub palm_mute_threshold: FloatParam,

    #[id = "smart_gate"]
    pub smart_gate_enabled: BoolParam,

    #[id = "adaptive_threshold"]
    pub adaptive_threshold_enabled: BoolParam,

    #[id = "adaptive_threshold_speed"]
    pub adaptive_threshold_speed: FloatParam,

    #[id = "noise_classification"]
    pub noise_classification_enabled: BoolParam,

    #[id = "preset"]
    pub preset: FloatParam,
}

impl Default for NoiseGateParams {
    fn default() -> Self {
        Self {
            threshold_db: FloatParam::new(
                "Threshold",
                -30.0,
                FloatRange::Linear { min: -60.0, max: 0.0 },
            )
            .with_unit(" dB"),

            attack_ms: FloatParam::new(
                "Attack",
                0.5,
                FloatRange::Skewed {
                    min: 0.01,
                    max: 50.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" ms"),

            hold_ms: FloatParam::new(
                "Hold",
                10.0,
                FloatRange::Linear { min: 0.0, max: 500.0 },
            )
            .with_unit(" ms"),

            release_ms: FloatParam::new(
                "Release",
                80.0,
                FloatRange::Skewed {
                    min: 5.0,
                    max: 1000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" ms"),

            range_db: FloatParam::new(
                "Range",
                -80.0,
                FloatRange::Linear { min: -80.0, max: 0.0 },
            )
            .with_unit(" dB"),

            sidechain_hpf_hz: FloatParam::new(
                "Sidechain HPF",
                220.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 800.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" Hz"),

            sidechain_lpf_hz: FloatParam::new(
                "Sidechain LPF",
                20000.0,
                FloatRange::Skewed {
                    min: 200.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" Hz"),

            lookahead_ms: FloatParam::new(
                "Lookahead",
                0.0,
                FloatRange::Linear { min: 0.0, max: 10.0 },
            )
            .with_unit(" ms"),

            display_mode: FloatParam::new(
                "Display Mode",
                0.0,
                FloatRange::Linear { min: 0.0, max: 2.0 },
            ),

            pick_detector_enabled: BoolParam::new(
                "Pick Detector",
                false,
            ),

            pick_sensitivity: FloatParam::new(
                "Pick Sensitivity",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            palm_mute_enabled: BoolParam::new(
                "Palm Mute Detector",
                false,
            ),

            palm_mute_threshold: FloatParam::new(
                "Palm Mute Threshold",
                0.3,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            smart_gate_enabled: BoolParam::new(
                "Smart Gate",
                false,
            ),

            adaptive_threshold_enabled: BoolParam::new(
                "Adaptive Threshold",
                false,
            ),

            adaptive_threshold_speed: FloatParam::new(
                "Adaptive Threshold Speed",
                0.1,
                FloatRange::Linear { min: 0.01, max: 1.0 },
            ),

            noise_classification_enabled: BoolParam::new(
                "Noise Classification",
                false,
            ),

            preset: FloatParam::new(
                "Preset",
                0.0,
                FloatRange::Linear { min: 0.0, max: 8.0 },
            ),
        }
    }
}
