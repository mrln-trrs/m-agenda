/// Motor matemático de proyección afín continua para el planificador semanal.
#[derive(Debug, Clone, Copy)]
pub struct TimelineTransform {
    /// Origen X del lienzo útil (después del gutter de horas)
    pub x_offset: f32,
    /// Ancho útil para las 7 columnas de días
    pub available_width: f32,
    /// Origen Y del lienzo útil (debajo de los encabezados de días)
    pub y_offset: f32,
    /// Altura del viewport visible
    pub viewport_height: f32,
    /// Minuto de origen visible en la parte superior (t_origin)
    pub origin_minute: f32,
    /// Factor de escala vertical: píxeles por minuto (S_y)
    pub sy: f32,
}

impl TimelineTransform {
    pub fn new(
        x_offset: f32,
        available_width: f32,
        y_offset: f32,
        viewport_height: f32,
        origin_minute: f32,
        sy: f32,
    ) -> Self {
        Self {
            x_offset,
            available_width: available_width.max(10.0),
            y_offset,
            viewport_height: viewport_height.max(10.0),
            origin_minute: origin_minute.clamp(0.0, 1440.0),
            sy: sy.max(0.1),
        }
    }

    /// Ancho nominal de cada una de las 7 columnas de días
    #[inline]
    pub fn column_width(&self) -> f32 {
        self.available_width / 7.0
    }

    /// Coordenadas horizontales de la columna para el día `d` (0 = Lunes, 6 = Domingo)
    #[inline]
    pub fn day_column_x(&self, day: u8) -> (f32, f32) {
        let w_col = self.column_width();
        let x_left = self.x_offset + (day.clamp(0, 6) as f32) * w_col;
        (x_left, x_left + w_col)
    }

    /// Convierte una coordenada X en el índice de día de la semana (0..=6)
    pub fn x_to_day(&self, x: f32) -> Option<u8> {
        if x < self.x_offset || x > self.x_offset + self.available_width {
            return None;
        }
        let w_col = self.column_width();
        let day_f = (x - self.x_offset) / w_col;
        let day = (day_f.floor() as u8).clamp(0, 6);
        Some(day)
    }

    /// Proyecta un minuto continuo t a su posición vertical Y en píxeles
    #[inline]
    pub fn time_to_y(&self, minute: f32) -> f32 {
        self.y_offset + (minute - self.origin_minute) * self.sy
    }

    /// Proyección inversa: convierte una coordenada vertical Y a minutos continuos
    #[inline]
    pub fn y_to_time(&self, y: f32) -> f32 {
        self.origin_minute + (y - self.y_offset) / self.sy
    }

    /// Altura en píxeles de una duración dada en minutos
    #[inline]
    pub fn duration_to_height(&self, duration_minutes: f32) -> f32 {
        duration_minutes * self.sy
    }

    /// Calcula la geometría (x, y, w, h) de un evento teniendo en cuenta su subdivisión de carril (k de K)
    pub fn compute_event_box(
        &self,
        day: u8,
        start_min: f32,
        duration_min: f32,
        lane_index: usize,
        total_lanes: usize,
        padding: f32,
    ) -> (f32, f32, f32, f32) {
        let (col_x_left, col_x_right) = self.day_column_x(day);
        let col_w = col_x_right - col_x_left;

        let k = total_lanes.max(1) as f32;
        let w_sub = col_w / k;
        let x_sub = col_x_left + (lane_index as f32) * w_sub;

        let y_top = self.time_to_y(start_min);
        let h_node = self.duration_to_height(duration_min);

        // Aplicamos padding interno sutil para separación estética entre carriles
        let x = x_sub + padding;
        let w = (w_sub - 2.0 * padding).max(2.0);
        let y = y_top;
        let h = h_node.max(1.0);

        (x, y, w, h)
    }

    /// Determina si un rango de tiempo [start_min, end_min) es visible en el viewport
    pub fn is_time_range_visible(&self, start_min: f32, end_min: f32) -> bool {
        let visible_start = self.origin_minute;
        let visible_end = self.origin_minute + self.viewport_height / self.sy;
        start_min < visible_end && end_min > visible_start
    }
}
