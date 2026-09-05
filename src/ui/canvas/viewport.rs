use egui::Rect;
use crate::engine::TimelineTransform;

pub const DEFAULT_ORIGIN_MINUTE: f32 = 420.0; // 07:00
pub const DEFAULT_SY: f32 = 1.2;             // 1.2 px por minuto
pub const MAX_SY: f32 = 8.0;                 // Zoom micro
pub const TIME_GUTTER_WIDTH: f32 = 70.0;
pub const HEADER_HEIGHT: f32 = 44.0;
pub const TOTAL_DAY_MINUTES: f32 = 1440.0;   // 24 horas continuas

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

    /// Calcula dinámicamente el factor Sy mínimo para que las 24h cubran exactamente el 100% de la altura útil
    #[inline]
    pub fn min_sy(&self, useful_height: f32) -> f32 {
        (useful_height / TOTAL_DAY_MINUTES).max(0.1)
    }

    /// Desplaza verticalmente el lienzo acotado entre 00:00 y 24:00 sin dejar espacios vacíos
    pub fn pan(&mut self, delta_screen_y: f32, useful_height: f32) {
        let delta_minutes = delta_screen_y / self.sy;
        let max_origin = (TOTAL_DAY_MINUTES - useful_height / self.sy).max(0.0);
        if max_origin < 1e-3 {
            self.origin_minute = 0.0;
        } else {
            self.origin_minute = (self.origin_minute - delta_minutes).clamp(0.0, max_origin);
        }
    }

    /// Zoom centrado en el cursor que se detiene exactamente al ocupar el 100% de la pantalla
    pub fn zoom_at(&mut self, pivot_screen_y: f32, factor: f32, canvas_top_y: f32, useful_height: f32) {
        let useful_y = pivot_screen_y - canvas_top_y - HEADER_HEIGHT;
        let minute_at_cursor = self.origin_minute + (useful_y / self.sy);

        let min_sy = self.min_sy(useful_height);
        let new_sy = (self.sy * factor).clamp(min_sy, MAX_SY);
        self.sy = new_sy;

        // Mantener el minuto bajo el cursor en la misma posición de pantalla
        let new_origin = minute_at_cursor - (useful_y / new_sy);
        let max_origin = (TOTAL_DAY_MINUTES - useful_height / new_sy).max(0.0);
        if max_origin < 1e-3 {
            self.origin_minute = 0.0;
        } else {
            self.origin_minute = new_origin.clamp(0.0, max_origin);
        }
    }

    /// Clampa los límites en caso de redimensionamiento de ventana
    pub fn clamp_view(&mut self, useful_height: f32) {
        let min_sy = self.min_sy(useful_height);
        if self.sy < min_sy {
            self.sy = min_sy;
        }
        let max_origin = (TOTAL_DAY_MINUTES - useful_height / self.sy).max(0.0);
        if max_origin < 1e-3 {
            self.origin_minute = 0.0;
        } else {
            self.origin_minute = self.origin_minute.clamp(0.0, max_origin);
        }
    }

    /// Restablece la vista para encuadrar el día completo (00:00 a 24:00 ocupando 100% de la pantalla)
    pub fn reset_full_day(&mut self, useful_height: f32) {
        self.sy = self.min_sy(useful_height);
        self.origin_minute = 0.0;
    }

    pub fn reset(&mut self, useful_height: f32) {
        self.sy = DEFAULT_SY.max(self.min_sy(useful_height));
        self.origin_minute = DEFAULT_ORIGIN_MINUTE;
        self.clamp_view(useful_height);
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
