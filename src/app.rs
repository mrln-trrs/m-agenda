use eframe::egui::{self, CentralPanel, Context, TopBottomPanel};
use crate::config::{AgendaConfig, AppTheme};
use crate::model::AgendaModel;
use crate::ui::canvas::{draw_adaptive_grid, render_events_and_buffers, GridViewport};
use crate::viewports::{
    show_event_dialog, show_export_dialog, show_settings_viewport, EventDialogState,
    ExportDialogState,
};

pub struct ChronovexApp {
    pub model: AgendaModel,
    pub config: AgendaConfig,
    pub viewport: GridViewport,
    pub selected_event_id: Option<String>,
    pub show_settings_window: bool,
    pub export_state: ExportDialogState,
    pub event_dialog_state: EventDialogState,
}

impl ChronovexApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            model: AgendaModel::create_sample_week(),
            config: AgendaConfig::default(),
            viewport: GridViewport::new(),
            selected_event_id: None,
            show_settings_window: false,
            export_state: ExportDialogState::default(),
            event_dialog_state: EventDialogState::default(),
        }
    }
}

impl eframe::App for ChronovexApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // 1. Renderizar ventana nativa secundaria de opciones (Native Viewport independiente del SO)
        show_settings_viewport(ctx, &mut self.show_settings_window, &mut self.config);

        let theme = AppTheme::for_mode(self.config.theme_mode);

        // 2. Barra Superior Minimalista (Top Toolbar)
        TopBottomPanel::top("chronovex_top_bar").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.heading("🗓 Chronovex");
                ui.label(egui::RichText::new("Vector Weekly Planner").small().color(theme.text_secondary));

                ui.separator();

                if ui.button("➕ Nueva Actividad").clicked() {
                    self.event_dialog_state.open_for_new(0, 480.0, 45.0);
                }

                if ui.button("⚙ Opciones (Ventana Externa)").clicked() {
                    self.show_settings_window = true;
                }

                if ui.button("📄 Exportar 16:9").clicked() {
                    self.export_state.is_open = true;
                    self.export_state.show_buffers = self.config.show_travel_buffers;
                }

                ui.separator();

                if ui.button("↺ Reset Vista").clicked() {
                    self.viewport.reset();
                }

                ui.label(format!("Escala S_y: {:.2} px/min", self.viewport.sy));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(ref sel_id) = self.selected_event_id {
                        if let Some(ev) = self.model.get_event(sel_id) {
                            if ui.button("✏ Editar Seleccionado").clicked() {
                                self.event_dialog_state.open_for_edit(ev);
                            }
                        }
                    }
                });
            });
            ui.add_space(4.0);
        });

        // 3. Lienzo Vectorial Continuo Central (Central Canvas a 60 FPS)
        CentralPanel::default()
            .frame(egui::Frame::default().fill(theme.bg_canvas))
            .show(ctx, |ui| {
                let available_rect = ui.available_rect_before_wrap();
                let response = ui.allocate_rect(available_rect, egui::Sense::click_and_drag());
                let painter = ui.painter_at(available_rect);

                // Manejo de Paneo (arrastre vertical con ratón o trackpad)
                if response.dragged() {
                    let drag_delta = response.drag_delta();
                    self.viewport.pan(drag_delta.y);
                }

                // Manejo de Zoom suave con rueda del ratón (centrado en el cursor)
                let scroll_delta = ui.input(|i| i.raw_scroll_delta);
                if scroll_delta.y != 0.0 {
                    let mouse_pos = ui.input(|i| i.pointer.hover_pos()).unwrap_or(available_rect.center());
                    let zoom_factor = if scroll_delta.y > 0.0 { 1.10 } else { 0.90 };
                    self.viewport.zoom_at(mouse_pos.y, zoom_factor, available_rect.min.y);
                }

                let transform = self.viewport.to_transform(available_rect);

                // Detección de doble clic o clic para crear o seleccionar evento
                if response.clicked() {
                    if let Some(click_pos) = response.interact_pointer_pos() {
                        if let Some(day) = transform.x_to_day(click_pos.x) {
                            // Verificar si se hizo clic sobre un evento existente
                            let day_events = self.model.events_for_day(day);
                            let lane_assignments = crate::engine::resolve_day_overlaps(&day_events);
                            let mut clicked_event = None;

                            for ev in &day_events {
                                let assign = lane_assignments.get(&ev.id).copied().unwrap_or(crate::engine::LaneAssignment {
                                    lane_index: 0,
                                    total_lanes: 1,
                                });
                                let (ex, ey, ew, eh) = transform.compute_event_box(
                                    day,
                                    ev.time.start_min,
                                    ev.time.duration(),
                                    assign.lane_index,
                                    assign.total_lanes,
                                    2.5,
                                );
                                let card_rect = egui::Rect::from_min_size(egui::Pos2::new(ex, ey), egui::Vec2::new(ew, eh));
                                if card_rect.contains(click_pos) {
                                    clicked_event = Some((*ev).clone());
                                    break;
                                }
                            }

                            if let Some(ev) = clicked_event {
                                self.selected_event_id = Some(ev.id.clone());
                            } else {
                                self.selected_event_id = None;
                            }
                        }
                    }
                }

                if response.double_clicked() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        if let Some(day) = transform.x_to_day(pos.x) {
                            let mut start_min = transform.y_to_time(pos.y);
                            if self.config.snap_minutes > 0.0 {
                                start_min = (start_min / self.config.snap_minutes).round() * self.config.snap_minutes;
                            }
                            self.event_dialog_state.open_for_new(day, start_min, 45.0);
                        }
                    }
                }

                // 4. Dibujar Grilla Multiescala Adaptativa
                draw_adaptive_grid(&painter, available_rect, &transform, &theme);

                // 5. Renderizar Tarjetas de Eventos con Recorte Vectorial Estricto y Tríada Esencial
                render_events_and_buffers(
                    &painter,
                    available_rect,
                    &transform,
                    &self.model,
                    &theme,
                    self.selected_event_id.as_deref(),
                    self.config.show_travel_buffers,
                    self.config.use_lod_hysteresis,
                );
            });

        // 6. Diálogos Modales Integrados
        show_event_dialog(ctx, &mut self.event_dialog_state, &mut self.model);
        show_export_dialog(ctx, &mut self.export_state, &self.model);
    }
}
