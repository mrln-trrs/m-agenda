use serde::{Deserialize, Serialize};
use super::event::{ChecklistItem, Event};
use super::time_range::TimeRange;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgendaModel {
    pub events: Vec<Event>,
}

impl AgendaModel {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn remove_event(&mut self, id: &str) -> bool {
        let initial_len = self.events.len();
        self.events.retain(|e| e.id != id);
        self.events.len() < initial_len
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.iter().find(|e| e.id == id)
    }

    pub fn get_event_mut(&mut self, id: &str) -> Option<&mut Event> {
        self.events.iter_mut().find(|e| e.id == id)
    }

    pub fn events_for_day(&self, day: u8) -> Vec<&Event> {
        self.events.iter().filter(|e| e.day == day).collect()
    }

    /// Retorna el rango horario activo [t_min, t_max] de todos los eventos de la semana
    pub fn active_time_bounds(&self) -> (f32, f32) {
        if self.events.is_empty() {
            // Default 07:00 (420 min) a 22:00 (1320 min)
            return (420.0, 1320.0);
        }

        let mut min_t = 1440.0f32;
        let mut max_t = 0.0f32;

        for e in &self.events {
            if e.time.start_min < min_t {
                min_t = e.time.start_min;
            }
            if e.time.end_min > max_t {
                max_t = e.time.end_min;
            }
        }

        if min_t >= max_t {
            (420.0, 1320.0)
        } else {
            (min_t, max_t)
        }
    }

    /// Genera una semana de ejemplo que cubre todos los casos de prueba de la especificación
    pub fn create_sample_week() -> Self {
        let mut model = Self::new();

        // Lunes (0)
        // Standup micro-bloque (15 min) -> LOD 0
        model.add_event(Event::new(
            "mon_1",
            0,
            TimeRange::new(480.0, 495.0), // 08:00 - 08:15
            "Standup Semanal",
            "🔗 Zoom Room 1",
            [239, 68, 68], // Red
        ));

        // Bloque de 45 min inmediatamente contiguo (08:15 - 09:00) -> VAL-01 & VAL-03
        model.add_event(Event::new(
            "mon_2",
            0,
            TimeRange::new(495.0, 540.0), // 08:15 - 09:00 (45 min)
            "Arquitectura del Compilador",
            "📍 Aula Magna 301",
            [59, 130, 246], // Blue
        ));

        // Bloque de 60 min contiguo (09:00 - 10:00) -> VAL-03 (Sin brechas ni solapes)
        model.add_event(Event::new(
            "mon_3",
            0,
            TimeRange::new(540.0, 600.0), // 09:00 - 10:00 (60 min)
            "Diseño de Motores Gráficos",
            "📍 Lab Vectorial 402",
            [16, 185, 129], // Emerald
        ));

        // Bloque de 2 horas (120 min) con checklist -> LOD 2
        let mut mon_deep = Event::new(
            "mon_4",
            0,
            TimeRange::new(660.0, 780.0), // 11:00 - 13:00 (120 min)
            "Deep Work: Pipeline de Renderizado",
            "🏠 Home Office",
            [139, 92, 246], // Violet
        );
        mon_deep.notes = "Revisar algoritmos de viewport culling y matrices de proyección afín.".into();
        mon_deep.checklist = vec![
            ChecklistItem { text: "Revisar clipper de polígonos".into(), completed: true },
            ChecklistItem { text: "Optimizar caché tipográfica".into(), completed: false },
            ChecklistItem { text: "Medir FPS a 60 cuadros".into(), completed: true },
        ];
        model.add_event(mon_deep);

        // Martes (1): Sesiones de trabajo
        model.add_event(Event::new(
            "tue_1",
            1,
            TimeRange::new(510.0, 600.0), // 08:30 - 10:00 (90 min)
            "Auditoría de Rendimiento GPU",
            "🔗 Meet / Devs",
            [245, 158, 11], // Amber
        ));
        model.add_event(Event::new(
            "tue_2",
            1,
            TimeRange::new(630.0, 720.0), // 10:30 - 12:00 (90 min)
            "Sistemas de Coordenadas Continuas",
            "📍 Auditorio Central",
            [59, 130, 246],
        ));

        // Miércoles (2): Eventos concurrentes solapados (RF1.3)
        // Bloque A: 14:00 - 15:30 (90 min)
        model.add_event(Event::new(
            "wed_solape_1",
            2,
            TimeRange::new(840.0, 930.0), // 14:00 - 15:30
            "Comité Técnico de Gráficos",
            "📍 Sala Innovación 1",
            [236, 72, 153], // Pink
        ));
        // Bloque B: 14:30 - 16:00 (90 min) -> Concurrencia K = 2
        model.add_event(Event::new(
            "wed_solape_2",
            2,
            TimeRange::new(870.0, 960.0), // 14:30 - 16:00
            "Revisión de Seguridad & WCAG",
            "🔗 Teams Room B",
            [16, 185, 129],
        ));

        // Jueves (3): Buffer de traslado de 15 min entre eventos (< 30 min)
        model.add_event(Event::new(
            "thu_1",
            3,
            TimeRange::new(540.0, 600.0), // 09:00 - 10:00
            "Planificación de Producto",
            "📍 Oficina Principal",
            [59, 130, 246],
        ));
        // Brecha de 15 min: 10:00 - 10:15 (Traslado)
        model.add_event(Event::new(
            "thu_2",
            3,
            TimeRange::new(615.0, 675.0), // 10:15 - 11:15 (60 min)
            "Reunión con Clientes",
            "📍 Sucursal Norte",
            [139, 92, 246],
        ));

        // Viernes (4): Jornada vespertina
        model.add_event(Event::new(
            "fri_1",
            4,
            TimeRange::new(600.0, 690.0), // 10:00 - 11:30
            "Demostración de Exportación 16:9",
            "🔗 Zoom Broadcast",
            [16, 185, 129],
        ));
        model.add_event(Event::new(
            "fri_2",
            4,
            TimeRange::new(720.0, 780.0), // 12:00 - 13:00
            "Retrospectiva del Sprint",
            "📍 Sala de Juntas",
            [245, 158, 11],
        ));

        model
    }
}
