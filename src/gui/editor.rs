use crate::gui::state::GuiState;
use crate::params::NoiseGateParams;
use crate::presets::Preset;
use nih_plug_egui::egui;
use nih_plug::prelude::*;
use std::sync::Arc;

pub fn create(params: Arc<NoiseGateParams>, gui_state: Arc<GuiState>) -> Option<Box<dyn Editor>> {
    let egui_state = nih_plug_egui::EguiState::from_size(900, 600);
    let user_state = NoiseGateEditorState {
        params: params.clone(),
        gui_state: gui_state.clone(),
    };
    
    nih_plug_egui::create_egui_editor(
        egui_state,
        user_state,
        |ctx, _state| {
            // Build function - called once at startup
            NoiseGateEditor::build_ui(ctx);
        },
        |ctx, setter, state| {
            // Update function - called every frame
            NoiseGateEditor::update_ui(ctx, setter, state);
        },
    )
}

struct NoiseGateEditorState {
    params: Arc<NoiseGateParams>,
    gui_state: Arc<GuiState>,
}

struct NoiseGateEditor;

impl NoiseGateEditor {
    fn build_ui(ctx: &egui::Context) {
        // Professional dark theme
        let mut style = (*ctx.style()).clone();
        style.visuals.panel_fill = egui::Color32::from_rgb(28, 28, 32);
        style.visuals.window_fill = egui::Color32::from_rgb(24, 24, 28);
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(32, 32, 38);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(32, 32, 38);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(42, 42, 48);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(52, 52, 58);
        ctx.set_style(style);
    }

    fn update_ui(ctx: &egui::Context, setter: &ParamSetter, state: &mut NoiseGateEditorState) {
        Self::render_ui(ctx, state.params.clone(), state.gui_state.clone(), setter);
    }

    fn render_ui(
        ctx: &egui::Context,
        params: Arc<NoiseGateParams>,
        gui_state: Arc<GuiState>,
        setter: &ParamSetter,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            
            // Header with plugin name and preset selector
            Self::render_header(ui, params.clone(), setter);
            
            ui.add_space(15.0);
            
            // Waveform visualization
            Self::render_waveform(ui, gui_state.clone());
            
            ui.add_space(15.0);
            
            // Main controls grid
            Self::render_main_controls(ui, params.clone(), setter);
            
            ui.add_space(15.0);
            
            // Advanced controls
            Self::render_advanced_controls(ui, params.clone(), setter);
        });
    }

    fn render_header(ui: &mut egui::Ui, params: Arc<NoiseGateParams>, setter: &ParamSetter) {
        ui.horizontal(|ui| {
            // Plugin name
            ui.heading("DjentCut Noise V.1 DEMO");
            ui.add_space(20.0);
            
            ui.label(egui::RichText::new("Noise Gate")
                .color(egui::Color32::from_rgb(150, 150, 160))
                .size(14.0));
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Preset selector
                ui.label("Preset:");
                let current_preset_idx = params.preset.value() as usize;
                let preset_names = [
                    "Default", "Modern Metal", "Djent", "Thall",
                    "Hardcore", "Deathcore", "Prog Metal", "Bass", "Clean"
                ];
                
                ui.add_space(5.0);
                
                let mut selected = current_preset_idx;
                egui::ComboBox::from_id_salt("preset_selector")
                    .selected_text(preset_names[selected])
                    .width(150.0)
                    .show_ui(ui, |ui| {
                        for (i, &name) in preset_names.iter().enumerate() {
                            if ui.selectable_value(&mut selected, i, name).changed() {
                                let new_preset = Preset::from_index(i as f32);
                                Self::apply_preset(setter, params.clone(), new_preset);
                            }
                        }
                    });
            });
        });
    }

    fn render_waveform(ui: &mut egui::Ui, gui_state: Arc<GuiState>) {
        let available_height = ui.available_height() * 0.35;
        let desired_height = available_height.min(200.0);
        
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Waveform Display")
                .color(egui::Color32::from_rgb(180, 180, 190))
                .size(13.0));
            
            ui.add_space(5.0);
            
            let (response, painter) = ui.allocate_painter(
                egui::vec2(ui.available_width(), desired_height),
                egui::Sense::hover()
            );
            
            let rect = response.rect;
            
            // Background
            painter.rect_filled(
                rect,
                egui::CornerRadius::same(4),
                egui::Color32::from_rgb(20, 20, 24)
            );
            
            // Grid lines
            Self::draw_waveform_grid(&painter, rect);
            
            // Draw waveforms
            Self::draw_waveforms(&painter, rect, gui_state);
        });
    }

    fn draw_waveform_grid(painter: &egui::Painter, rect: egui::Rect) {
        let grid_color = egui::Color32::from_rgba_unmultiplied(80, 80, 90, 30);
        
        // Horizontal lines
        for i in 0..=4 {
            let y = rect.min.y + (rect.height() / 4.0) * i as f32;
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                (1.0, grid_color)
            );
        }
        
        // Vertical lines
        for i in 0..=8 {
            let x = rect.min.x + (rect.width() / 8.0) * i as f32;
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                (1.0, grid_color)
            );
        }
        
        // Center line (zero crossing)
        let center_y = rect.center().y;
        painter.line_segment(
            [egui::pos2(rect.min.x, center_y), egui::pos2(rect.max.x, center_y)],
            (1.5, egui::Color32::from_rgba_unmultiplied(120, 120, 130, 60))
        );
    }

    fn draw_waveforms(painter: &egui::Painter, rect: egui::Rect, gui_state: Arc<GuiState>) {
        let input_samples = gui_state.waveform.input_samples.load();
        let output_samples = gui_state.waveform.output_samples.load();
        let gain_reduction = gui_state.waveform.gain_reduction.load();
        
        let center_y = rect.center().y;
        let amplitude_scale = rect.height() / 2.0 * 0.8;
        let x_step = rect.width() / 512.0;
        
        // Draw input waveform (blue, semi-transparent) with smooth interpolation
        let mut input_points = Vec::with_capacity(1536); // 3x resolution for smoother curves
        for i in 0..1536 {
            let sample_idx = (i as f32 / 3.0) as usize;
            let sample_idx_next = (sample_idx + 1).min(511);
            let t = (i as f32 / 3.0) - sample_idx as f32;
            
            let sample_curr = input_samples[sample_idx];
            let sample_next = input_samples[sample_idx_next];
            let interpolated_sample = sample_curr * (1.0 - t) + sample_next * t;
            
            let x = rect.min.x + i as f32 * (x_step / 3.0);
            let y = center_y - interpolated_sample * amplitude_scale;
            input_points.push(egui::pos2(x, y));
        }
        painter.add(egui::Shape::line(
            input_points,
            (1.5, egui::Color32::from_rgba_unmultiplied(100, 150, 255, 80))
        ));
        
        // Draw output waveform (green) with smooth interpolation
        let mut output_points = Vec::with_capacity(1536);
        for i in 0..1536 {
            let sample_idx = (i as f32 / 3.0) as usize;
            let sample_idx_next = (sample_idx + 1).min(511);
            let t = (i as f32 / 3.0) - sample_idx as f32;
            
            let sample_curr = output_samples[sample_idx];
            let sample_next = output_samples[sample_idx_next];
            let interpolated_sample = sample_curr * (1.0 - t) + sample_next * t;
            
            let x = rect.min.x + i as f32 * (x_step / 3.0);
            let y = center_y - interpolated_sample * amplitude_scale;
            output_points.push(egui::pos2(x, y));
        }
        painter.add(egui::Shape::line(
            output_points,
            (2.0, egui::Color32::from_rgb(100, 200, 100))
        ));
        
        // Draw gain reduction (red, at bottom) with smooth interpolation
        let gr_height = rect.height() * 0.15;
        let gr_base = rect.max.y - 5.0;
        let mut gr_points = Vec::with_capacity(1536);
        for i in 0..1536 {
            let sample_idx = (i as f32 / 3.0) as usize;
            let sample_idx_next = (sample_idx + 1).min(511);
            let t = (i as f32 / 3.0) - sample_idx as f32;
            
            let gr_curr = gain_reduction[sample_idx];
            let gr_next = gain_reduction[sample_idx_next];
            let interpolated_gr = gr_curr * (1.0 - t) + gr_next * t;
            
            let x = rect.min.x + i as f32 * (x_step / 3.0);
            let y = gr_base - (1.0 - interpolated_gr) * gr_height;
            gr_points.push(egui::pos2(x, y));
        }
        painter.add(egui::Shape::line(
            gr_points,
            (2.0, egui::Color32::from_rgb(255, 100, 100))
        ));
        
        // Legend
        Self::draw_waveform_legend(painter, rect);
    }

    fn draw_waveform_legend(painter: &egui::Painter, rect: egui::Rect) {
        let legend_x = rect.min.x + 10.0;
        let legend_y = rect.min.y + 10.0;
        let line_height = 16.0;
        
        // Input legend
        painter.line_segment(
            [egui::pos2(legend_x, legend_y), egui::pos2(legend_x + 20.0, legend_y)],
            (2.0, egui::Color32::from_rgba_unmultiplied(100, 150, 255, 80))
        );
        painter.text(
            egui::pos2(legend_x + 25.0, legend_y),
            egui::Align2::LEFT_CENTER,
            "Input",
            egui::FontId::proportional(11.0),
            egui::Color32::from_rgb(150, 150, 160)
        );
        
        // Output legend
        painter.line_segment(
            [egui::pos2(legend_x, legend_y + line_height), egui::pos2(legend_x + 20.0, legend_y + line_height)],
            (2.0, egui::Color32::from_rgb(100, 200, 100))
        );
        painter.text(
            egui::pos2(legend_x + 25.0, legend_y + line_height),
            egui::Align2::LEFT_CENTER,
            "Output",
            egui::FontId::proportional(11.0),
            egui::Color32::from_rgb(150, 150, 160)
        );
        
        // Gain reduction legend
        painter.line_segment(
            [egui::pos2(legend_x, legend_y + line_height * 2.0), egui::pos2(legend_x + 20.0, legend_y + line_height * 2.0)],
            (2.0, egui::Color32::from_rgb(255, 100, 100))
        );
        painter.text(
            egui::pos2(legend_x + 25.0, legend_y + line_height * 2.0),
            egui::Align2::LEFT_CENTER,
            "Gain Reduction",
            egui::FontId::proportional(11.0),
            egui::Color32::from_rgb(150, 150, 160)
        );
    }

    fn render_main_controls(ui: &mut egui::Ui, params: Arc<NoiseGateParams>, setter: &ParamSetter) {
        egui::Grid::new("main_controls")
            .num_columns(8)
            .spacing([10.0, 15.0])
            .show(ui, |ui| {
                // Threshold
                Self::render_knob(ui, &params.threshold_db, "Threshold", -60.0, 0.0, "dB", setter);
                
                // Attack
                Self::render_knob(ui, &params.attack_ms, "Attack", 0.01, 50.0, "ms", setter);
                
                // Hold
                Self::render_knob(ui, &params.hold_ms, "Hold", 0.0, 500.0, "ms", setter);
                
                // Release
                Self::render_knob(ui, &params.release_ms, "Release", 5.0, 1000.0, "ms", setter);
                
                // Range
                Self::render_knob(ui, &params.range_db, "Range", -80.0, 0.0, "dB", setter);
                
                // Sidechain HPF
                Self::render_knob(ui, &params.sidechain_hpf_hz, "SC HPF", 20.0, 800.0, "Hz", setter);
                
                // Sidechain LPF
                Self::render_knob(ui, &params.sidechain_lpf_hz, "SC LPF", 200.0, 20000.0, "Hz", setter);
                
                // Lookahead
                Self::render_knob(ui, &params.lookahead_ms, "Lookahead", 0.0, 10.0, "ms", setter);
            });
    }

    fn render_advanced_controls(ui: &mut egui::Ui, params: Arc<NoiseGateParams>, setter: &ParamSetter) {
        ui.separator();
        ui.add_space(10.0);
        
        ui.label(egui::RichText::new("Advanced Features")
            .color(egui::Color32::from_rgb(180, 180, 190))
            .size(13.0));
        
        ui.add_space(15.0);
        
        ui.horizontal(|ui| {
            // Adaptive Release
            Self::render_toggle(ui, &params.adaptive_release, "Adaptive Release", setter);
            
            // Pick Detector
            Self::render_toggle(ui, &params.pick_detector_enabled, "Pick Detector", setter);
            
            if params.pick_detector_enabled.value() {
                Self::render_knob_small(ui, &params.pick_sensitivity, "Sens", 0.0, 1.0, "", setter);
            }
            
            // Palm Mute Detector
            Self::render_toggle(ui, &params.palm_mute_enabled, "Palm Mute", setter);
            
            if params.palm_mute_enabled.value() {
                Self::render_knob_small(ui, &params.palm_mute_threshold, "Thresh", 0.0, 1.0, "", setter);
            }
            
            // Smart Gate
            Self::render_toggle(ui, &params.smart_gate_enabled, "Smart Gate", setter);
            
            // Adaptive Threshold
            Self::render_toggle(ui, &params.adaptive_threshold_enabled, "Adaptive Thresh", setter);
            
            if params.adaptive_threshold_enabled.value() {
                Self::render_knob_small(ui, &params.adaptive_threshold_speed, "Speed", 0.01, 1.0, "", setter);
            }
            
            // Noise Classification
            Self::render_toggle(ui, &params.noise_classification_enabled, "Noise Class", setter);
        });
    }

    fn render_knob(ui: &mut egui::Ui, param: &FloatParam, label: &str, min: f32, max: f32, unit: &str, setter: &ParamSetter) {
        ui.vertical(|ui| {
            ui.add_space(5.0);
            
            let value = param.value();
            let normalized = (value - min) / (max - min);
            
            // Draw knob circle
            let knob_size = 50.0;
            let (response, painter) = ui.allocate_painter(
                egui::vec2(knob_size, knob_size),
                egui::Sense::click_and_drag()
            );
            
            let center = response.rect.center();
            let radius = knob_size / 2.0 - 5.0;
            
            // Knob background
            painter.circle_filled(
                center,
                radius,
                egui::Color32::from_rgb(32, 32, 38)
            );
            
            // Knob border
            painter.circle_stroke(
                center,
                radius,
                (2.0, egui::Color32::from_rgb(60, 60, 70))
            );
            
            // Indicator line
            let angle = normalized * std::f32::consts::PI * 1.5 - std::f32::consts::PI * 0.75;
            let indicator_end = egui::pos2(
                center.x + angle.cos() * (radius - 8.0),
                center.y + angle.sin() * (radius - 8.0)
            );
            painter.line_segment(
                [center, indicator_end],
                (2.5, egui::Color32::from_rgb(100, 150, 255))
            );
            
            // Handle drag
            if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if response.rect.contains(pointer_pos) {
                    if ui.input(|i| i.pointer.any_pressed()) {
                        let dy = center.y - pointer_pos.y;
                        let delta = dy / 100.0;
                        let new_value = (value + delta * (max - min)).clamp(min, max);
                        setter.begin_set_parameter(param);
                        setter.set_parameter(param, new_value);
                        setter.end_set_parameter(param);
                    }
                }
            }
            
            // Label
            ui.add_space(5.0);
            ui.label(egui::RichText::new(label)
                .color(egui::Color32::from_rgb(160, 160, 170))
                .size(11.0)
            );
            
            // Value display
            let value_text = if unit.is_empty() {
                format!("{:.2}", value)
            } else {
                format!("{:.1} {}", value, unit)
            };
            ui.label(egui::RichText::new(value_text)
                .color(egui::Color32::from_rgb(200, 200, 210))
                .size(12.0)
            );
        });
    }

    fn render_knob_small(ui: &mut egui::Ui, param: &FloatParam, label: &str, min: f32, max: f32, unit: &str, setter: &ParamSetter) {
        ui.vertical(|ui| {
            let value = param.value();
            let normalized = (value - min) / (max - min);
            
            let knob_size = 40.0;
            let (response, painter) = ui.allocate_painter(
                egui::vec2(knob_size, knob_size),
                egui::Sense::click_and_drag()
            );
            
            let center = response.rect.center();
            let radius = knob_size / 2.0 - 4.0;
            
            painter.circle_filled(
                center,
                radius,
                egui::Color32::from_rgb(32, 32, 38)
            );
            
            painter.circle_stroke(
                center,
                radius,
                (1.5, egui::Color32::from_rgb(60, 60, 70))
            );
            
            let angle = normalized * std::f32::consts::PI * 1.5 - std::f32::consts::PI * 0.75;
            let indicator_end = egui::pos2(
                center.x + angle.cos() * (radius - 6.0),
                center.y + angle.sin() * (radius - 6.0)
            );
            painter.line_segment(
                [center, indicator_end],
                (2.0, egui::Color32::from_rgb(100, 150, 255))
            );
            
            if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if response.rect.contains(pointer_pos) {
                    if ui.input(|i| i.pointer.any_pressed()) {
                        let dy = center.y - pointer_pos.y;
                        let delta = dy / 100.0;
                        let new_value = (value + delta * (max - min)).clamp(min, max);
                        setter.begin_set_parameter(param);
                        setter.set_parameter(param, new_value);
                        setter.end_set_parameter(param);
                    }
                }
            }
            
            ui.label(egui::RichText::new(label)
                .color(egui::Color32::from_rgb(140, 140, 150))
                .size(10.0)
            );
            
            let value_text = if unit.is_empty() {
                format!("{:.2}", value)
            } else {
                format!("{:.1}{}", value, unit)
            };
            ui.label(egui::RichText::new(value_text)
                .color(egui::Color32::from_rgb(180, 180, 190))
                .size(10.0)
            );
        });
    }

    fn render_toggle(ui: &mut egui::Ui, param: &BoolParam, label: &str, setter: &ParamSetter) {
        ui.vertical(|ui| {
            ui.add_space(5.0);
            
            let is_enabled = param.value();
            
            let toggle_size = egui::vec2(50.0, 26.0);
            let (response, painter) = ui.allocate_painter(toggle_size, egui::Sense::click());
            
            let bg_color = if is_enabled {
                egui::Color32::from_rgb(100, 150, 255)
            } else {
                egui::Color32::from_rgb(50, 50, 55)
            };
            
            painter.rect_filled(
                response.rect,
                egui::CornerRadius::same(13),
                bg_color
            );
            
            let circle_radius = 10.0;
            let circle_x = if is_enabled {
                response.rect.max.x - circle_radius - 3.0
            } else {
                response.rect.min.x + circle_radius + 3.0
            };
            let circle_center = egui::pos2(circle_x, response.rect.center().y);
            
            painter.circle_filled(
                circle_center,
                circle_radius,
                egui::Color32::WHITE
            );
            
            if response.clicked() {
                setter.begin_set_parameter(param);
                setter.set_parameter(param, !is_enabled);
                setter.end_set_parameter(param);
            }
            
            ui.add_space(8.0);
            ui.label(egui::RichText::new(label)
                .color(egui::Color32::from_rgb(180, 180, 190))
                .size(12.0)
            );
        });
    }

    fn apply_preset(setter: &ParamSetter, params: Arc<NoiseGateParams>, preset: Preset) {
        let values = preset.get_values();
        
        setter.begin_set_parameter(&params.threshold_db);
        setter.set_parameter(&params.threshold_db, values.threshold_db);
        setter.end_set_parameter(&params.threshold_db);
        
        setter.begin_set_parameter(&params.attack_ms);
        setter.set_parameter(&params.attack_ms, values.attack_ms);
        setter.end_set_parameter(&params.attack_ms);
        
        setter.begin_set_parameter(&params.hold_ms);
        setter.set_parameter(&params.hold_ms, values.hold_ms);
        setter.end_set_parameter(&params.hold_ms);
        
        setter.begin_set_parameter(&params.release_ms);
        setter.set_parameter(&params.release_ms, values.release_ms);
        setter.end_set_parameter(&params.release_ms);
        
        setter.begin_set_parameter(&params.range_db);
        setter.set_parameter(&params.range_db, values.range_db);
        setter.end_set_parameter(&params.range_db);
        
        setter.begin_set_parameter(&params.sidechain_hpf_hz);
        setter.set_parameter(&params.sidechain_hpf_hz, values.sidechain_hpf_hz);
        setter.end_set_parameter(&params.sidechain_hpf_hz);
        
        setter.begin_set_parameter(&params.sidechain_lpf_hz);
        setter.set_parameter(&params.sidechain_lpf_hz, values.sidechain_lpf_hz);
        setter.end_set_parameter(&params.sidechain_lpf_hz);
        
        setter.begin_set_parameter(&params.lookahead_ms);
        setter.set_parameter(&params.lookahead_ms, values.lookahead_ms);
        setter.end_set_parameter(&params.lookahead_ms);
        
        setter.begin_set_parameter(&params.adaptive_release);
        setter.set_parameter(&params.adaptive_release, values.adaptive_release);
        setter.end_set_parameter(&params.adaptive_release);
        
        setter.begin_set_parameter(&params.pick_detector_enabled);
        setter.set_parameter(&params.pick_detector_enabled, values.pick_detector_enabled);
        setter.end_set_parameter(&params.pick_detector_enabled);
        
        setter.begin_set_parameter(&params.pick_sensitivity);
        setter.set_parameter(&params.pick_sensitivity, values.pick_sensitivity);
        setter.end_set_parameter(&params.pick_sensitivity);
        
        setter.begin_set_parameter(&params.palm_mute_enabled);
        setter.set_parameter(&params.palm_mute_enabled, values.palm_mute_enabled);
        setter.end_set_parameter(&params.palm_mute_enabled);
        
        setter.begin_set_parameter(&params.palm_mute_threshold);
        setter.set_parameter(&params.palm_mute_threshold, values.palm_mute_threshold);
        setter.end_set_parameter(&params.palm_mute_threshold);
        
        setter.begin_set_parameter(&params.smart_gate_enabled);
        setter.set_parameter(&params.smart_gate_enabled, values.smart_gate_enabled);
        setter.end_set_parameter(&params.smart_gate_enabled);
        
        setter.begin_set_parameter(&params.adaptive_threshold_enabled);
        setter.set_parameter(&params.adaptive_threshold_enabled, values.adaptive_threshold_enabled);
        setter.end_set_parameter(&params.adaptive_threshold_enabled);
        
        setter.begin_set_parameter(&params.adaptive_threshold_speed);
        setter.set_parameter(&params.adaptive_threshold_speed, values.adaptive_threshold_speed);
        setter.end_set_parameter(&params.adaptive_threshold_speed);
        
        setter.begin_set_parameter(&params.noise_classification_enabled);
        setter.set_parameter(&params.noise_classification_enabled, values.noise_classification_enabled);
        setter.end_set_parameter(&params.noise_classification_enabled);
        
        let preset_idx = preset as i32 as f32;
        setter.begin_set_parameter(&params.preset);
        setter.set_parameter(&params.preset, preset_idx);
        setter.end_set_parameter(&params.preset);
    }
}
