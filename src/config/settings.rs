use serde::{Deserialize, Serialize};
use super::theme::ThemeMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgendaConfig {
    pub theme_mode: ThemeMode,
    pub show_travel_buffers: bool,
    pub snap_minutes: f32,
    pub live_preview: bool,
    pub use_lod_hysteresis: bool,
    pub font_scale: f32,
    pub start_day_hour: u32,
    pub end_day_hour: u32,
}

impl Default for AgendaConfig {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Dark,
            show_travel_buffers: true,
            snap_minutes: 5.0, // Snapping suave a múltiplos de 5 min al arrastrar (0.0 = continuo puro)
            live_preview: true,
            use_lod_hysteresis: true,
            font_scale: 1.0,
            start_day_hour: 7,
            end_day_hour: 22,
        }
    }
}
