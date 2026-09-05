use egui::{
    Align2, Color32, FontFamily, FontId, Painter, Pos2, Rect, Rounding, Stroke, Vec2,
};
use crate::config::AppTheme;
use crate::engine::{
    detect_travel_buffers, resolve_day_overlaps, SemanticLod, TimelineTransform,
};
use crate::model::AgendaModel;

pub fn render_events_and_buffers(
    painter: &Painter,
    canvas_rect: Rect,
    transform: &TimelineTransform,
    model: &AgendaModel,
    theme: &AppTheme,
    selected_id: Option<&str>,
    show_buffers: bool,
    use_hysteresis: bool,
) {
    let font_title = FontId::new(12.5, FontFamily::Proportional);
    let font_subtitle = FontId::new(10.5, FontFamily::Proportional);
    let font_time = FontId::new(10.0, FontFamily::Monospace);
    let font_small = FontId::new(9.5, FontFamily::Proportional);

    for d in 0..7 {
        let day_events = model.events_for_day(d);
        if day_events.is_empty() {
            continue;
        }

        let lane_assignments = resolve_day_overlaps(&day_events);

        // 1. Capa de traslados/descansos achurados (< 30 min)
        if show_buffers {
            let buffers = detect_travel_buffers(d, &day_events, &lane_assignments, 30.0);
            for buf in buffers {
                let (bx, by, bw, bh) = transform.compute_event_box(
                    d,
                    buf.time.start_min,
                    buf.duration_min,
                    buf.lane_index,
                    buf.total_lanes,
                    2.0,
                );

                let buf_rect = Rect::from_min_size(Pos2::new(bx, by), Vec2::new(bw, bh));
                // Viewport culling
                if buf_rect.max.y < canvas_rect.min.y || buf_rect.min.y > canvas_rect.max.y {
                    continue;
                }

                // Fondo con patrón translúcido de advertencia suave
                painter.rect_filled(
                    buf_rect,
                    Rounding::same(3.0),
                    Color32::from_rgba_premultiplied(234, 179, 8, 25),
                );
                painter.rect_stroke(
                    buf_rect,
                    Rounding::same(3.0),
                    Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(234, 179, 8, 80)),
                );

                // Etiqueta de buffer si hay espacio vertical
                if bh >= 14.0 && bw >= 40.0 {
                    painter.text(
                        Pos2::new(bx + 4.0, by + 2.0),
                        Align2::LEFT_TOP,
                        format!("⏱ {:.0}m buffer", buf.duration_min),
                        font_small.clone(),
                        theme.buffer_hatch,
                    );
                }
            }
        }

        // 2. Renderizado de tarjetas de eventos vectoriales
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
                2.5,
            );

            let card_rect = Rect::from_min_size(Pos2::new(ex, ey), Vec2::new(ew, eh));

            // Viewport culling: descartar nodos completamente fuera de pantalla
            if card_rect.max.y < canvas_rect.min.y || card_rect.min.y > canvas_rect.max.y {
                continue;
            }

            let is_selected = selected_id == Some(&ev.id);
            let [r, g, b] = ev.color_rgb;
            let accent_color = Color32::from_rgb(r, g, b);

            let stroke_border = if is_selected {
                Stroke::new(2.5_f32, theme.selection)
            } else {
                Stroke::new(1.2_f32, accent_color)
            };

            // Bounding-box clipping estricto (RF2.3): Nada invade celdas vecinas
            let card_painter = painter.with_clip_rect(card_rect);

            // Fondo de tarjeta con esquinas redondeadas
            card_painter.rect_filled(card_rect, Rounding::same(5.0), theme.card_bg);
            card_painter.rect_stroke(card_rect, Rounding::same(5.0), stroke_border);

            // Barra vertical de categoría a la izquierda (4 px)
            let bar_rect = Rect::from_min_size(card_rect.min, Vec2::new(4.0, eh));
            card_painter.rect_filled(bar_rect, Rounding::same(2.0), accent_color);

            // Clasificación semántica de LOD
            let lod = if use_hysteresis {
                SemanticLod::from_height_with_hysteresis(eh, None)
            } else {
                SemanticLod::from_height(eh)
            };

            let text_left = ex + 9.0;

            match lod {
                SemanticLod::Lod0Micro => {
                    // LOD 0: Micro-bloques (< 38 px)
                    let text_str = format!("{} {}", ev.time.format_interval(), ev.title);
                    card_painter.text(
                        Pos2::new(text_left, ey + eh * 0.5),
                        Align2::LEFT_CENTER,
                        text_str,
                        font_small.clone(),
                        theme.text_primary,
                    );
                }
                SemanticLod::Lod1Standard => {
                    // LOD 1: Tríada Esencial (QUÉ, CUÁNDO, DÓNDE) para bloques >= 45 min
                    // Renglón 1: Horario
                    card_painter.text(
                        Pos2::new(text_left, ey + 4.0),
                        Align2::LEFT_TOP,
                        ev.when_text(),
                        font_time.clone(),
                        theme.text_secondary,
                    );

                    // Renglón 2: Título (Qué)
                    card_painter.text(
                        Pos2::new(text_left, ey + 17.0),
                        Align2::LEFT_TOP,
                        &ev.title,
                        font_title.clone(),
                        theme.text_primary,
                    );

                    // Renglón 3: Ubicación (Dónde)
                    if eh >= 48.0 && !ev.location.is_empty() {
                        card_painter.text(
                            Pos2::new(text_left, ey + 32.0),
                            Align2::LEFT_TOP,
                            &ev.location,
                            font_subtitle.clone(),
                            theme.text_secondary,
                        );
                    }
                }
                SemanticLod::Lod2Detailed => {
                    // LOD 2: Detalle Máximo (Tríada + Checklists + Notas)
                    // Horario
                    card_painter.text(
                        Pos2::new(text_left, ey + 5.0),
                        Align2::LEFT_TOP,
                        ev.when_text(),
                        font_time.clone(),
                        theme.text_secondary,
                    );

                    // Título
                    card_painter.text(
                        Pos2::new(text_left, ey + 18.0),
                        Align2::LEFT_TOP,
                        &ev.title,
                        font_title.clone(),
                        theme.text_primary,
                    );

                    // Ubicación
                    card_painter.text(
                        Pos2::new(text_left, ey + 34.0),
                        Align2::LEFT_TOP,
                        &ev.location,
                        font_subtitle.clone(),
                        theme.text_secondary,
                    );

                    let mut cursor_y = ey + 52.0;

                    // Checklist interactiva o indicadores de subtareas
                    for item in &ev.checklist {
                        if cursor_y + 14.0 > card_rect.max.y {
                            break;
                        }
                        let icon = if item.completed { "☑ " } else { "☐ " };
                        card_painter.text(
                            Pos2::new(text_left, cursor_y),
                            Align2::LEFT_TOP,
                            format!("{}{}", icon, item.text),
                            font_small.clone(),
                            theme.text_secondary,
                        );
                        cursor_y += 14.0;
                    }

                    // Notas breves
                    if !ev.notes.is_empty() && cursor_y + 14.0 <= card_rect.max.y {
                        card_painter.text(
                            Pos2::new(text_left, cursor_y),
                            Align2::LEFT_TOP,
                            &ev.notes,
                            font_small.clone(),
                            theme.text_secondary,
                        );
                    }
                }
            }
        }
    }
}
