use serde::{Deserialize, Serialize};
use super::time_range::TimeRange;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChecklistItem {
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: String,
    /// Día de la semana: 0 = Lunes, 6 = Domingo
    pub day: u8,
    /// Rango temporal continuo [start_min, end_min)
    pub time: TimeRange,
    /// QUÉ: Título o nombre de la actividad
    pub title: String,
    /// DÓNDE: Espacio físico, enlace virtual o modalidad ("📍 Aula 302", "🔗 Google Meet", "🏠 Home Office")
    pub location: String,
    /// Categoría o etiqueta
    pub category: String,
    /// Color distintivo RGB de categoría
    pub color_rgb: [u8; 3],
    /// Notas de contexto (LOD 2)
    pub notes: String,
    /// Lista de tareas o subtareas (LOD 2)
    pub checklist: Vec<ChecklistItem>,
}

impl Event {
    pub fn new(
        id: impl Into<String>,
        day: u8,
        time: TimeRange,
        title: impl Into<String>,
        location: impl Into<String>,
        color_rgb: [u8; 3],
    ) -> Self {
        Self {
            id: id.into(),
            day: day.clamp(0, 6),
            time,
            title: title.into(),
            location: location.into(),
            category: "General".into(),
            color_rgb,
            notes: String::new(),
            checklist: Vec::new(),
        }
    }

    /// CUÁNDO: Retorna el intervalo horario formateado ("08:15 - 09:00")
    pub fn when_text(&self) -> String {
        self.time.format_interval()
    }
}
