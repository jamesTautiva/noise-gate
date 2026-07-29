use nice_plug_egui::EguiState;
use std::sync::Arc;
use egui::{Color32, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        let egui_state = EguiState::from_size(800, 480);

        nice_plug_egui::create_egui_editor(
            egui_state,
            params.clone(),
            Default::default(),
            |ctx, queue, user_data| {
                let _ = queue;
                let _ = user_data;

                // Estilo Dark/Industrial
                let mut visuals = egui::Visuals::dark();
                visuals.panel_fill = Color32::from_rgb(14, 16, 20);
                visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(22, 25, 30);
                ctx.set_visuals(visuals);
            },
            |ui, setter, queue, user_data| {
                let params = user_data.as_ref();
                let _ = queue;

                let cyan_neon = Color32::from_rgb(0, 240, 212);
                let orange_neon = Color32::from_rgb(255, 140, 0);

                egui::CentralPanel::default().show(ui, |ui| {
                    ui.add_space(8.0);

                    // 1. TOP BAR
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("DJENTCUT")
                                .size(22.0)
                                .color(Color32::from_rgb(230, 235, 240))
                                .strong()
                                .monospace(),
                        );
                    });

                    ui.add_space(10.0);

                    // 2. MIDDLE PANEL: WAVEFORM GRAPH & GR METER
                    ui.horizontal(|ui| {
                        // Canvas principal de Onda y Threshold
                        let graph_size = Vec2::new(710.0, 160.0);
                        let (rect, _) = ui.allocate_exact_size(graph_size, Sense::hover());
                        let painter = ui.painter_at(rect);

                        // Fondo del display
                        painter.rect_filled(rect, 4.0, Color32::from_rgb(10, 12, 15));
                        painter.rect_stroke(
                            rect,
                            4.0,
                            Stroke::new(1.0, Color32::from_rgb(35, 40, 48)),
                            StrokeKind::Inside,
                        );

                        // Línea de THRESHOLD (Cyan)
                        let threshold_val = params.threshold_db.value();
                        let norm_thresh = (threshold_val + 60.0) / 60.0;
                        let thresh_y = rect.max.y - (norm_thresh * rect.height());

                        painter.line_segment(
                            [Pos2::new(rect.min.x, thresh_y), Pos2::new(rect.max.x, thresh_y)],
                            Stroke::new(1.5, cyan_neon),
                        );

                        painter.text(
                            Pos2::new(rect.max.x - 70.0, thresh_y - 10.0),
                            egui::Align2::LEFT_TOP,
                            "THRESHOLD LINE",
                            egui::FontId::monospace(9.0),
                            cyan_neon,
                        );

                        // Bloque Naranja de Gain Reduction
                        let chop_rect = Rect::from_min_max(
                            Pos2::new(rect.min.x + 400.0, thresh_y),
                            Pos2::new(rect.min.x + 550.0, rect.max.y - 5.0),
                        );
                        painter.rect_filled(chop_rect, 0.0, Color32::from_rgba_unmultiplied(255, 140, 0, 40));
                        painter.rect_stroke(chop_rect, 0.0, Stroke::new(1.0, orange_neon), StrokeKind::Inside);

                        ui.add_space(15.0);

                        // Vúmetro Vertical (GR Meter)
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("GR").size(10.0).strong());
                            let meter_size = Vec2::new(12.0, 140.0);
                            let (m_rect, _) = ui.allocate_exact_size(meter_size, Sense::hover());
                            ui.painter().rect_filled(m_rect, 2.0, Color32::from_rgb(20, 22, 26));
                            ui.painter().rect_filled(
                                Rect::from_min_max(
                                    Pos2::new(m_rect.min.x, m_rect.min.y + 30.0),
                                    m_rect.max,
                                ),
                                2.0,
                                orange_neon,
                            );
                        });
                    });

                    ui.add_space(25.0);

                    // 3. BOTTOM PANEL: CONTROLES
                    ui.horizontal(|ui| {
                        ui.add_space(20.0);

                        // Sidechain HPF
                        let mut hpf = params.sidechain_hpf_hz.value();
                        if custom_knob(ui, &mut hpf, 20.0..=800.0, "SIDECHAIN HPF", "Hz", 50.0, cyan_neon).changed() {
                            setter.begin_set_parameter(&params.sidechain_hpf_hz);
                            setter.set_parameter(&params.sidechain_hpf_hz, hpf);
                            setter.end_set_parameter(&params.sidechain_hpf_hz);
                        }

                        ui.add_space(40.0);

                        // THRESHOLD (Knob Principal)
                        let mut threshold = params.threshold_db.value();
                        if custom_knob(ui, &mut threshold, -60.0..=0.0, "THRESHOLD", "dB", 80.0, cyan_neon).changed() {
                            setter.begin_set_parameter(&params.threshold_db);
                            setter.set_parameter(&params.threshold_db, threshold);
                            setter.end_set_parameter(&params.threshold_db);
                        }

                        ui.add_space(40.0);

                        // Release
                        let mut release = params.release_ms.value();
                        if custom_knob(ui, &mut release, 1.0..=1000.0, "RELEASE", "ms", 50.0, cyan_neon).changed() {
                            setter.begin_set_parameter(&params.release_ms);
                            setter.set_parameter(&params.release_ms, release);
                            setter.end_set_parameter(&params.release_ms);
                        }

                        ui.add_space(30.0);

                        // Attack
                        let mut attack = params.attack_ms.value();
                        if custom_knob(ui, &mut attack, 0.01..=50.0, "ATTACK", "ms", 50.0, cyan_neon).changed() {
                            setter.begin_set_parameter(&params.attack_ms);
                            setter.set_parameter(&params.attack_ms, attack);
                            setter.end_set_parameter(&params.attack_ms);
                        }

                        ui.add_space(40.0);

                        // Floor Slider Vertical
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("FLOOR").size(11.0).strong());
                            let mut floor = params.floor_db.value();
                            let floor_res = ui.add(
                                egui::Slider::new(&mut floor, -80.0..=0.0)
                                    .vertical()
                                    .show_value(false),
                            );
                            if floor_res.changed() {
                                setter.begin_set_parameter(&params.floor_db);
                                setter.set_parameter(&params.floor_db, floor);
                                setter.end_set_parameter(&params.floor_db);
                            }
                            ui.label(egui::RichText::new(format!("{:.0} dB", floor)).size(10.0));
                        });
                    });

                    // Footer
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("JAMES TAUTIVA DEV")
                                .size(10.0)
                                .color(Color32::from_rgb(100, 110, 120))
                                .strong(),
                        );
                    });
                });
            },
        )
    }