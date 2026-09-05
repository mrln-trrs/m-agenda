use crate::engine::{
    detect_travel_buffers, resolve_day_overlaps, SemanticLod,
};
use crate::model::{AgendaModel, TimeRange};
use super::auto_frame::{
    ExportFraming, CANVAS_16_9_HEIGHT, CANVAS_16_9_WIDTH, DEFAULT_HEADER_HEIGHT,
    DEFAULT_TIME_GUTTER_WIDTH,
};

pub const DAY_NAMES: [&str; 7] = [
    "LUNES", "MARTES", "MIÉRCOLES", "JUEVES", "VIERNES", "SÁBADO", "DOMINGO",
];

pub struct SvgExportOptions {
    pub dark_mode: bool,
    pub show_travel_buffers: bool,
    pub title: String,
}

impl Default for SvgExportOptions {
    fn default() -> Self {
        Self {
            dark_mode: true,
            show_travel_buffers: true,
            title: "Planificador Semanal Vectorial - Chronovex".into(),
        }
    }
}

/// Genera un documento SVG 16:9 vectorial puro (1920x1080)
pub fn generate_svg_16_9(model: &AgendaModel, options: &SvgExportOptions) -> String {
    let framing = ExportFraming::compute(model, 30.0);
    let transform = &framing.transform;

    let (bg_color, card_bg_default, text_primary, text_secondary, grid_line_color, header_bg) =
        if options.dark_mode {
            ("#0f172a", "#1e293b", "#f8fafc", "#94a3b8", "#334155", "#1e293b")
        } else {
            ("#f8fafc", "#ffffff", "#0f172a", "#64748b", "#cbd5e1", "#ffffff")
        };

    let mut out = String::with_capacity(64 * 1024);

    // SVG Header
    out.push_str(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">
<defs>
  <style>
    .day-header {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 14px; font-weight: 700; fill: {}; letter-spacing: 1px; text-anchor: middle; }}
    .time-label {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 12px; font-weight: 600; fill: {}; text-anchor: end; }}
    .ev-title {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 13px; font-weight: 700; fill: {}; }}
    .ev-time {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 11px; font-weight: 600; fill: {}; }}
    .ev-loc {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 11px; font-weight: 500; fill: {}; }}
    .buffer-label {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 10px; font-weight: 600; fill: #eab308; }}
  </style>
  <pattern id="hatch-pattern" width="10" height="10" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
    <line x1="0" y1="0" x2="0" y2="10" stroke="#ca8a04" stroke-width="2" stroke-opacity="0.35" />
  </pattern>
</defs>
<rect width="100%" height="100%" fill="{}" />
"##,
        CANVAS_16_9_WIDTH,
        CANVAS_16_9_HEIGHT,
        CANVAS_16_9_WIDTH,
        CANVAS_16_9_HEIGHT,
        text_primary,
        text_secondary,
        text_primary,
        text_secondary,
        text_secondary,
        bg_color,
    ));

    // 1. Cabeceras de días (LUNES a DOMINGO)
    let w_col = transform.column_width();
    for d in 0..7 {
        let (x_left, x_right) = transform.day_column_x(d);
        let x_center = (x_left + x_right) / 2.0;

        // Fondo de celda de cabecera
        out.push_str(&format!(
            r##"<rect x="{:.1}" y="12" width="{:.1}" height="48" rx="8" fill="{}" stroke="{}" stroke-width="1" />
<text x="{:.1}" y="42" class="day-header">{}</text>
"##,
            x_left + 2.0,
            w_col - 4.0,
            header_bg,
            grid_line_color,
            x_center,
            DAY_NAMES[d as usize],
        ));

        // Separadores verticales de columnas
        out.push_str(&format!(
            r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width="1" stroke-dasharray="3 3" />
"##,
            x_left,
            DEFAULT_HEADER_HEIGHT,
            x_left,
            CANVAS_16_9_HEIGHT - 35.0,
            grid_line_color,
        ));
    }

    // Línea final vertical de domingo
    let (_, right_domingo) = transform.day_column_x(6);
    out.push_str(&format!(
        r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width="1" stroke-dasharray="3 3" />
"##,
        right_domingo,
        DEFAULT_HEADER_HEIGHT,
        right_domingo,
        CANVAS_16_9_HEIGHT - 35.0,
        grid_line_color,
    ));

    // 2. Grilla horizontal y etiquetas horarias (cada 60 min y sublíneas cada 30 min)
    let first_hour = (framing.t_min / 60.0).floor() as i32;
    let last_hour = (framing.t_max / 60.0).ceil() as i32;

    for h in first_hour..=last_hour {
        let minute = (h * 60) as f32;
        let y = transform.time_to_y(minute);

        if y >= DEFAULT_HEADER_HEIGHT && y <= CANVAS_16_9_HEIGHT - 30.0 {
            let label = format!("{:02}:00", (h % 24));
            // Línea principal de hora
            out.push_str(&format!(
                r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width="1.2" />
<text x="{:.1}" y="{:.1}" class="time-label">{}</text>
"##,
                DEFAULT_TIME_GUTTER_WIDTH,
                y,
                right_domingo,
                y,
                grid_line_color,
                DEFAULT_TIME_GUTTER_WIDTH - 12.0,
                y + 4.0,
                label,
            ));

            // Sublínea de media hora (:30) si no supera el límite
            let y_half = transform.time_to_y(minute + 30.0);
            if y_half <= CANVAS_16_9_HEIGHT - 30.0 {
                out.push_str(&format!(
                    r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width="0.8" stroke-dasharray="4 4" stroke-opacity="0.6" />
"##,
                    DEFAULT_TIME_GUTTER_WIDTH,
                    y_half,
                    right_domingo,
                    y_half,
                    grid_line_color,
                ));
            }
        }
    }

    // 3. Renderizado de Eventos y Buffers de cada día
    for d in 0..7 {
        let day_events = model.events_for_day(d);
        if day_events.is_empty() {
            continue;
        }

        let lane_assignments = resolve_day_overlaps(&day_events);

        // Capa opcional de traslados/descansos (< 30 min)
        if options.show_travel_buffers {
            let buffers = detect_travel_buffers(d, &day_events, &lane_assignments, 30.0);
            for buf in buffers {
                let (bx, by, bw, bh) = transform.compute_event_box(
                    d,
                    buf.time.start_min,
                    buf.duration_min,
                    buf.lane_index,
                    buf.total_lanes,
                    3.0,
                );
                out.push_str(&format!(
                    r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="4" fill="url(#hatch-pattern)" stroke="#ca8a04" stroke-width="1" stroke-dasharray="2 2" />
"##,
                    bx, by, bw, bh,
                ));
                if bh >= 16.0 && bw >= 60.0 {
                    out.push_str(&format!(
                        r##"<text x="{:.1}" y="{:.1}" class="buffer-label">⏱ {:.0}m traslado</text>
"##,
                        bx + 6.0,
                        by + 12.0,
                        buf.duration_min,
                    ));
                }
            }
        }

        // Renderizado de eventos con LOD y Tríada Esencial
        for ev in day_events {
            let assign = lane_assignments.get(&ev.id).copied().unwrap_or(crate::engine::LaneAssignment {
                lane_index: 0,
                total_lanes: 1,
            });

            let (ex, ey, ew, eh) = transform.compute_event_box(
                d,
                ev.time.start_min,
                ev.time.duration(),
                assign.lane_index,
                assign.total_lanes,
                3.0,
            );

            let [cr, cg, cb] = ev.color_rgb;
            let accent_hex = format!("#{:02x}{:02x}{:02x}", cr, cg, cb);
            let clip_id = format!("clip-{}", ev.id);
            let lod = SemanticLod::from_height(eh);

            // Definición de clipPath para bounding-box estricto (RF2.3)
            out.push_str(&format!(
                r##"<clipPath id="{}">
  <rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="6" />
</clipPath>
<g clip-path="url(#{})">
  <rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="6" fill="{}" stroke="{}" stroke-width="1.5" />
  <rect x="{:.1}" y="{:.1}" width="5" height="{:.1}" fill="{}" />
"##,
                clip_id,
                ex, ey, ew, eh,
                clip_id,
                ex, ey, ew, eh,
                card_bg_default,
                accent_hex,
                ex, ey, eh,
                accent_hex,
            ));

            let content_x = ex + 10.0;
            let clean_title = xml_escape(&ev.title);
            let clean_when = xml_escape(&ev.when_text());
            let clean_loc = xml_escape(&ev.location);

            match lod {
                SemanticLod::Lod0Micro => {
                    // LOD 0: Micro-bloque (< 38 px)
                    out.push_str(&format!(
                        r##"  <text x="{:.1}" y="{:.1}" class="ev-title" font-size="11">{} - {}</text>
"##,
                        content_x,
                        ey + eh * 0.7,
                        TimeRange::format_minutes(ev.time.start_min),
                        clean_title,
                    ));
                }
                SemanticLod::Lod1Standard => {
                    // LOD 1: Estándar Semanal (38 px <= h < 85 px) -> Tríada Esencial Garantizada (VAL-01)
                    out.push_str(&format!(
                        r##"  <text x="{:.1}" y="{:.1}" class="ev-time">{}</text>
  <text x="{:.1}" y="{:.1}" class="ev-title">{}</text>
  <text x="{:.1}" y="{:.1}" class="ev-loc">{}</text>
"##,
                        content_x,
                        ey + 14.0,
                        clean_when,
                        content_x,
                        ey + 28.0,
                        clean_title,
                        content_x,
                        ey + 42.0,
                        clean_loc,
                    ));
                }
                SemanticLod::Lod2Detailed => {
                    // LOD 2: Detallado (h >= 85 px) -> Tríada + subtareas / notas
                    out.push_str(&format!(
                        r##"  <text x="{:.1}" y="{:.1}" class="ev-time">{}</text>
  <text x="{:.1}" y="{:.1}" class="ev-title">{}</text>
  <text x="{:.1}" y="{:.1}" class="ev-loc">{}</text>
"##,
                        content_x,
                        ey + 16.0,
                        clean_when,
                        content_x,
                        ey + 32.0,
                        clean_title,
                        content_x,
                        ey + 48.0,
                        clean_loc,
                    ));

                    if !ev.notes.is_empty() && eh >= 95.0 {
                        let clean_notes = xml_escape(&ev.notes);
                        out.push_str(&format!(
                            r##"  <text x="{:.1}" y="{:.1}" class="ev-loc" font-style="italic" fill="{}">{}</text>
"##,
                            content_x,
                            ey + 64.0,
                            text_secondary,
                            clean_notes,
                        ));
                    }
                }
            }

            out.push_str("</g>\n");
        }
    }

    out.push_str("</svg>\n");
    out
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
