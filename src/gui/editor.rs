use crate::gui::state::GuiState;
use crate::params::NoiseGateParams;
use crate::presets::Preset;
use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, Color32, CornerRadius, Pos2, Rect, Vec2};
use std::sync::Arc;

pub fn create(params: Arc<NoiseGateParams>, gui_state: Arc<GuiState>) -> Option<Box<dyn Editor>> {
    let egui_state = nih_plug_egui::EguiState::from_size(800, 500);
    let user_state = NoiseGateEditorState { params, gui_state };

    nih_plug_egui::create_egui_editor(
        egui_state,
        user_state,
        |ctx, _state| {
            // Build function - inicialización de estilos
            let mut style = (*ctx.style()).clone();
            style.visuals.dark_mode = true;
            style.visuals.panel_fill = Color32::from_rgb(18, 19, 22);
            ctx.set_style(style);
        },
        |ctx, setter, state| {
            NoiseGateEditor::render_ui(ctx, setter, state);
        },
    )
}

struct NoiseGateEditorState {
    params: Arc<NoiseGateParams>,
    gui_state: Arc<GuiState>,
}

struct NoiseGateEditor;

impl NoiseGateEditor {
    fn render_ui(ctx: &egui::Context, setter: &ParamSetter, state: &NoiseGateEditorState) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(12.0, 12.0);

            // 1. Header & Preset Manager
            Self::render_header(ui, &state.params, setter);

            ui.add_space(4.0);

            // 2. Central Display (Meters + GR Visualizer)
            Self::render_meter_bridge(ui, &state.gui_state);

            ui.add_space(6.0);

            // 3. Controles Principales (Knobs Pro)
            Self::render_main_controls(ui, &state.params, setter);

            ui.add_space(6.0);

            // 4. Módulos Avanzados (Filter & Smart Toggles)
            Self::render_advanced_section(ui, &state.params, setter);
        });
    }

    fn render_header(ui: &mut egui::Ui, params: &Arc<NoiseGateParams>, setter: &ParamSetter) {
        ui.horizontal(|ui| {
            ui.heading(
                egui::RichText::new("DJENTCUT")
                    .color(Color32::from_rgb(237, 237, 237))
                    .size(18.0)
                    .strong(),
            );
            ui.label(
                egui::RichText::new("GATE")
                    .color(Color32::from_rgb(0, 240, 255))
                    .size(18.0)
                    .strong(),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let preset_value = params.preset.value();
                let preset_names = [
                    "Default", "Modern Metal", "Djent", "Thall", "Hardcore", "Deathcore",
                    "Prog Metal", "Bass", "Clean",
                ];
                let current_idx = (preset_value as usize).min(preset_names.len() - 1);

                if ui.button("▶").clicked() {
                    let next = ((preset_value + 1.0) as usize).min(preset_names.len() - 1) as f32;
                    Self::apply_preset_by_index(setter, params, next);
                }

                ui.label(
                    egui::RichText::new(preset_names[current_idx])
                        .color(Color32::from_rgb(200, 200, 210))
                        .strong()
                        .size(12.0),
                );

                if ui.button("◀").clicked() {
                    let prev = (preset_value - 1.0).max(0.0);
                    Self::apply_preset_by_index(setter, params, prev);
                }

                ui.label(
                    egui::RichText::new("PRESET:")
                        .color(Color32::from_rgb(143, 147, 160))
                        .size(11.0),
                );
            });
        });
    }

    fn render_meter_bridge(ui: &mut egui::Ui, gui_state: &Arc<GuiState>) {
        egui::Frame::NONE
            .fill(Color32::from_rgb(27, 28, 32))
            .corner_radius(CornerRadius::same(6))
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Cálculo de niveles
                    let input_samples = gui_state.waveform.input_samples.load();
                    let output_samples = gui_state.waveform.output_samples.load();
                    let gr_samples = gui_state.waveform.gain_reduction.load();

                    let in_db = Self::calc_rms_db(&input_samples);
                    let out_db = Self::calc_rms_db(&output_samples);
                    let gr_db = Self::calc_avg_gr_db(&gr_samples);

                    // Renderizado de la barra GR central horizontal
                    let width = ui.available_width();
                    let height = 24.0;
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
                    let painter = ui.painter_at(rect);

                    // Fondo de medidor
                    painter.rect_filled(rect, CornerRadius::same(4), Color32::from_rgb(18, 19, 22));

                    // Barra de Gain Reduction (crece hacia la izquierda o destaca en rojo)
                    let gr_norm = ((gr_db.abs()) / 60.0).clamp(0.0, 1.0);
                    if gr_norm > 0.001 {
                        let gr_width = width * gr_norm;
                        let gr_rect = Rect::from_min_size(
                            Pos2::new(rect.min.x, rect.min.y),
                            Vec2::new(gr_width, height),
                        );
                        painter.rect_filled(gr_rect, CornerRadius::same(4), Color32::from_rgb(255, 59, 92));
                    }

                    // Texto del puente de medición - siempre muestra atenuación en dB
                    let gr_display = if gr_db.abs() < 0.1 {
                        0.0
                    } else {
                        gr_db.abs()
                    };
                    
                    let status_text = format!("GAIN REDUCTION: {:.1} dB", gr_display);
                    let status_color = Color32::from_rgb(200, 200, 210);

                    painter.text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        status_text,
                        egui::FontId::proportional(11.0),
                        status_color,
                    );

                    // Sub-lectura IN / OUT en texto pequeño
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("IN: {:.1} dB", in_db))
                                .color(Color32::from_rgb(143, 147, 160))
                                .size(10.0),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("OUT: {:.1} dB", out_db))
                                    .color(Color32::from_rgb(143, 147, 160))
                                    .size(10.0),
                            );
                        });
                    });
                });
            });
    }

    fn render_main_controls(ui: &mut egui::Ui, params: &Arc<NoiseGateParams>, setter: &ParamSetter) {
        egui::Frame::NONE
            .fill(Color32::from_rgb(27, 28, 32))
            .corner_radius(CornerRadius::same(6))
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.columns(5, |cols| {
                    Self::render_arc_knob(&mut cols[0], &params.threshold_db, "THRESHOLD", -60.0, 0.0, "dB", setter);
                    Self::render_arc_knob(&mut cols[1], &params.attack_ms, "ATTACK", 0.01, 50.0, "ms", setter);
                    Self::render_arc_knob(&mut cols[2], &params.hold_ms, "HOLD", 0.0, 500.0, "ms", setter);
                    Self::render_arc_knob(&mut cols[3], &params.release_ms, "RELEASE", 5.0, 1000.0, "ms", setter);
                    Self::render_arc_knob(&mut cols[4], &params.range_db, "RANGE", -80.0, 0.0, "dB", setter);
                });
            });
    }

    fn render_advanced_section(ui: &mut egui::Ui, params: &Arc<NoiseGateParams>, setter: &ParamSetter) {
        egui::Frame::NONE
            .fill(Color32::from_rgb(27, 28, 32))
            .corner_radius(CornerRadius::same(6))
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Controles de Sidechain (Filtros)
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("SIDECHAIN FILTERS")
                                .color(Color32::from_rgb(143, 147, 160))
                                .size(10.0)
                                .strong(),
                        );
                        ui.horizontal(|ui| {
                            Self::render_compact_knob(ui, &params.sidechain_hpf_hz, "HPF", 20.0, 800.0, "Hz", setter);
                            ui.add_space(10.0);
                            Self::render_compact_knob(ui, &params.sidechain_lpf_hz, "LPF", 200.0, 20000.0, "Hz", setter);
                            ui.add_space(10.0);
                            Self::render_compact_knob(ui, &params.lookahead_ms, "LOOKAHEAD", 0.0, 10.0, "ms", setter);
                        });
                    });

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);

                    // Controles Inteligentes (Toggles Modernos)
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("SMART DETECTION")
                                .color(Color32::from_rgb(143, 147, 160))
                                .size(10.0)
                                .strong(),
                        );
                        ui.horizontal(|ui| {
                            Self::render_modern_toggle(ui, &params.pick_detector_enabled, "PICK", setter);
                            Self::render_modern_toggle(ui, &params.palm_mute_enabled, "PALM", setter);
                            Self::render_modern_toggle(ui, &params.smart_gate_enabled, "SMART", setter);
                            Self::render_modern_toggle(ui, &params.adaptive_threshold_enabled, "ADAPT", setter);
                        });
                    });
                });
            });
    }

    // --- COMPONENTES PERSONALIZADOS UX/UI ---

    fn render_arc_knob(
        ui: &mut egui::Ui,
        param: &FloatParam,
        label: &str,
        min: f32,
        max: f32,
        unit: &str,
        setter: &ParamSetter,
    ) {
        let value = param.value();
        let norm = (value - min) / (max - min);

        ui.vertical_centered(|ui| {
            let size = Vec2::splat(54.0);
            let (rect, response) = ui.allocate_exact_size(size, egui::Sense::drag());
            let painter = ui.painter();

            // Drag vertical con modificador Shift para precisión
            if response.dragged() {
                let delta = -response.drag_delta().y;
                let speed = if ui.input(|i| i.modifiers.shift) { 0.001 } else { 0.005 };
                let new_norm = (norm + delta * speed).clamp(0.0, 1.0);
                let new_val = min + new_norm * (max - min);

                setter.begin_set_parameter(param);
                setter.set_parameter(param, new_val);
                setter.end_set_parameter(param);
            }

            // Reset con Doble-click
            if response.double_clicked() {
                setter.begin_set_parameter(param);
                setter.set_parameter(param, param.default_plain_value());
                setter.end_set_parameter(param);
            }

            let center = rect.center();
            let radius = rect.width() / 2.0 - 4.0;

            // Dibujar Arco de Fondo
            let start_angle = std::f32::consts::PI * 0.75;
            let end_angle = std::f32::consts::PI * 2.25;

            painter.circle_filled(center, radius, Color32::from_rgb(18, 19, 22));

            // Dibujar el arco activo con cyan
            let current_angle = start_angle + norm * (end_angle - start_angle);
            let n_points = 32;
            let points: Vec<Pos2> = (0..=n_points)
                .map(|i| {
                    let t = i as f32 / n_points as f32;
                    let angle = start_angle + t * (current_angle - start_angle);
                    Pos2::new(center.x + radius * angle.cos(), center.y + radius * angle.sin())
                })
                .collect();

            if points.len() > 1 {
                painter.add(egui::Shape::line(
                    points,
                    egui::Stroke::new(3.0, Color32::from_rgb(0, 240, 255)),
                ));
            }

            // Punto / Indicador
            let dot_pos = Pos2::new(
                center.x + (radius - 6.0) * current_angle.cos(),
                center.y + (radius - 6.0) * current_angle.sin(),
            );
            painter.circle_filled(dot_pos, 2.5, Color32::WHITE);

            // Labels
            ui.label(
                egui::RichText::new(label)
                    .color(Color32::from_rgb(143, 147, 160))
                    .size(10.0)
                    .strong(),
            );
            ui.label(
                egui::RichText::new(format!("{:.1} {}", value, unit))
                    .color(Color32::from_rgb(237, 237, 237))
                    .size(11.0),
            );
        });
    }

    fn render_compact_knob(
        ui: &mut egui::Ui,
        param: &FloatParam,
        label: &str,
        min: f32,
        max: f32,
        unit: &str,
        setter: &ParamSetter,
    ) {
        let value = param.value();
        let norm = (value - min) / (max - min);

        ui.horizontal(|ui| {
            let size = Vec2::splat(28.0);
            let (rect, response) = ui.allocate_exact_size(size, egui::Sense::drag());
            let painter = ui.painter();

            if response.dragged() {
                let delta = -response.drag_delta().y;
                let new_norm = (norm + delta * 0.005).clamp(0.0, 1.0);
                let new_val = min + new_norm * (max - min);

                setter.begin_set_parameter(param);
                setter.set_parameter(param, new_val);
                setter.end_set_parameter(param);
            }

            let center = rect.center();
            let radius = rect.width() / 2.0 - 2.0;

            painter.circle_filled(center, radius, Color32::from_rgb(18, 19, 22));

            let angle = std::f32::consts::PI * 0.75 + norm * (std::f32::consts::PI * 1.5);
            let indicator = Pos2::new(
                center.x + (radius - 3.0) * angle.cos(),
                center.y + (radius - 3.0) * angle.sin(),
            );
            painter.line_segment([center, indicator], egui::Stroke::new(2.0, Color32::from_rgb(0, 240, 255)));

            ui.vertical(|ui| {
                ui.label(egui::RichText::new(label).color(Color32::from_rgb(143, 147, 160)).size(9.0));
                ui.label(egui::RichText::new(format!("{:.0}{}", value, unit)).color(Color32::WHITE).size(10.0));
            });
        });
    }

    fn render_modern_toggle(ui: &mut egui::Ui, param: &BoolParam, label: &str, setter: &ParamSetter) {
        let value = param.value();
        let active_color = Color32::from_rgb(0, 240, 255);
        let inactive_color = Color32::from_rgb(35, 37, 43);

        let button = egui::Button::new(
            egui::RichText::new(label)
                .size(10.0)
                .color(if value { Color32::BLACK } else { Color32::from_rgb(143, 147, 160) })
                .strong(),
        )
        .fill(if value { active_color } else { inactive_color })
        .corner_radius(CornerRadius::same(4));

        if ui.add(button).clicked() {
            setter.begin_set_parameter(param);
            setter.set_parameter(param, !value);
            setter.end_set_parameter(param);
        }
    }

    // --- AUXILIARES MATEMÁTICOS ---

    fn calc_rms_db(samples: &[f32; 512]) -> f32 {
        let sum_sq: f32 = samples.iter().map(|&s| s * s).sum();
        let rms = (sum_sq / 512.0).sqrt();
        if rms > 1e-6 { 
            let db = 20.0 * rms.log10();
            if db.is_finite() { db } else { -60.0 }
        } else { -60.0 }
    }

    fn calc_avg_gr_db(samples: &[f32; 512]) -> f32 {
        let mut total_gr = 0.0;
        let mut count = 0;
        for &gr in samples.iter() {
            if gr > 0.001 && gr.is_finite() {
                let gr_db = 20.0 * gr.log10();
                if gr_db.is_finite() {
                    total_gr += gr_db;
                    count += 1;
                }
            }
        }
        if count > 0 {
            let avg = total_gr / count as f32;
            if avg.is_finite() { avg } else { 0.0 }
        } else { 0.0 }
    }

    fn apply_preset_by_index(setter: &ParamSetter, params: &Arc<NoiseGateParams>, index: f32) {
        let preset_obj = Preset::from_index(index);
        let values = preset_obj.get_values();

        setter.begin_set_parameter(&params.preset);
        setter.set_parameter(&params.preset, index);
        setter.end_set_parameter(&params.preset);

        setter.set_parameter(&params.threshold_db, values.threshold_db);
        setter.set_parameter(&params.attack_ms, values.attack_ms);
        setter.set_parameter(&params.hold_ms, values.hold_ms);
        setter.set_parameter(&params.release_ms, values.release_ms);
        setter.set_parameter(&params.range_db, values.range_db);
        setter.set_parameter(&params.sidechain_hpf_hz, values.sidechain_hpf_hz);
        setter.set_parameter(&params.sidechain_lpf_hz, values.sidechain_lpf_hz);
        setter.set_parameter(&params.lookahead_ms, values.lookahead_ms);
        setter.set_parameter(&params.pick_detector_enabled, values.pick_detector_enabled);
        setter.set_parameter(&params.palm_mute_enabled, values.palm_mute_enabled);
        setter.set_parameter(&params.smart_gate_enabled, values.smart_gate_enabled);
        setter.set_parameter(&params.adaptive_threshold_enabled, values.adaptive_threshold_enabled);
    }
}