use crate::engine::TimelineTransform;
use crate::model::AgendaModel;

pub const CANVAS_16_9_WIDTH: f32 = 1920.0;
pub const CANVAS_16_9_HEIGHT: f32 = 1080.0;
pub const DEFAULT_HEADER_HEIGHT: f32 = 75.0;
pub const DEFAULT_TIME_GUTTER_WIDTH: f32 = 95.0;
pub const DEFAULT_BOTTOM_MARGIN: f32 = 35.0;
pub const DEFAULT_RIGHT_MARGIN: f32 = 30.0;

#[derive(Debug, Clone)]
pub struct ExportFraming {
    pub t_min: f32,
    pub t_max: f32,
    pub sy: f32,
    pub transform: TimelineTransform,
}

impl ExportFraming {
    /// Calcula el encuadre automático 16:9 analizando los eventos activos de la semana (RF3.1, VAL-05)
    pub fn compute(model: &AgendaModel, buffer_minutes: f32) -> Self {
        let (raw_min, raw_max) = model.active_time_bounds();

        // Aplicar margen temporal de amortiguación (ej. 30 min)
        let t_min = (raw_min - buffer_minutes).max(0.0);
        let t_max = (raw_max + buffer_minutes).min(1440.0);
        let time_span = (t_max - t_min).max(60.0); // Mínimo 1 hora de rango

        let useful_width = CANVAS_16_9_WIDTH - DEFAULT_TIME_GUTTER_WIDTH - DEFAULT_RIGHT_MARGIN;
        let useful_height = CANVAS_16_9_HEIGHT - DEFAULT_HEADER_HEIGHT - DEFAULT_BOTTOM_MARGIN;

        let sy = useful_height / time_span;

        let transform = TimelineTransform::new(
            DEFAULT_TIME_GUTTER_WIDTH,
            useful_width,
            DEFAULT_HEADER_HEIGHT,
            useful_height,
            t_min,
            sy,
        );

        Self {
            t_min,
            t_max,
            sy,
            transform,
        }
    }
}
