use serde::{Deserialize, Serialize};

/// Representa un rango de tiempo continuo [start_min, end_min) dentro de un día [0.0, 1440.0).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimeRange {
    pub start_min: f32,
    pub end_min: f32,
}

impl TimeRange {
    pub fn new(start_min: f32, end_min: f32) -> Self {
        let s = start_min.clamp(0.0, 1440.0);
        let e = end_min.clamp(s, 1440.0);
        Self {
            start_min: s,
            end_min: e,
        }
    }

    pub fn from_hm(start_h: u32, start_m: u32, end_h: u32, end_m: u32) -> Self {
        let start = (start_h * 60 + start_m) as f32;
        let end = (end_h * 60 + end_m) as f32;
        Self::new(start, end)
    }

    /// Duración en minutos continuos
    pub fn duration(&self) -> f32 {
        (self.end_min - self.start_min).max(0.0)
    }

    /// Comprueba si dos intervalos temporales se solapan en [t1, t2)
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start_min < other.end_min && other.start_min < self.end_min
    }

    /// Formatea un valor escalar de minutos a "HH:MM"
    pub fn format_minutes(min: f32) -> String {
        let total_sec = (min * 60.0).round() as i32;
        let total_sec = total_sec.clamp(0, 1440 * 60);
        let h = (total_sec / 3600) % 24;
        let m = (total_sec % 3600) / 60;
        format!("{:02}:{:02}", h, m)
    }

    /// Retorna el formato canónico "HH:MM - HH:MM"
    pub fn format_interval(&self) -> String {
        format!(
            "{} - {}",
            Self::format_minutes(self.start_min),
            Self::format_minutes(self.end_min)
        )
    }
}
