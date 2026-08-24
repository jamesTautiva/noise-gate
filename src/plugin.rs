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
        
        // Crear GUI state inmediatamente para asegurar conexión
        let gui_state = Arc::new(gui::state::GuiState::new());
        let mut processor = NoiseGateProcessor::new(params.clone());
        processor.set_gui_waveform(gui_state.waveform.clone());
        
        Self {
            params,
            processor,
            gui_state: Some(gui_state),
        }
    }
}

impl Plugin for NoiseGate {
    const NAME: &'static str = "DjentCut Noise";
    const VENDOR: &'static str = "ShadowDSP";
    const URL: &'static str = "https://jamestautiva.netlify.app/";
    const EMAIL: &'static str = "tautivamolanoj@gmail.com";
    const VERSION: &'static str = "1.0.0";

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
        // GUI state ya existe en default(), usar directamente
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