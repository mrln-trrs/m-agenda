use egui::Rect;
use crate::engine::TimelineTransform;

pub const DEFAULT_ORIGIN_MINUTE: f32 = 420.0; // 07:00
pub const DEFAULT_SY: f32 = 1.2;             // 1.2 px por minuto
pub const MIN_SY: f32 = 0.4;                 // Zoom macro
pub const MAX_SY: f32 = 8.0;                 // Zoom micro
pub const TIME_GUTTER_WIDTH: f32 = 70.0;
pub const HEADER_HEIGHT: f32 = 44.0;

#[derive(Debug, Clone)]
pub struct GridViewport {
    /// Minuto continuo visible en la parte superior del canvas
    pub origin_minute: f32,
    /// Factor de escala vertical (px / minuto)
    pub sy: f32,
}

impl Default for GridViewport {
    fn default() -> Self {
        Self {
            origin_minute: DEFAULT_ORIGIN_MINUTE,
            sy: DEFAULT_SY,
        }
    }
}

impl GridViewport {
    pub fn new() -> Self {
        Self::default()
    }

    /// Desplaza verticalmente el lienzo
    pub fn pan(&mut self, delta_screen_y: f32) {
        let delta_minutes = delta_screen_y / self.sy;
        self.origin_minute = (self.origin_minute - delta_minutes).clamp(0.0, 1440.0 - 60.0);
    }

    /// Hace zoom vertical centrado en el punto del cursor
    pub fn zoom_at(&mut self, pivot_screen_y: f32, factor: f32, canvas_top_y: f32) {
        let useful_y = pivot_screen_y - canvas_top_y - HEADER_HEIGHT;
        let minute_at_cursor = self.origin_minute + (useful_y / self.sy);

        let new_sy = (self.sy * factor).clamp(MIN_SY, MAX_SY);
        self.sy = new_sy;

        // Mantener el minuto bajo el cursor en la misma posición de pantalla
        let new_origin = minute_at_cursor - (useful_y / new_sy);
        self.origin_minute = new_origin.clamp(0.0, 1440.0 - 60.0);
    }

    pub fn reset(&mut self) {
        self.origin_minute = DEFAULT_ORIGIN_MINUTE;
        self.sy = DEFAULT_SY;
    }

    /// Genera la matriz de transformación para el rectángulo útil del canvas
    pub fn to_transform(&self, rect: Rect) -> TimelineTransform {
        let x_offset = rect.min.x + TIME_GUTTER_WIDTH;
        let available_width = (rect.max.x - x_offset).max(10.0);
        let y_offset = rect.min.y + HEADER_HEIGHT;
        let viewport_height = (rect.max.y - y_offset).max(10.0);

        TimelineTransform::new(
            x_offset,
            available_width,
            y_offset,
            viewport_height,
            self.origin_minute,
            self.sy,
        )
    }
}
