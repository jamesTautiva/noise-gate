use crate::params::NoiseGateParams;
use crate::dsp::NoiseGateProcessor;
use crate::gui;
use nih_plug::prelude::*;
use std::sync::Arc;

pub struct NoiseGate {
    params: Arc<NoiseGateParams>,
    processor: NoiseGateProcessor,
    gui_state: Option<Arc<gui::state::GuiState>>,
}

impl Default for NoiseGate {
    fn default() -> Self {
        let params = Arc::new(NoiseGateParams::default());
        let processor = NoiseGateProcessor::new(params.clone());
        
        Self {
            params,
            processor,
            gui_state: None,
        }
    }
}

impl Plugin for NoiseGate {
    const NAME: &'static str = "DjentCut Noise";
    const VENDOR: &'static str = "James Tautiva";
    const URL: &'static str = "https://example.com";
    const EMAIL: &'static str = "tu@email.com";
    const VERSION: &'static str = "0.1.0";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        // Create GUI state if it doesn't exist
        if self.gui_state.is_none() {
            let gui_state = Arc::new(gui::state::GuiState::new());
            self.processor.set_gui_waveform(gui_state.waveform.clone());
            self.gui_state = Some(gui_state);
        }
        
        gui::create(self.params.clone(), self.gui_state.clone().unwrap())
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let sample_rate = context.transport().sample_rate;
        self.processor.process(buffer, sample_rate);
        ProcessStatus::Normal
    }
}

impl Vst3Plugin for NoiseGate {
    const VST3_CLASS_ID: [u8; 16] = *b"DjentCutPlugX012";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}