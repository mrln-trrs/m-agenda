use egui::{Align2, FontFamily, FontId, Painter, Pos2, Rect, Stroke};
use crate::config::AppTheme;
use crate::engine::TimelineTransform;
use crate::export::svg::DAY_NAMES;

pub fn draw_adaptive_grid(
    painter: &Painter,
    rect: Rect,
    transform: &TimelineTransform,
    theme: &AppTheme,
) {
    let font_header = FontId::new(13.0, FontFamily::Proportional);
    let font_hours = FontId::new(11.0, FontFamily::Monospace);

    // 1. Cabeceras de Días
    let header_rect = Rect::from_min_max(
        Pos2::new(rect.min.x, rect.min.y),
        Pos2::new(rect.max.x, transform.y_offset),
    );
    painter.rect_filled(header_rect, 0.0, theme.card_bg);

    for d in 0..7 {
        let (x_left, x_right) = transform.day_column_x(d);
        let center_x = (x_left + x_right) / 2.0;

        // Nombre del día
        painter.text(
            Pos2::new(center_x, rect.min.y + 22.0),
            Align2::CENTER_CENTER,
            DAY_NAMES[d as usize],
            font_header.clone(),
            theme.text_primary,
        );

        // Líneas separadoras verticales entre columnas
        painter.line_segment(
            [
                Pos2::new(x_left, transform.y_offset),
                Pos2::new(x_left, rect.max.y),
            ],
            Stroke::new(1.0_f32, theme.grid_macro),
        );
    }

    // Línea vertical derecha de domingo
    let (_, x_dom_right) = transform.day_column_x(6);
    painter.line_segment(
        [
            Pos2::new(x_dom_right, transform.y_offset),
            Pos2::new(x_dom_right, rect.max.y),
        ],
        Stroke::new(1.0_f32, theme.grid_macro),
    );

    // 2. Líneas Horizontales Adaptativas según Sy
    let min_minute = transform.y_to_time(transform.y_offset).max(0.0);
    let max_minute = transform.y_to_time(rect.max.y).min(1440.0);

    let sy = transform.sy;
    let step_minutes = if sy < 0.6 {
        120
    } else if sy < 1.0 {
        60
    } else if sy < 3.0 {
        30
    } else {
        15
    };

    let start_tick = ((min_minute / step_minutes as f32).floor() as i32) * step_minutes;
    let end_tick = ((max_minute / step_minutes as f32).ceil() as i32) * step_minutes;

    for tick in (start_tick..=end_tick).step_by(step_minutes as usize) {
        if tick < 0 || tick > 1440 {
            continue;
        }

        let y = transform.time_to_y(tick as f32);
        if y < transform.y_offset || y > rect.max.y {
            continue;
        }

        let is_major_hour = tick % 60 == 0;
        let is_half_hour = tick % 30 == 0 && !is_major_hour;

        let stroke = if is_major_hour {
            Stroke::new(1.0_f32, theme.grid_macro)
        } else if is_half_hour {
            Stroke::new(0.8_f32, theme.grid_mid)
        } else {
            Stroke::new(0.5_f32, theme.grid_micro)
        };

        // Línea horizontal que cruza los 7 días
        painter.line_segment(
            [
                Pos2::new(transform.x_offset, y),
                Pos2::new(x_dom_right, y),
            ],
            stroke,
        );

        // Etiqueta horaria en el gutter izquierdo para horas principales (y medias horas si sy >= 1.2)
        if is_major_hour || (is_half_hour && sy >= 1.5) {
            let h = (tick / 60) % 24;
            let m = tick % 60;
            let label = format!("{:02}:{:02}", h, m);

            painter.text(
                Pos2::new(transform.x_offset - 10.0, y),
                Align2::RIGHT_CENTER,
                label,
                font_hours.clone(),
                theme.time_label,
            );
        }
    }
}
