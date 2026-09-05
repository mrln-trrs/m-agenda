use egui::{Context, Window};
use crate::model::{Event, TimeRange, AgendaModel};

#[derive(Default)]
pub struct EventDialogState {
    pub is_open: bool,
    pub event_id: Option<String>,
    pub day: u8,
    pub start_hour: u32,
    pub start_minute: u32,
    pub duration_minutes: f32,
    pub title: String,
    pub location: String,
    pub color_rgb: [u8; 3],
    pub notes: String,
}

impl EventDialogState {
    pub fn open_for_new(&mut self, day: u8, start_min: f32, duration: f32) {
        self.is_open = true;
        self.event_id = None;
        self.day = day.clamp(0, 6);
        let start_total = start_min.round() as u32;
        self.start_hour = (start_total / 60) % 24;
        self.start_minute = start_total % 60;
        self.duration_minutes = duration.max(15.0);
        self.title = "Nuevo Bloque".into();
        self.location = "📍 Presencial / Virtual".into();
        self.color_rgb = [59, 130, 246]; // Azul
        self.notes = String::new();
    }

    pub fn open_for_edit(&mut self, event: &Event) {
        self.is_open = true;
        self.event_id = Some(event.id.clone());
        self.day = event.day;
        let start_total = event.time.start_min.round() as u32;
        self.start_hour = (start_total / 60) % 24;
        self.start_minute = start_total % 60;
        self.duration_minutes = event.time.duration();
        self.title = event.title.clone();
        self.location = event.location.clone();
        self.color_rgb = event.color_rgb;
        self.notes = event.notes.clone();
    }
}

pub fn show_event_dialog(
    ctx: &Context,
    state: &mut EventDialogState,
    model: &mut AgendaModel,
) {
    if !state.is_open {
        return;
    }

    let mut open_flag = state.is_open;
    let mut close_dialog = false;

    Window::new(if state.event_id.is_some() { "✏ Editar Actividad" } else { "➕ Nueva Actividad" })
        .open(&mut open_flag)
        .collapsible(false)
        .resizable(false)
        .default_size([380.0, 420.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.add_space(4.0);

            ui.label(egui::RichText::new("QUÉ (Título):").strong());
            ui.text_edit_singleline(&mut state.title);

            ui.add_space(6.0);
            ui.label(egui::RichText::new("DÓNDE (Ubicación o Enlace):").strong());
            ui.text_edit_singleline(&mut state.location);

            ui.add_space(6.0);
            ui.label(egui::RichText::new("CUÁNDO (Día y Horario):").strong());
            ui.horizontal(|ui| {
                ui.label("Día:");
                egui::ComboBox::from_id_salt("event_day_combo")
                    .selected_text(match state.day {
                        0 => "Lunes",
                        1 => "Martes",
                        2 => "Miércoles",
                        3 => "Jueves",
                        4 => "Viernes",
                        5 => "Sábado",
                        _ => "Domingo",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut state.day, 0, "Lunes");
                        ui.selectable_value(&mut state.day, 1, "Martes");
                        ui.selectable_value(&mut state.day, 2, "Miércoles");
                        ui.selectable_value(&mut state.day, 3, "Jueves");
                        ui.selectable_value(&mut state.day, 4, "Viernes");
                        ui.selectable_value(&mut state.day, 5, "Sábado");
                        ui.selectable_value(&mut state.day, 6, "Domingo");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Inicio:");
                ui.add(egui::DragValue::new(&mut state.start_hour).range(0..=23).prefix("H: "));
                ui.add(egui::DragValue::new(&mut state.start_minute).range(0..=59).prefix("M: "));
            });

            ui.horizontal(|ui| {
                ui.label("Duración:");
                ui.add(egui::DragValue::new(&mut state.duration_minutes).range(5.0..=720.0).suffix(" min"));
                if ui.button("45m").clicked() { state.duration_minutes = 45.0; }
                if ui.button("60m").clicked() { state.duration_minutes = 60.0; }
                if ui.button("90m").clicked() { state.duration_minutes = 90.0; }
            });

            let start_f = (state.start_hour * 60 + state.start_minute) as f32;
            let end_f = start_f + state.duration_minutes;
            let preview_range = TimeRange::new(start_f, end_f);
            ui.label(format!("Intervalo resultante: {}", preview_range.format_interval()));

            ui.add_space(6.0);
            ui.label(egui::RichText::new("Color distintivo:").strong());
            ui.horizontal(|ui| {
                let colors = [
                    ("Azul", [59, 130, 246]),
                    ("Esmeralda", [16, 185, 129]),
                    ("Violeta", [139, 92, 246]),
                    ("Ámbar", [245, 158, 11]),
                    ("Rosa", [236, 72, 153]),
                    ("Rojo", [239, 68, 68]),
                ];
                for (name, col) in colors {
                    if ui.selectable_label(state.color_rgb == col, name).clicked() {
                        state.color_rgb = col;
                    }
                }
            });

            ui.add_space(6.0);
            ui.label(egui::RichText::new("Notas (LOD 2):").strong());
            ui.text_edit_multiline(&mut state.notes);

            ui.add_space(10.0);
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("💾 Guardar").clicked() {
                    let start_min = (state.start_hour * 60 + state.start_minute) as f32;
                    let time = TimeRange::new(start_min, start_min + state.duration_minutes);
                    let id = state.event_id.clone().unwrap_or_else(|| format!("ev_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()));

                    let mut ev = Event::new(
                        id,
                        state.day,
                        time,
                        &state.title,
                        &state.location,
                        state.color_rgb,
                    );
                    ev.notes = state.notes.clone();

                    if let Some(ref existing_id) = state.event_id {
                        model.remove_event(existing_id);
                    }
                    model.add_event(ev);
                    close_dialog = true;
                }

                if state.event_id.is_some() {
                    if ui.button("🗑 Eliminar").clicked() {
                        if let Some(ref id) = state.event_id {
                            model.remove_event(id);
                        }
                        close_dialog = true;
                    }
                }

                if ui.button("Cancelar").clicked() {
                    close_dialog = true;
                }
            });
        });

    if close_dialog || !open_flag {
        state.is_open = false;
    }
}
