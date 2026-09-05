use printpdf::*;
use printpdf::path::{PaintMode, WindingOrder};
use std::io::BufWriter;
use crate::model::AgendaModel;
use crate::engine::{resolve_day_overlaps, SemanticLod};
use super::auto_frame::ExportFraming;
use super::svg::DAY_NAMES;

pub struct PdfExportOptions {
    pub dark_mode: bool,
    pub title: String,
}

impl Default for PdfExportOptions {
    fn default() -> Self {
        Self {
            dark_mode: false,
            title: "Chronovex Semanal 16:9".into(),
        }
    }
}

#[inline]
fn pt_point(x: f32, y: f32) -> Point {
    Point::new(Pt(x).into(), Pt(y).into())
}

#[inline]
fn to_mm(val: f32) -> Mm {
    Pt(val).into()
}

/// Genera un documento PDF vectorial puro con printpdf (VAL-02: resolución infinita, cero bitmaps)
pub fn generate_pdf_16_9(model: &AgendaModel, options: &PdfExportOptions) -> Result<Vec<u8>, String> {
    let width_pt = 1920.0f32;
    let height_pt = 1080.0f32;

    let (doc, page1, layer1) = PdfDocument::new(
        &options.title,
        to_mm(width_pt),
        to_mm(height_pt),
        "Capa Vectorial",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font_bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Error cargando fuente HelveticaBold: {:?}", e))?;
    let font_regular = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Error cargando fuente Helvetica: {:?}", e))?;

    let framing = ExportFraming::compute(model, 30.0);
    let transform = &framing.transform;

    // En PDF el origen (0, 0) está en la esquina inferior izquierda.
    // Función de conversión de Y de pantalla (origen arriba) a Y de PDF (origen abajo)
    let to_pdf_y = |screen_y: f32| -> f32 {
        height_pt - screen_y
    };

    // Fondo de la lámina
    let (bg_color, card_bg, text_primary, text_secondary, grid_color) = if options.dark_mode {
        (
            Color::Rgb(Rgb::new(15.0 / 255.0, 23.0 / 255.0, 42.0 / 255.0, None)),
            Color::Rgb(Rgb::new(30.0 / 255.0, 41.0 / 255.0, 59.0 / 255.0, None)),
            Color::Rgb(Rgb::new(248.0 / 255.0, 250.0 / 255.0, 252.0 / 255.0, None)),
            Color::Rgb(Rgb::new(148.0 / 255.0, 163.0 / 255.0, 184.0 / 255.0, None)),
            Color::Rgb(Rgb::new(51.0 / 255.0, 65.0 / 255.0, 85.0 / 255.0, None)),
        )
    } else {
        (
            Color::Rgb(Rgb::new(248.0 / 255.0, 250.0 / 255.0, 252.0 / 255.0, None)),
            Color::Rgb(Rgb::new(1.0, 1.0, 1.0, None)),
            Color::Rgb(Rgb::new(15.0 / 255.0, 23.0 / 255.0, 42.0 / 255.0, None)),
            Color::Rgb(Rgb::new(100.0 / 255.0, 116.0 / 255.0, 139.0 / 255.0, None)),
            Color::Rgb(Rgb::new(203.0 / 255.0, 213.0 / 255.0, 225.0 / 255.0, None)),
        )
    };

    current_layer.set_fill_color(bg_color);
    let bg_rect = Polygon {
        rings: vec![vec![
            (pt_point(0.0, 0.0), false),
            (pt_point(width_pt, 0.0), false),
            (pt_point(width_pt, height_pt), false),
            (pt_point(0.0, height_pt), false),
        ]],
        mode: PaintMode::Fill,
        winding_order: WindingOrder::NonZero,
    };
    current_layer.add_polygon(bg_rect);

    // 1. Cabeceras de días y separadores de columnas
    let w_col = transform.column_width();
    for d in 0..7 {
        let (x_left, x_right) = transform.day_column_x(d);

        // Caja de cabecera
        current_layer.set_fill_color(card_bg.clone());
        current_layer.set_outline_color(grid_color.clone());
        current_layer.set_outline_thickness(1.0);
        let header_top_y = to_pdf_y(12.0);
        let header_bottom_y = to_pdf_y(60.0);

        let header_rect = Polygon {
            rings: vec![vec![
                (pt_point(x_left + 2.0, header_bottom_y), false),
                (pt_point(x_right - 2.0, header_bottom_y), false),
                (pt_point(x_right - 2.0, header_top_y), false),
                (pt_point(x_left + 2.0, header_top_y), false),
            ]],
            mode: PaintMode::FillStroke,
            winding_order: WindingOrder::NonZero,
        };
        current_layer.add_polygon(header_rect);

        // Texto de cabecera
        current_layer.set_fill_color(text_primary.clone());
        current_layer.use_text(
            DAY_NAMES[d as usize],
            14.0,
            to_mm(x_left + (w_col * 0.3)),
            to_mm(to_pdf_y(40.0)),
            &font_bold,
        );

        // Línea vertical separadora
        current_layer.set_outline_color(grid_color.clone());
        current_layer.set_outline_thickness(0.8);
        let col_line = Line {
            points: vec![
                (pt_point(x_left, to_pdf_y(height_pt - 35.0)), false),
                (pt_point(x_left, to_pdf_y(75.0)), false),
            ],
            is_closed: false,
        };
        current_layer.add_line(col_line);
    }

    // Línea final vertical
    let (_, x_domingo_right) = transform.day_column_x(6);
    let last_col_line = Line {
        points: vec![
            (pt_point(x_domingo_right, to_pdf_y(height_pt - 35.0)), false),
            (pt_point(x_domingo_right, to_pdf_y(75.0)), false),
        ],
        is_closed: false,
    };
    current_layer.add_line(last_col_line);

    // 2. Grilla horizontal y etiquetas horarias
    let first_hour = (framing.t_min / 60.0).floor() as i32;
    let last_hour = (framing.t_max / 60.0).ceil() as i32;

    for h in first_hour..=last_hour {
        let minute = (h * 60) as f32;
        let y_screen = transform.time_to_y(minute);
        if y_screen >= 75.0 && y_screen <= height_pt - 30.0 {
            let y_pdf = to_pdf_y(y_screen);
            let hour_line = Line {
                points: vec![
                    (pt_point(95.0, y_pdf), false),
                    (pt_point(x_domingo_right, y_pdf), false),
                ],
                is_closed: false,
            };
            current_layer.set_outline_color(grid_color.clone());
            current_layer.set_outline_thickness(1.0);
            current_layer.add_line(hour_line);

            let label = format!("{:02}:00", h % 24);
            current_layer.set_fill_color(text_secondary.clone());
            current_layer.use_text(label, 12.0, to_mm(40.0), to_mm(y_pdf - 4.0), &font_bold);
        }
    }

    // 3. Renderizado de tarjetas de eventos vectoriales
    for d in 0..7 {
        let day_events = model.events_for_day(d);
        if day_events.is_empty() {
            continue;
        }

        let lane_assignments = resolve_day_overlaps(&day_events);

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

            let y_top_pdf = to_pdf_y(ey);
            let y_bottom_pdf = to_pdf_y(ey + eh);

            let [r, g, b] = ev.color_rgb;
            let accent_color = Color::Rgb(Rgb::new(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                None,
            ));

            // Rectángulo de tarjeta
            current_layer.set_fill_color(card_bg.clone());
            current_layer.set_outline_color(accent_color.clone());
            current_layer.set_outline_thickness(1.5);

            let card_poly = Polygon {
                rings: vec![vec![
                    (pt_point(ex, y_bottom_pdf), false),
                    (pt_point(ex + ew, y_bottom_pdf), false),
                    (pt_point(ex + ew, y_top_pdf), false),
                    (pt_point(ex, y_top_pdf), false),
                ]],
                mode: PaintMode::FillStroke,
                winding_order: WindingOrder::NonZero,
            };
            current_layer.add_polygon(card_poly);

            // Barra vertical de acento (5 pt ancho)
            current_layer.set_fill_color(accent_color.clone());
            let bar_poly = Polygon {
                rings: vec![vec![
                    (pt_point(ex, y_bottom_pdf), false),
                    (pt_point(ex + 5.0, y_bottom_pdf), false),
                    (pt_point(ex + 5.0, y_top_pdf), false),
                    (pt_point(ex, y_top_pdf), false),
                ]],
                mode: PaintMode::Fill,
                winding_order: WindingOrder::NonZero,
            };
            current_layer.add_polygon(bar_poly);

            // Textos según LOD
            let lod = SemanticLod::from_height(eh);
            let content_x = ex + 10.0;

            match lod {
                SemanticLod::Lod0Micro => {
                    current_layer.set_fill_color(text_primary.clone());
                    let label = format!("{} {}", ev.time.format_interval(), ev.title);
                    let truncated = if label.len() > 24 { &label[..24] } else { &label };
                    current_layer.use_text(truncated, 10.0, to_mm(content_x), to_mm(y_top_pdf - 12.0), &font_bold);
                }
                SemanticLod::Lod1Standard | SemanticLod::Lod2Detailed => {
                    // Tríada Esencial: CUÁNDO, QUÉ, DÓNDE
                    // Renglón 1: Cuándo
                    current_layer.set_fill_color(text_secondary.clone());
                    current_layer.use_text(&ev.when_text(), 10.0, to_mm(content_x), to_mm(y_top_pdf - 14.0), &font_bold);

                    // Renglón 2: Qué
                    current_layer.set_fill_color(text_primary.clone());
                    let title_display = if ev.title.len() > 30 { &ev.title[..30] } else { &ev.title };
                    current_layer.use_text(title_display, 12.0, to_mm(content_x), to_mm(y_top_pdf - 28.0), &font_bold);

                    // Renglón 3: Dónde
                    if eh >= 48.0 {
                        current_layer.set_fill_color(text_secondary.clone());
                        let loc_display = if ev.location.len() > 32 { &ev.location[..32] } else { &ev.location };
                        current_layer.use_text(loc_display, 10.0, to_mm(content_x), to_mm(y_top_pdf - 42.0), &font_regular);
                    }
                }
            }
        }
    }

    let mut pdf_bytes = Vec::new();
    {
        let mut writer = BufWriter::new(&mut pdf_bytes);
        doc.save(&mut writer)
            .map_err(|e| format!("Error guardando PDF: {:?}", e))?;
    }

    Ok(pdf_bytes)
}
